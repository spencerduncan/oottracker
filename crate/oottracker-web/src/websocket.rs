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
    sqlx::SqlitePool,
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
    pool: &SqlitePool,
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
                                Ok(()) => {
                                    tracing::debug!(room = %room_name, "SubscribeRoom: received change notification");
                                }
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
                            let mut updates_sent = 0;
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
                                    updates_sent += 1;
                                }
                            }
                            if updates_sent > 0 {
                                tracing::debug!(room = %room_name, updates_sent, "SubscribeRoom: sent cell updates");
                            } else {
                                tracing::debug!(room = %room_name, "SubscribeRoom: no cell changes detected");
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
            ClientMessage::UpdateSettings {
                room,
                max_bottles,
                token,
            } => {
                // Check authorization before allowing settings modification
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
                // Clamp max_bottles to valid range (1-4)
                let max_bottles = max_bottles.clamp(1, 4);
                edit_room(pool, &rooms, room, |room| {
                    room.model.max_bottles = max_bottles;
                    Ok(())
                })
                .await?
            }
        }
    }
}

async fn client_connection(
    pool: SqlitePool,
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
    pool: SqlitePool,
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

#[cfg(test)]
mod tests {
    use super::*;
    use oottracker::websocket::ServerMessage;
    use std::env;

    // ============================================================================
    // Origin Validation Tests
    // ============================================================================

    mod origin_validation {
        use super::*;

        #[test]
        fn test_no_origin_is_allowed() {
            // Requests without an Origin header should be allowed (same-origin or non-browser)
            assert!(is_origin_allowed(None));
        }

        #[test]
        fn test_default_allowed_origins() {
            // Test all default allowed origins
            let allowed = [
                "http://localhost",
                "http://localhost:8000",
                "http://127.0.0.1",
                "http://127.0.0.1:8000",
                "https://oottracker.fenhl.net",
            ];

            for origin in allowed {
                assert!(
                    is_origin_allowed(Some(origin)),
                    "Origin '{}' should be allowed",
                    origin
                );
            }
        }

        #[test]
        fn test_localhost_with_different_ports() {
            // localhost with various ports should be allowed (prefix matching)
            assert!(is_origin_allowed(Some("http://localhost:3000")));
            assert!(is_origin_allowed(Some("http://localhost:5000")));
            assert!(is_origin_allowed(Some("http://localhost:9090")));
            assert!(is_origin_allowed(Some("http://127.0.0.1:3000")));
            assert!(is_origin_allowed(Some("http://127.0.0.1:5000")));
        }

        #[test]
        fn test_forbidden_origins() {
            // Random external origins should be rejected
            let forbidden = [
                "http://evil.com",
                "https://malicious-site.org",
                "http://localhost.evil.com",
                "http://not-localhost",
                "http://192.168.1.1",
                "https://example.com",
            ];

            for origin in forbidden {
                assert!(
                    !is_origin_allowed(Some(origin)),
                    "Origin '{}' should be forbidden",
                    origin
                );
            }
        }

        #[test]
        fn test_env_var_override() {
            // Save original env var if present
            let original = env::var("WEBSOCKET_ALLOWED_ORIGINS").ok();

            // Set custom allowed origins
            env::set_var(
                "WEBSOCKET_ALLOWED_ORIGINS",
                "https://custom.example.com,http://test.local",
            );

            // Custom origins should now be allowed
            assert!(is_origin_allowed(Some("https://custom.example.com")));
            assert!(is_origin_allowed(Some("http://test.local")));
            assert!(is_origin_allowed(Some("http://test.local:8080"))); // with port

            // Default origins should now be forbidden (env var overrides defaults)
            assert!(!is_origin_allowed(Some("http://localhost")));
            assert!(!is_origin_allowed(Some("https://oottracker.fenhl.net")));

            // Restore original env var
            match original {
                Some(val) => env::set_var("WEBSOCKET_ALLOWED_ORIGINS", val),
                None => env::remove_var("WEBSOCKET_ALLOWED_ORIGINS"),
            }
        }

        #[test]
        fn test_env_var_with_whitespace() {
            // Save original env var if present
            let original = env::var("WEBSOCKET_ALLOWED_ORIGINS").ok();

            // Set env var with whitespace around entries
            env::set_var(
                "WEBSOCKET_ALLOWED_ORIGINS",
                "  https://spaced.example.com  ,  http://another.test  ",
            );

            // Should still match after trimming
            assert!(is_origin_allowed(Some("https://spaced.example.com")));
            assert!(is_origin_allowed(Some("http://another.test")));

            // Restore original env var
            match original {
                Some(val) => env::set_var("WEBSOCKET_ALLOWED_ORIGINS", val),
                None => env::remove_var("WEBSOCKET_ALLOWED_ORIGINS"),
            }
        }

        #[test]
        fn test_origin_case_sensitivity() {
            // Origins are case-sensitive for the scheme and host
            // Note: In practice, browsers normalize these, but our validation is exact
            assert!(!is_origin_allowed(Some("HTTP://localhost")));
            assert!(!is_origin_allowed(Some("http://LOCALHOST")));
            assert!(!is_origin_allowed(Some("HTTPS://oottracker.fenhl.net")));
        }
    }

    // ============================================================================
    // Server Message Tests
    // ============================================================================

    mod server_message {
        use super::*;

        #[test]
        fn test_from_error_with_string() {
            let msg = ServerMessage::from_error("test error message");

            match msg {
                ServerMessage::Error { debug, display } => {
                    assert!(debug.contains("test error message"));
                    assert_eq!(display, "test error message");
                }
                _ => panic!("Expected Error variant"),
            }
        }

        #[test]
        fn test_from_error_with_custom_error() {
            #[derive(Debug)]
            struct TestError {
                code: u32,
                message: String,
            }

            impl std::fmt::Display for TestError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "Error {}: {}", self.code, self.message)
                }
            }

            let err = TestError {
                code: 404,
                message: "not found".to_string(),
            };
            let msg = ServerMessage::from_error(&err);

            match msg {
                ServerMessage::Error { debug, display } => {
                    // Debug format includes struct details
                    assert!(debug.contains("TestError"));
                    assert!(debug.contains("404"));
                    // Display format is user-friendly
                    assert_eq!(display, "Error 404: not found");
                }
                _ => panic!("Expected Error variant"),
            }
        }

        #[test]
        fn test_from_error_with_io_error() {
            let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
            let msg = ServerMessage::from_error(&io_err);

            match msg {
                ServerMessage::Error { debug, display } => {
                    assert!(!debug.is_empty());
                    assert!(display.contains("file not found"));
                }
                _ => panic!("Expected Error variant"),
            }
        }
    }

    // ============================================================================
    // ForbiddenOrigin Rejection Tests
    // ============================================================================

    mod forbidden_origin {
        use super::*;

        #[test]
        fn test_forbidden_origin_is_debug() {
            let rejection = ForbiddenOrigin;
            let debug_str = format!("{:?}", rejection);
            assert_eq!(debug_str, "ForbiddenOrigin");
        }

        #[test]
        fn test_forbidden_origin_is_reject() {
            // Verify ForbiddenOrigin implements warp::reject::Reject
            fn assert_reject<T: warp::reject::Reject>() {}
            assert_reject::<ForbiddenOrigin>();
        }
    }

    // ============================================================================
    // Message Serialization Tests
    // ============================================================================

    mod message_serialization {
        use async_proto::Protocol;
        use oottracker::ui::{
            AccessibilityStatus, CellOverlay, CellRender, CellStyle, ImageInfo, TrackerLayout,
        };
        use oottracker::websocket::{ClientMessage, MwItem, RoomToken, ServerMessage};
        use std::num::NonZeroU8;

        #[test]
        fn test_room_token_creation() {
            let token = RoomToken::new("secret-token-123");
            assert_eq!(token.as_str(), "secret-token-123");
        }

        #[test]
        fn test_room_token_equality() {
            let token1 = RoomToken::new("token-a");
            let token2 = RoomToken::new("token-a");
            let token3 = RoomToken::new("token-b");

            assert_eq!(token1, token2);
            assert_ne!(token1, token3);
        }

        #[test]
        fn test_mw_item_creation() {
            let item = MwItem {
                source: NonZeroU8::new(1).unwrap(),
                key: 12345,
                kind: 42,
            };

            assert_eq!(item.source.get(), 1);
            assert_eq!(item.key, 12345);
            assert_eq!(item.kind, 42);
        }

        #[test]
        fn test_mw_item_ordering() {
            let item1 = MwItem {
                source: NonZeroU8::new(1).unwrap(),
                key: 100,
                kind: 1,
            };
            let item2 = MwItem {
                source: NonZeroU8::new(2).unwrap(),
                key: 100,
                kind: 1,
            };
            let item3 = MwItem {
                source: NonZeroU8::new(1).unwrap(),
                key: 200,
                kind: 1,
            };

            // MwItem implements Ord - verify ordering works
            assert!(item1 < item2); // source 1 < source 2
            assert!(item1 < item3); // same source, key 100 < 200
        }

        #[tokio::test]
        async fn test_server_message_ping_roundtrip() {
            // Test that Ping message serializes and deserializes correctly
            let original = ServerMessage::Ping;

            // Serialize to bytes
            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize Ping");

            // Deserialize back
            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ServerMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize Ping");

            match deserialized {
                ServerMessage::Ping => {} // Success
                _ => panic!("Expected Ping"),
            }
        }

        #[tokio::test]
        async fn test_server_message_error_roundtrip() {
            let original = ServerMessage::Error {
                debug: "Debug info here".to_string(),
                display: "User-friendly message".to_string(),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize Error");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ServerMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize Error");

            match deserialized {
                ServerMessage::Error { debug, display } => {
                    assert_eq!(debug, "Debug info here");
                    assert_eq!(display, "User-friendly message");
                }
                _ => panic!("Expected Error"),
            }
        }

        #[tokio::test]
        async fn test_server_message_unauthorized_roundtrip() {
            let original = ServerMessage::Unauthorized {
                room: "test-room-123".to_string(),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize Unauthorized");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ServerMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize Unauthorized");

            match deserialized {
                ServerMessage::Unauthorized { room } => {
                    assert_eq!(room, "test-room-123");
                }
                _ => panic!("Expected Unauthorized"),
            }
        }

        #[tokio::test]
        async fn test_server_message_init_roundtrip() {
            let cells = vec![
                CellRender::new(
                    ImageInfo::new("test-image"),
                    CellStyle::Normal,
                    CellOverlay::None,
                ),
                CellRender::new(
                    ImageInfo::new("another-image"),
                    CellStyle::Dimmed,
                    CellOverlay::Count {
                        count: 5,
                        count_img: ImageInfo::new("count-image"),
                    },
                ),
            ];
            let original = ServerMessage::Init(cells.clone());

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize Init");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ServerMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize Init");

            match deserialized {
                ServerMessage::Init(deserialized_cells) => {
                    assert_eq!(deserialized_cells.len(), 2);
                    assert_eq!(deserialized_cells[0], cells[0]);
                    assert_eq!(deserialized_cells[1], cells[1]);
                }
                _ => panic!("Expected Init"),
            }
        }

        #[tokio::test]
        async fn test_server_message_update_roundtrip() {
            let cell = CellRender::new(
                ImageInfo::new("updated-cell"),
                CellStyle::Normal,
                CellOverlay::None,
            )
            .with_accessibility(AccessibilityStatus::Accessible);

            let original = ServerMessage::Update {
                cell_id: 42,
                new_cell: cell.clone(),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize Update");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ServerMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize Update");

            match deserialized {
                ServerMessage::Update { cell_id, new_cell } => {
                    assert_eq!(cell_id, 42);
                    assert_eq!(new_cell, cell);
                    assert_eq!(
                        new_cell.accessibility,
                        Some(AccessibilityStatus::Accessible)
                    );
                }
                _ => panic!("Expected Update"),
            }
        }

        #[tokio::test]
        async fn test_client_message_pong_roundtrip() {
            let original = ClientMessage::Pong;

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize Pong");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ClientMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize Pong");

            match deserialized {
                ClientMessage::Pong => {} // Success
                _ => panic!("Expected Pong"),
            }
        }

        #[tokio::test]
        async fn test_client_message_subscribe_raw_roundtrip() {
            let original = ClientMessage::SubscribeRaw {
                room: "my-test-room".to_string(),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize SubscribeRaw");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ClientMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize SubscribeRaw");

            match deserialized {
                ClientMessage::SubscribeRaw { room } => {
                    assert_eq!(room, "my-test-room");
                }
                _ => panic!("Expected SubscribeRaw"),
            }
        }

        #[tokio::test]
        async fn test_client_message_subscribe_room_roundtrip() {
            let original = ClientMessage::SubscribeRoom {
                room: "tracker-room".to_string(),
                layout: TrackerLayout::default_auto(),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize SubscribeRoom");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ClientMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize SubscribeRoom");

            match deserialized {
                ClientMessage::SubscribeRoom { room, layout: _ } => {
                    assert_eq!(room, "tracker-room");
                    // Layout comparison is complex, just verify it deserializes
                }
                _ => panic!("Expected SubscribeRoom"),
            }
        }

        #[tokio::test]
        async fn test_client_message_click_room_with_token_roundtrip() {
            let original = ClientMessage::ClickRoom {
                room: "click-test-room".to_string(),
                layout: TrackerLayout::default_auto(),
                cell_id: 5,
                right: true,
                token: Some(RoomToken::new("secret-click-token")),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize ClickRoom");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ClientMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize ClickRoom");

            match deserialized {
                ClientMessage::ClickRoom {
                    room,
                    layout: _,
                    cell_id,
                    right,
                    token,
                } => {
                    assert_eq!(room, "click-test-room");
                    assert_eq!(cell_id, 5);
                    assert!(right);
                    assert_eq!(token, Some(RoomToken::new("secret-click-token")));
                }
                _ => panic!("Expected ClickRoom"),
            }
        }

        #[tokio::test]
        async fn test_client_message_mw_create_room_roundtrip() {
            let worlds = vec![
                (None, vec![]),
                (
                    None,
                    vec![MwItem {
                        source: NonZeroU8::new(1).unwrap(),
                        key: 1,
                        kind: 100,
                    }],
                ),
            ];

            let original = ClientMessage::MwCreateRoom {
                room: "mw-room".to_string(),
                worlds: worlds.clone(),
                token: Some(RoomToken::new("mw-token")),
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize MwCreateRoom");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ClientMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize MwCreateRoom");

            match deserialized {
                ClientMessage::MwCreateRoom {
                    room,
                    worlds: deser_worlds,
                    token,
                } => {
                    assert_eq!(room, "mw-room");
                    assert_eq!(deser_worlds.len(), worlds.len());
                    assert_eq!(token, Some(RoomToken::new("mw-token")));
                }
                _ => panic!("Expected MwCreateRoom"),
            }
        }

        #[tokio::test]
        async fn test_client_message_mw_queue_item_roundtrip() {
            let original = ClientMessage::MwQueueItem {
                room: "queue-room".to_string(),
                source_world: NonZeroU8::new(1).unwrap(),
                key: 42,
                kind: 123,
                target_world: NonZeroU8::new(2).unwrap(),
                token: None,
            };

            let mut buffer = Vec::new();
            original
                .write(&mut buffer)
                .await
                .expect("Failed to serialize MwQueueItem");

            let mut cursor = std::io::Cursor::new(buffer);
            let deserialized = ClientMessage::read(&mut cursor)
                .await
                .expect("Failed to deserialize MwQueueItem");

            match deserialized {
                ClientMessage::MwQueueItem {
                    room,
                    source_world,
                    key,
                    kind,
                    target_world,
                    token,
                } => {
                    assert_eq!(room, "queue-room");
                    assert_eq!(source_world.get(), 1);
                    assert_eq!(key, 42);
                    assert_eq!(kind, 123);
                    assert_eq!(target_world.get(), 2);
                    assert!(token.is_none());
                }
                _ => panic!("Expected MwQueueItem"),
            }
        }
    }

    // ============================================================================
    // Subscription Logic Tests
    // ============================================================================

    mod subscription_logic {
        use oottracker::ui::{CellOverlay, CellRender, CellStyle, ImageInfo};

        #[test]
        fn test_cell_render_equality_for_update_detection() {
            // The subscription logic sends updates only when cells differ
            // This test verifies the equality comparison works correctly

            let cell1 = CellRender::new(
                ImageInfo::new("sword"),
                CellStyle::Normal,
                CellOverlay::None,
            );

            let cell2 = CellRender::new(
                ImageInfo::new("sword"),
                CellStyle::Normal,
                CellOverlay::None,
            );

            let cell3 = CellRender::new(
                ImageInfo::new("sword"),
                CellStyle::Dimmed, // Different style
                CellOverlay::None,
            );

            let cell4 = CellRender::new(
                ImageInfo::new("shield"), // Different image
                CellStyle::Normal,
                CellOverlay::None,
            );

            // Same cells should be equal (no update needed)
            assert_eq!(cell1, cell2);

            // Different style should not be equal (update needed)
            assert_ne!(cell1, cell3);

            // Different image should not be equal (update needed)
            assert_ne!(cell1, cell4);
        }

        #[test]
        fn test_cell_render_with_count_overlay() {
            let cell1 = CellRender::new(
                ImageInfo::new("skulltula"),
                CellStyle::Normal,
                CellOverlay::Count {
                    count: 50,
                    count_img: ImageInfo::new("count"),
                },
            );

            let cell2 = CellRender::new(
                ImageInfo::new("skulltula"),
                CellStyle::Normal,
                CellOverlay::Count {
                    count: 50,
                    count_img: ImageInfo::new("count"),
                },
            );

            let cell3 = CellRender::new(
                ImageInfo::new("skulltula"),
                CellStyle::Normal,
                CellOverlay::Count {
                    count: 51,
                    count_img: ImageInfo::new("count"),
                }, // Different count
            );

            assert_eq!(cell1, cell2);
            assert_ne!(cell1, cell3);
        }

        #[test]
        fn test_update_detection_simulation() {
            // Simulate the update detection logic used in client_session
            let old_cells = vec![
                CellRender::new(ImageInfo::new("a"), CellStyle::Normal, CellOverlay::None),
                CellRender::new(ImageInfo::new("b"), CellStyle::Normal, CellOverlay::None),
                CellRender::new(ImageInfo::new("c"), CellStyle::Normal, CellOverlay::None),
            ];

            let new_cells = vec![
                CellRender::new(ImageInfo::new("a"), CellStyle::Normal, CellOverlay::None), // Same
                CellRender::new(ImageInfo::new("b"), CellStyle::Dimmed, CellOverlay::None), // Changed
                CellRender::new(ImageInfo::new("c"), CellStyle::Normal, CellOverlay::None), // Same
            ];

            // Find indices that need updates
            let updates_needed: Vec<usize> = old_cells
                .iter()
                .zip(&new_cells)
                .enumerate()
                .filter_map(|(i, (old, new))| if old != new { Some(i) } else { None })
                .collect();

            assert_eq!(
                updates_needed,
                vec![1],
                "Only cell at index 1 should need update"
            );
        }

        #[test]
        fn test_all_cells_changed() {
            let old_cells = vec![
                CellRender::new(ImageInfo::new("a"), CellStyle::Dimmed, CellOverlay::None),
                CellRender::new(ImageInfo::new("b"), CellStyle::Dimmed, CellOverlay::None),
            ];

            let new_cells = vec![
                CellRender::new(ImageInfo::new("a"), CellStyle::Normal, CellOverlay::None),
                CellRender::new(ImageInfo::new("b"), CellStyle::Normal, CellOverlay::None),
            ];

            let updates_needed: Vec<usize> = old_cells
                .iter()
                .zip(&new_cells)
                .enumerate()
                .filter_map(|(i, (old, new))| if old != new { Some(i) } else { None })
                .collect();

            assert_eq!(updates_needed, vec![0, 1], "Both cells should need updates");
        }

        #[test]
        fn test_no_cells_changed() {
            let old_cells = vec![
                CellRender::new(ImageInfo::new("x"), CellStyle::Normal, CellOverlay::None),
                CellRender::new(ImageInfo::new("y"), CellStyle::Normal, CellOverlay::None),
            ];

            let new_cells = old_cells.clone();

            let updates_needed: Vec<usize> = old_cells
                .iter()
                .zip(&new_cells)
                .enumerate()
                .filter_map(|(i, (old, new))| if old != new { Some(i) } else { None })
                .collect();

            assert!(updates_needed.is_empty(), "No updates should be needed");
        }
    }

    // ============================================================================
    // Broadcast Logic Tests (Cell ID Conversion)
    // ============================================================================

    mod broadcast_logic {
        #[test]
        fn test_cell_id_u8_conversion() {
            // The websocket protocol uses u8 for cell IDs
            // Verify the conversion works for valid ranges

            // Valid cell IDs (0-255)
            for i in 0u8..=255 {
                let cell_id: u8 = i;
                let index: usize = cell_id.into();
                assert_eq!(index, i as usize);
            }
        }

        #[test]
        fn test_cell_id_from_index() {
            // Test converting index to cell_id (used in ServerMessage::Update)
            let indices: Vec<usize> = vec![0, 1, 42, 100, 255];

            for idx in indices {
                let cell_id: u8 = idx.try_into().expect("index should fit in u8");
                assert_eq!(cell_id as usize, idx);
            }
        }

        #[test]
        #[should_panic(expected = "too many cells")]
        fn test_cell_id_overflow_panics() {
            // The code uses expect("too many cells") for conversion
            // Indices >= 256 should panic
            let _: u8 = 256usize.try_into().expect("too many cells");
        }
    }
}
