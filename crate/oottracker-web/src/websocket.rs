use {
    crate::{
        edit_room, get_room,
        mw::{AutoUpdate, MwState},
        restream::render_double_cell,
        Error, MwRooms, Restreams, Rooms,
    },
    async_proto::Protocol,
    futures::stream::{SplitSink, Stream, StreamExt as _},
    iced_core::keyboard::Modifiers as KeyboardModifiers,
    oottracker::websocket::{ClientMessage, MwItem, ServerMessage},
    sqlx::PgPool,
    std::{env, sync::Arc, time::Duration},
    tokio::{sync::Mutex, time::sleep},
    tracing::{error, warn},
    warp::{
        reject::Rejection,
        reply::Reply,
        ws::{Message, WebSocket},
    },
};

/// Default allowed origins for WebSocket connections.
/// These can be overridden via the WEBSOCKET_ALLOWED_ORIGINS environment variable.
const DEFAULT_ALLOWED_ORIGINS: &[&str] = &[
    "http://localhost",
    "http://localhost:8000",
    "http://127.0.0.1",
    "http://127.0.0.1:8000",
    "https://oottracker.fenhl.net",
];

/// Validates whether the provided Origin header is allowed.
/// Returns true if:
/// - The origin is in the allowed list (from env var or default)
/// - No origin is provided (same-origin requests from some clients)
fn is_origin_allowed(origin: Option<&str>) -> bool {
    let Some(origin) = origin else {
        // Allow requests without Origin header (e.g., same-origin or non-browser clients)
        return true;
    };

    // Check environment variable for custom allowed origins
    if let Ok(allowed) = env::var("WEBSOCKET_ALLOWED_ORIGINS") {
        return allowed.split(',').any(|allowed_origin| {
            let allowed_origin = allowed_origin.trim();
            origin == allowed_origin || origin.starts_with(&format!("{allowed_origin}:"))
        });
    }

    // Check against default allowed origins
    DEFAULT_ALLOWED_ORIGINS.iter().any(|&allowed_origin| {
        origin == allowed_origin || origin.starts_with(&format!("{allowed_origin}:"))
    })
}

type WsSink = Arc<Mutex<SplitSink<WebSocket, Message>>>;

