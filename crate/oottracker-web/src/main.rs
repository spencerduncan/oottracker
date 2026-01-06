#![deny(
    rust_2018_idioms,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![allow(unused_extern_crates)] // apparently rocket-derive still uses `extern crate`
#![allow(renamed_and_removed_lints)]
// private_in_public was removed in recent Rust
// Rocket's route macros generate code that uses route functions in ways the
// compiler can't track before macro expansion, causing false "unused import" warnings.
#![allow(unused_imports)]
#![forbid(unsafe_code)]

use {
    crate::{mw::MwState, restream::RestreamState},
    async_proto::{ReadError, WriteError},
    derive_more::From,
    futures::{
        future::{FutureExt as _, TryFutureExt as _},
        stream::TryStreamExt as _,
    },
    lazy_regex::regex_is_match,
    oottracker::{websocket::RoomToken, Knowledge, ModelState, Ram, TrackerCtx},
    rocket::http::Status,
    sqlx::{Row, SqlitePool},
    std::{
        collections::hash_map::{self, HashMap},
        fmt,
        sync::Arc,
        time::{Duration, Instant},
    },
    tokio::sync::{watch::*, Mutex, RwLock},
    warp::Filter as _,
};

mod http;
mod mw;
mod restream;
mod websocket;

type MwRooms = Arc<RwLock<HashMap<String, Arc<RwLock<MwState>>>>>;
type Restreams = Arc<RwLock<HashMap<String, RestreamState>>>;
type Rooms = Arc<Mutex<HashMap<String, RoomState>>>;

struct RoomState {
    name: String,
    tx: Sender<()>,
    rx: Receiver<()>,
    last_saved: Instant,
    model: ModelState,
    /// Optional token for write authorization. If set, clients must provide this token to modify the room.
    token: Option<RoomToken>,
}

impl RoomState {
    pub(crate) fn new(name: &str) -> Result<Self, Error> {
        if regex_is_match!("^[0-9a-z]+(?:-[0-9a-z]+)*$", name) {
            Ok(Self::from_model(name, ModelState::default(), None))
        } else {
            Err(Error::RoomName)
        }
    }

    fn from_model(name: &str, model: ModelState, token: Option<RoomToken>) -> Self {
        let (tx, rx) = channel(());
        Self {
            tx,
            rx,
            model,
            name: name.to_owned(),
            last_saved: Instant::now(),
            token,
        }
    }

    /// Check if the provided token authorizes write access to this room.
    /// Returns true if:
    /// - The room has no token set (open access)
    /// - The provided token matches the room's token
    pub(crate) fn check_auth(&self, provided_token: &Option<RoomToken>) -> bool {
        match &self.token {
            None => true, // No token required
            Some(room_token) => provided_token.as_ref().is_some_and(|t| t == room_token),
        }
    }

    pub(crate) async fn save(&mut self, pool: &SqlitePool) -> Result<(), Error> {
        if self.last_saved.elapsed() >= Duration::from_secs(60) {
            self.force_save(pool).await?;
        }
        Ok(())
    }

    pub(crate) async fn force_save(&mut self, pool: &SqlitePool) -> Result<(), Error> {
        let ModelState {
            ref knowledge,
            ref ram,
            ..
        } = self.model; //TODO include tracker context
                        //TODO versioning (e.g. to recover RAM from previous versions)
        let knowledge_json = serde_json::to_string(knowledge)?;
        let ram_json = serde_json::to_string(&ram.to_ranges())?;
        sqlx::query("INSERT OR REPLACE INTO rooms (name, knowledge, ram) VALUES (?, ?, ?)")
            .bind(&self.name)
            .bind(&knowledge_json)
            .bind(&ram_json)
            .execute(pool)
            .await?;
        self.last_saved = Instant::now();
        Ok(())
    }
}

async fn get_room<T>(
    rooms: &Rooms,
    name: String,
    f: impl FnOnce(&RoomState) -> T,
) -> Result<T, Error> {
    let mut rooms = rooms.lock().await;
    Ok(f(match rooms.entry(name.clone()) {
        hash_map::Entry::Occupied(entry) => entry.into_mut(),
        hash_map::Entry::Vacant(entry) => entry.insert(RoomState::new(&name)?),
    }))
}

async fn edit_room(
    pool: &SqlitePool,
    rooms: &Rooms,
    name: String,
    f: impl FnOnce(&mut RoomState) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut rooms = rooms.lock().await;
    let room = match rooms.entry(name.clone()) {
        hash_map::Entry::Occupied(entry) => entry.into_mut(),
        hash_map::Entry::Vacant(entry) => entry.insert(RoomState::new(&name)?),
    };
    f(room)?;
    room.tx
        .send(())
        .expect("failed to notify websockets about state change");
    room.save(pool).await?;
    Ok(())
}