async fn client_session(
    pool: &PgPool,
    rooms: Rooms,
    restreams: Restreams,
    mw_rooms: MwRooms,
    mut stream: impl Stream<Item = Result<Message, warp::Error>> + Unpin + Send,
    sink: WsSink,
) -> Result<(), Error> {
    let ping_sink = WsSink::clone(&sink);
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;
            if let Err(e) = ServerMessage::Ping
                .write_warp(&mut *ping_sink.lock().await)
                .await
            {
                warn!("WebSocket ping failed, closing connection: {e}");
                break;
            }
        }
    });
    loop {
        match ClientMessage::read_warp(&mut stream).await? {
            ClientMessage::Pong => {}
            ClientMessage::SubscribeRestream {
                restream,
                runner,
                layout,
            } => {
                let restreams = Restreams::clone(&restreams);
                let sink = WsSink::clone(&sink);
                tokio::spawn(async move {
                    let restream_name = restream.clone();
                    let runner_name = runner.clone();
                    let (mut old_cells, mut rx) = {
                        let restreams = restreams.read().await;
                        let restream = match restreams.get(&restream) {
                            Some(restream) => restream,
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such restream")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(restream = %restream_name, "Failed to send 'no such restream' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let (rx, runner) = match restream.runner(&runner) {
                            Some((_, rx, runner)) => (rx, runner),
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such runner")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(runner = %runner_name, "Failed to send 'no such runner' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let cells = layout
                            .cells()
                            .into_iter()
                            .map(|cell| cell.id.kind().render(&runner))
                            .collect::<Vec<_>>();
                        if let Err(e) = ServerMessage::Init(cells.clone())
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            warn!(restream = %restream_name, runner = %runner_name, "Failed to send Init message to client: {e}");
                            return;
                        }
                        (cells, rx.clone())
                    };
                    loop {
                        match rx.changed().await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!(restream = %restream_name, runner = %runner_name, "Restream watch channel closed: {e}");
                                break;
                            }
                        }
                        let new_cells = {
                            let restreams = restreams.read().await;
                            let restream = match restreams.get(&restream) {
                                Some(restream) => restream,
                                None => {
                                    if let Err(e) = ServerMessage::from_error("no such restream")
                                        .write_warp(&mut *sink.lock().await)
                                        .await
                                    {
                                        error!(restream = %restream_name, "Failed to send 'no such restream' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            let runner = match restream.runner(&runner) {
                                Some((_, _, runner)) => runner,
                                None => {
                                    if let Err(e) = ServerMessage::from_error("no such runner")
                                        .write_warp(&mut *sink.lock().await)
                                        .await
                                    {
                                        error!(runner = %runner_name, "Failed to send 'no such runner' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            layout
                                .cells()
                                .into_iter()
                                .map(|cell| cell.id.kind().render(&runner))
                                .collect::<Vec<_>>()
                        };
                        for (i, (old_cell, new_cell)) in
                            old_cells.iter().zip(&new_cells).enumerate()
                        {
                            if old_cell != new_cell {
                                if let Err(e) = (ServerMessage::Update {
                                    cell_id: i.try_into().expect("too many cells"),
                                    new_cell: new_cell.clone(),
                                })
                                .write_warp(&mut *sink.lock().await)
                                .await
                                {
                                    warn!(restream = %restream_name, runner = %runner_name, "Failed to send Update message to client: {e}");
                                    return;
                                }
                            }
                        }
                        old_cells = new_cells;
                    }
                });
            }
            ClientMessage::SubscribeDoubleRestream {
                restream,
                runner1,
                runner2,
                layout,
            } => {
                let restreams = Restreams::clone(&restreams);
                let sink = WsSink::clone(&sink);
                tokio::spawn(async move {
                    let restream_name = restream.clone();
                    let runner1_name = runner1.clone();
                    let runner2_name = runner2.clone();
                    let (mut old_cells, mut rx) = {
                        let restreams = restreams.read().await;
                        let restream = match restreams.get(&restream) {
                            Some(restream) => restream,
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such restream")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(restream = %restream_name, "Failed to send 'no such restream' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let (rx, runner1) = match restream.runner(&runner1) {
                            Some((_, rx, runner)) => (rx, runner),
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such runner")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(runner = %runner1_name, "Failed to send 'no such runner' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let runner2 = match restream.runner(&runner2) {
                            Some((_, _, runner)) => runner,
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such runner")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(runner = %runner2_name, "Failed to send 'no such runner' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let cells = layout
                            .cells()
                            .into_iter()
                            .map(|reward| render_double_cell(runner1, runner2, reward))
                            .collect::<Vec<_>>();
                        if let Err(e) = ServerMessage::Init(cells.clone())
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            warn!(restream = %restream_name, "Failed to send Init message to client: {e}");
                            return;
                        }
                        (cells, rx.clone())
                    };
                    loop {
                        match rx.changed().await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!(restream = %restream_name, "Double restream watch channel closed: {e}");
                                break;
                            }
                        }
                        let new_cells = {
                            let restreams = restreams.read().await;
                            let restream = match restreams.get(&restream) {
                                Some(restream) => restream,
                                None => {
                                    if let Err(e) = ServerMessage::from_error("no such restream")
                                        .write_warp(&mut *sink.lock().await)
                                        .await
                                    {
                                        error!(restream = %restream_name, "Failed to send 'no such restream' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            let runner1 = match restream.runner(&runner1) {
                                Some((_, _, runner)) => runner,
                                None => {
                                    if let Err(e) = ServerMessage::from_error("no such runner")
                                        .write_warp(&mut *sink.lock().await)
                                        .await
                                    {
                                        error!(runner = %runner1_name, "Failed to send 'no such runner' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            let runner2 = match restream.runner(&runner2) {
                                Some((_, _, runner)) => runner,
                                None => {
                                    if let Err(e) = ServerMessage::from_error("no such runner")
                                        .write_warp(&mut *sink.lock().await)
                                        .await
                                    {
                                        error!(runner = %runner2_name, "Failed to send 'no such runner' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            layout
                                .cells()
                                .into_iter()
                                .map(|reward| render_double_cell(runner1, runner2, reward))
                                .collect::<Vec<_>>()
                        };
                        for (i, (old_cell, new_cell)) in
                            old_cells.iter().zip(&new_cells).enumerate()
                        {
                            if old_cell != new_cell {
                                if let Err(e) = (ServerMessage::Update {
                                    cell_id: i.try_into().expect("too many cells"),
                                    new_cell: new_cell.clone(),
                                })
                                .write_warp(&mut *sink.lock().await)
                                .await
                                {
                                    warn!(restream = %restream_name, "Failed to send Update message to client: {e}");
                                    return;
                                }
                            }
                        }
                        old_cells = new_cells;
                    }
                });
            }
            ClientMessage::ClickRestream {
                restream,
                runner,
                layout,
                cell_id,
                right,
            } => {
                let restream_name = restream.clone();
                let runner_name = runner.clone();
                let mut restreams = restreams.write().await;
                let restream = match restreams.get_mut(&restream) {
                    Some(restream) => restream,
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such restream")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(restream = %restream_name, "Failed to send 'no such restream' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                let (tx, runner) = match restream.runner_mut(&runner) {
                    Some((tx, _, runner)) => (tx, runner),
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such runner")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(runner = %runner_name, "Failed to send 'no such runner' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                let cell = match layout.cells().get(usize::from(cell_id)) {
                    Some(cell) => cell.id,
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such cell")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(cell_id = %cell_id, "Failed to send 'no such cell' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                if right {
                    let _ /* no med right-click menu in web app */ = cell.kind().right_click(true /*TODO verify that the client has access?*/, KeyboardModifiers::default(), runner);
                } else {
                    let _ /* no med right-click menu in web app */ = cell.kind().left_click(true /*TODO verify that the client has access?*/, KeyboardModifiers::default(), runner);
                }
                tx.send(())
                    .expect("failed to notify websockets about state change");
            }
            ClientMessage::SubscribeRaw { room } => {
                let rooms = Rooms::clone(&rooms);
                let sink = WsSink::clone(&sink);
                tokio::spawn(async move {
                    let room_name = room.clone();
                    let result: Result<(), Error> = async {
                        let (mut old_model, mut rx) = get_room(&rooms, room.clone(), |room| {
                            (room.model.clone(), room.rx.clone())
                        })
                        .await?;
                        ServerMessage::InitRaw(old_model.clone())
                            .write_warp(&mut *sink.lock().await)
                            .await?;
                        loop {
                            match rx.changed().await {
                                Ok(()) => {}
                                Err(e) => {
                                    warn!(room = %room_name, "Room watch channel closed: {e}");
                                    break;
                                }
                            }
                            let new_model =
                                get_room(&rooms, room.clone(), |room| room.model.clone()).await?;
                            if old_model != new_model {
                                (ServerMessage::UpdateRaw(&new_model - &old_model))
                                    .write_warp(&mut *sink.lock().await)
                                    .await?;
                            }
                            old_model = new_model;
                        }
                        Ok(())
                    }
                    .await;
                    if let Err(e) = result {
                        error!(room = %room_name, "SubscribeRaw task error: {e}");
                        if let Err(send_err) = ServerMessage::from_error(&e)
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(room = %room_name, "Failed to send error to client: {send_err}");
                        }
                    }
                });
            }
            ClientMessage::SubscribeRoom { room, layout } => {
                let rooms = Rooms::clone(&rooms);
                let sink = WsSink::clone(&sink);
                tokio::spawn(async move {
                    let room_name = room.clone();
                    let result: Result<(), Error> = async {
                        let (mut old_cells, mut rx) = get_room(&rooms, room.clone(), |room| {
                            (
                                layout
                                    .cells()
                                    .into_iter()
                                    .map(|cell| cell.id.kind().render(&room.model))
                                    .collect::<Vec<_>>(),
                                room.rx.clone(),
                            )
                        })
                        .await?;
                        ServerMessage::Init(old_cells.clone())
                            .write_warp(&mut *sink.lock().await)
                            .await?;
                        loop {
                            match rx.changed().await {
                                Ok(()) => {}
                                Err(e) => {
                                    warn!(room = %room_name, "Room watch channel closed: {e}");
                                    break;
                                }
                            }
                            let new_cells = get_room(&rooms, room.clone(), |room| {
                                layout
                                    .cells()
                                    .into_iter()
                                    .map(|cell| cell.id.kind().render(&room.model))
                                    .collect::<Vec<_>>()
                            })
                            .await?;
                            for (i, (old_cell, new_cell)) in
                                old_cells.iter().zip(&new_cells).enumerate()
                            {
                                if old_cell != new_cell {
                                    (ServerMessage::Update {
                                        cell_id: i.try_into().expect("too many cells"),
                                        new_cell: new_cell.clone(),
                                    })
                                    .write_warp(&mut *sink.lock().await)
                                    .await?;
                                }
                            }
                            old_cells = new_cells;
                        }
                        Ok(())
                    }
                    .await;
                    if let Err(e) = result {
                        error!(room = %room_name, "SubscribeRoom task error: {e}");
                        if let Err(send_err) = ServerMessage::from_error(&e)
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(room = %room_name, "Failed to send error to client: {send_err}");
                        }
                    }
                });
            }
            ClientMessage::SetRaw { room, state, token } => {
                // Check authorization before allowing state modification
                {
                    let rooms_guard = rooms.lock().await;
                    if let Some(room_state) = rooms_guard.get(&room) {
                        if !room_state.check_auth(&token) {
                            if let Err(e) = (ServerMessage::Unauthorized { room: room.clone() })
                                .write_warp(&mut *sink.lock().await)
                                .await
                            {
                                warn!(room = %room, "Failed to send Unauthorized message to client: {e}");
                            }
                            continue;
                        }
                    }
                    // If room doesn't exist, it will be created (no auth needed for new rooms)
                }
                edit_room(pool, &rooms, room, |room| {
                    room.model = state;
                    Ok(())
                })
                .await?
            }
            ClientMessage::ClickRoom {
                room,
                layout,
                cell_id,
                right,
                token,
            } => {
                // Check authorization before allowing state modification
                {
                    let rooms_guard = rooms.lock().await;
                    if let Some(room_state) = rooms_guard.get(&room) {
                        if !room_state.check_auth(&token) {
                            if let Err(e) = (ServerMessage::Unauthorized { room: room.clone() })
                                .write_warp(&mut *sink.lock().await)
                                .await
                            {
                                warn!(room = %room, "Failed to send Unauthorized message to client: {e}");
                            }
                            continue;
                        }
                    }
                }
                let cell = match layout.cells().get(usize::from(cell_id)) {
                    Some(cell) => cell.id,
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such cell")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(room = %room, cell_id = %cell_id, "Failed to send 'no such cell' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                edit_room(pool, &rooms, room, |room| {
                    if right {
                        let _ /* no med right-click menu in web app */ = cell.kind().right_click(true /*TODO verify that the client has access?*/, KeyboardModifiers::default(), &mut room.model);
                    } else {
                        let _ /* no med right-click menu in web app */ = cell.kind().left_click(true /*TODO verify that the client has access?*/, KeyboardModifiers::default(), &mut room.model);
                    }
                    Ok(())
                }).await?;
            }
            ClientMessage::MwCreateRoom {
                room,
                worlds,
                token,
            } => {
                mw_rooms
                    .write()
                    .await
                    .insert(room, MwState::new(worlds, token));
            }
            ClientMessage::MwDeleteRoom { room, token } => {
                // Check authorization before allowing room deletion
                {
                    let mw_rooms_guard = mw_rooms.read().await;
                    if let Some(mw_room) = mw_rooms_guard.get(&room) {
                        let mw_room_guard = mw_room.read().await;
                        if !mw_room_guard.check_auth(&token) {
                            if let Err(e) = (ServerMessage::Unauthorized { room: room.clone() })
                                .write_warp(&mut *sink.lock().await)
                                .await
                            {
                                warn!(room = %room, "Failed to send Unauthorized message to client: {e}");
                            }
                            continue;
                        }
                    } else {
                        // Room doesn't exist, nothing to delete
                        continue;
                    }
                }
                mw_rooms.write().await.remove(&room);
            }
            ClientMessage::MwResetPlayer {
                room,
                world,
                save,
                token,
            } => {
                let mw_rooms_guard = mw_rooms.read().await;
                if let Some(mw_room) = mw_rooms_guard.get(&room) {
                    // Check authorization before allowing player reset
                    let mw_room_guard = mw_room.read().await;
                    if !mw_room_guard.check_auth(&token) {
                        if let Err(e) = (ServerMessage::Unauthorized { room: room.clone() })
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            warn!(room = %room, "Failed to send Unauthorized message to client: {e}");
                        }
                        continue;
                    }
                    if let Err(e) = mw_room_guard
                        .incoming_queue
                        .send(AutoUpdate::Reset { world, save })
                    {
                        warn!(room = %room, world = %world, "Failed to send reset to multiworld queue: {e}");
                    }
                } else {
                    if let Err(e) = ServerMessage::from_error("no such multiworld room")
                        .write_warp(&mut *sink.lock().await)
                        .await
                    {
                        error!(room = %room, "Failed to send 'no such multiworld room' error to client: {e}");
                    }
                }
            }
            #[allow(deprecated)]
            ClientMessage::MwGetItem { .. } => {
                if let Err(e) = ServerMessage::from_error(
                    "MwGetItem command is no longer supported, use MwQueueItem instead",
                )
                .write_warp(&mut *sink.lock().await)
                .await
                {
                    warn!("Failed to send deprecated MwGetItem error to client: {e}");
                }
            }
            ClientMessage::ClickMw {
                room,
                world,
                layout,
                cell_id,
                right,
                token,
            } => {
                let mw_rooms_guard = mw_rooms.read().await;
                let mw_room = match mw_rooms_guard.get(&room) {
                    Some(mw_room) => mw_room,
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such multiworld room")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(room = %room, "Failed to send 'no such multiworld room' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                let mut mw_room_guard = mw_room.write().await;
                // Check authorization before allowing state modification
                if !mw_room_guard.check_auth(&token) {
                    if let Err(e) = (ServerMessage::Unauthorized { room: room.clone() })
                        .write_warp(&mut *sink.lock().await)
                        .await
                    {
                        warn!(room = %room, "Failed to send Unauthorized message to client: {e}");
                    }
                    continue;
                }
                let (tx, model) = match mw_room_guard.world_mut(world) {
                    Some((tx, _, model, _, _)) => (tx, model),
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such world")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(room = %room, world = %world, "Failed to send 'no such world' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                let cell = match layout.cells().get(usize::from(cell_id)) {
                    Some(cell) => cell.id,
                    None => {
                        if let Err(e) = ServerMessage::from_error("no such cell")
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            error!(room = %room, cell_id = %cell_id, "Failed to send 'no such cell' error to client: {e}");
                        }
                        return Ok(());
                    }
                };
                if right {
                    let _ /* no med right-click menu in web app */ = cell.kind().right_click(true /*TODO verify that the client has access?*/, KeyboardModifiers::default(), model);
                } else {
                    let _ /* no med right-click menu in web app */ = cell.kind().left_click(true /*TODO verify that the client has access?*/, KeyboardModifiers::default(), model);
                }
                tx.send(())
                    .expect("failed to notify websockets about state change");
            }
            ClientMessage::SubscribeMw {
                room,
                world,
                layout,
            } => {
                let mw_rooms = MwRooms::clone(&mw_rooms);
                let sink = WsSink::clone(&sink);
                tokio::spawn(async move {
                    let room_name = room.clone();
                    let (mut old_cells, mut rx) = {
                        let mw_rooms = mw_rooms.read().await;
                        let mw_room = match mw_rooms.get(&room) {
                            Some(mw_room) => mw_room,
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such multiworld room")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(room = %room_name, "Failed to send 'no such multiworld room' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let mw_room = mw_room.read().await;
                        let (rx, model) = match mw_room.world(world) {
                            Some((_, rx, model, _, _)) => (rx, model),
                            None => {
                                if let Err(e) = ServerMessage::from_error("no such world")
                                    .write_warp(&mut *sink.lock().await)
                                    .await
                                {
                                    error!(room = %room_name, world = %world, "Failed to send 'no such world' error to client: {e}");
                                }
                                return;
                            }
                        };
                        let cells = layout
                            .cells()
                            .into_iter()
                            .map(|cell| cell.id.kind().render(&model))
                            .collect::<Vec<_>>();
                        if let Err(e) = ServerMessage::Init(cells.clone())
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            warn!(room = %room_name, world = %world, "Failed to send Init message to client: {e}");
                            return;
                        }
                        (cells, rx.clone())
                    };
                    loop {
                        match rx.changed().await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!(room = %room_name, world = %world, "Multiworld watch channel closed: {e}");
                                break;
                            }
                        }
                        let new_cells = {
                            let mw_rooms = mw_rooms.read().await;
                            let mw_room = match mw_rooms.get(&room) {
                                Some(mw_room) => mw_room,
                                None => {
                                    if let Err(e) =
                                        ServerMessage::from_error("no such multiworld room")
                                            .write_warp(&mut *sink.lock().await)
                                            .await
                                    {
                                        error!(room = %room_name, "Failed to send 'no such multiworld room' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            let mw_room = mw_room.read().await;
                            let model = match mw_room.world(world) {
                                Some((_, _, model, _, _)) => model,
                                None => {
                                    if let Err(e) = ServerMessage::from_error("no such world")
                                        .write_warp(&mut *sink.lock().await)
                                        .await
                                    {
                                        error!(room = %room_name, world = %world, "Failed to send 'no such world' error to client: {e}");
                                    }
                                    return;
                                }
                            };
                            layout
                                .cells()
                                .into_iter()
                                .map(|cell| cell.id.kind().render(&model))
                                .collect::<Vec<_>>()
                        };
                        for (i, (old_cell, new_cell)) in
                            old_cells.iter().zip(&new_cells).enumerate()
                        {
                            if old_cell != new_cell {
                                if let Err(e) = (ServerMessage::Update {
                                    cell_id: i.try_into().expect("too many cells"),
                                    new_cell: new_cell.clone(),
                                })
                                .write_warp(&mut *sink.lock().await)
                                .await
                                {
                                    warn!(room = %room_name, world = %world, "Failed to send Update message to client: {e}");
                                    return;
                                }
                            }
                        }
                        old_cells = new_cells;
                    }
                });
            }
            #[allow(deprecated)]
            ClientMessage::MwGetItemAll { .. } => {
                if let Err(e) = ServerMessage::from_error(
                    "MwGetItemAll command is no longer supported, use MwQueueItem instead",
                )
                .write_warp(&mut *sink.lock().await)
                .await
                {
                    warn!("Failed to send deprecated MwGetItemAll error to client: {e}");
                }
            }
            ClientMessage::MwQueueItem {
                room,
                source_world,
                key,
                kind,
                target_world,
                token,
            } => {
                let mw_rooms_guard = mw_rooms.read().await;
                if let Some(mw_room) = mw_rooms_guard.get(&room) {
                    let mw_room_guard = mw_room.read().await;
                    // Check authorization before allowing item queue
                    if !mw_room_guard.check_auth(&token) {
                        if let Err(e) = (ServerMessage::Unauthorized { room: room.clone() })
                            .write_warp(&mut *sink.lock().await)
                            .await
                        {
                            warn!(room = %room, "Failed to send Unauthorized message to client: {e}");
                        }
                        continue;
                    }
                    if let Err(e) = mw_room_guard.incoming_queue.send(AutoUpdate::Queue {
                        item: MwItem {
                            source: source_world,
                            key,
                            kind,
                        },
                        target_world,
                    }) {
                        warn!(room = %room, target_world = %target_world, "Failed to send item to multiworld queue: {e}");
                    }
                } else {
                    if let Err(e) = ServerMessage::from_error("no such multiworld room")
                        .write_warp(&mut *sink.lock().await)
                        .await
                    {
                        error!(room = %room, "Failed to send 'no such multiworld room' error to client: {e}");
                    }
                }
            }
        }
    }
}

async fn client_connection(
    pool: PgPool,
    rooms: Rooms,
    restreams: Restreams,
    mw_rooms: MwRooms,
    ws: WebSocket,
) {
    let (ws_sink, ws_stream) = ws.split();
    let ws_sink = WsSink::new(Mutex::new(ws_sink));
    if let Err(e) = client_session(
        &pool,
        rooms,
        restreams,
        mw_rooms,
        ws_stream,
        WsSink::clone(&ws_sink),
    )
    .await
    {
        error!("WebSocket client session error: {e}");
        if let Err(send_err) = ServerMessage::from_error(e)
            .write_warp(&mut *ws_sink.lock().await)
            .await
        {
            warn!("Failed to send session error to client: {send_err}");
        }
    }
}

/// Custom rejection for forbidden origin
#[derive(Debug)]
pub(crate) struct ForbiddenOrigin;

impl warp::reject::Reject for ForbiddenOrigin {}

pub(crate) async fn ws_handler(
    pool: PgPool,
    rooms: Rooms,
    restreams: Restreams,
    mw_rooms: MwRooms,
    origin: Option<String>,
    ws: warp::ws::Ws,
) -> Result<impl Reply, Rejection> {
    // Validate the Origin header
    if !is_origin_allowed(origin.as_deref()) {
        return Err(warp::reject::custom(ForbiddenOrigin));
    }

    Ok(ws.on_upgrade(move |ws| client_connection(pool, rooms, restreams, mw_rooms, ws)))
}