#[derive(Debug, From)]
enum Error {
    CellId,
    Json(serde_json::Error),
    RamDecode(oottracker::ram::DecodeError),
    Read(ReadError),
    Rocket(rocket::error::Error),
    RoomName,
    Sql(sqlx::Error),
    Task(tokio::task::JoinError),
    /// Authorization failed - invalid or missing token for room operation.
    Unauthorized(String),
    Write(WriteError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CellId => write!(f, "no such cell"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::RamDecode(e) => write!(f, "error decoding RAM: {e}"),
            Self::Read(e) => write!(f, "read error: {e}"),
            Self::Rocket(e) => write!(f, "rocket error: {e}"),
            Self::RoomName => write!(f, "invalid room name"),
            Self::Sql(e) => write!(f, "database error: {e}"),
            Self::Task(e) => write!(f, "task error: {e}"),
            Self::Unauthorized(room) => write!(
                f,
                "unauthorized: invalid or missing token for room '{room}'"
            ),
            Self::Write(e) => write!(f, "write error: {e}"),
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for Error {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Self::CellId => Err(Status::NotFound),
            Self::Json(_) => Err(Status::InternalServerError),
            Self::RamDecode(_) => Err(Status::InternalServerError),
            Self::Read(_) => Err(Status::InternalServerError),
            Self::Rocket(_) => Err(Status::InternalServerError),
            Self::RoomName => Err(Status::NotFound),
            Self::Sql(_) => Err(Status::InternalServerError),
            Self::Task(_) => Err(Status::InternalServerError),
            Self::Unauthorized(_) => Err(Status::Unauthorized),
            Self::Write(_) => Err(Status::InternalServerError),
        }
    }
}

#[wheel::main(rocket)]
async fn main() -> Result<(), Error> {
    // Connect to SQLite database (creates file if it doesn't exist)
    let pool = SqlitePool::connect("sqlite:oottracker.db?mode=rwc").await?;

    // Create schema if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS rooms (
            name TEXT PRIMARY KEY,
            knowledge TEXT NOT NULL,
            ram TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    // Load existing rooms from database
    let rooms = {
        let mut rooms = HashMap::default();
        let mut query = sqlx::query("SELECT name, knowledge, ram FROM rooms").fetch(&pool);
        while let Some(row) = query.try_next().await? {
            let name: String = row.get("name");
            let knowledge_json: String = row.get("knowledge");
            let ram_json: String = row.get("ram");

            let knowledge: Knowledge = serde_json::from_str(&knowledge_json)?;
            let ram_ranges: Vec<Vec<u8>> = serde_json::from_str(&ram_json)?;
            let ram = Ram::from_range_bufs(ram_ranges)?;

            let state = RoomState::from_model(
                &name,
                ModelState {
                    knowledge,
                    ram,
                    tracker_ctx: TrackerCtx::default(),
                    check_tracker: None,
                },
                None, // Rooms loaded from database don't have tokens (backwards compatible)
            );
            rooms.insert(name, state);
        }
        Rooms::new(Mutex::new(rooms))
    };
    //TODO force-save all rooms on stop
    let restreams = {
        //TODO remove hardcoded restream, allow configuring active restreams somehow
        let mut map = HashMap::default();
        let multiworld_3v3 = vec![vec!["a1", "b1"], vec!["a2", "b2"], vec!["a3", "b3"]];
        map.insert(format!("fenhl"), RestreamState::new(multiworld_3v3));
        Restreams::new(RwLock::new(map))
    };
    let mw_rooms = MwRooms::default();
    let websocket_task = {
        let pool = pool.clone();
        let rooms = Rooms::clone(&rooms);
        let restreams = Restreams::clone(&restreams);
        let mw_rooms = MwRooms::clone(&mw_rooms);
        let handler = warp::header::optional::<String>("origin")
            .and(warp::ws())
            .and_then(move |origin, ws| {
                websocket::ws_handler(
                    pool.clone(),
                    Rooms::clone(&rooms),
                    Restreams::clone(&restreams),
                    MwRooms::clone(&mw_rooms),
                    origin,
                    ws,
                )
            });
        tokio::spawn(warp::serve(handler).run(([127, 0, 0, 1], 24808))).err_into()
    };
    let rocket_task = tokio::spawn(http::rocket(pool, rooms, restreams, mw_rooms).launch()).map(
        |res| match res {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(Error::from(e)),
            Err(e) => Err(Error::from(e)),
        },
    );
    let ((), ()) = tokio::try_join!(websocket_task, rocket_task)?;
    Ok(())
}
