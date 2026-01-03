#![allow(deprecated)] // avoid deprecation errors in the Protocol derivation for ClientMessage

use {
    crate::{
        ui::{CellRender, DoubleTrackerLayout, TrackerLayout},
        ModelDelta, ModelState, Save,
    },
    async_proto::Protocol,
    std::{fmt, num::NonZeroU8},
};

/// A token used to authorize write operations on a room.
/// If a room has a token set, clients must provide the matching token to perform write operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Protocol)]
pub struct RoomToken(pub String);

impl RoomToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Protocol)]
pub struct MwItem {
    pub source: NonZeroU8,
    pub key: u32,
    pub kind: u16,
}

#[derive(Protocol)]
pub enum ClientMessage {
    Pong,
    SubscribeRestream {
        restream: String,
        runner: String,
        layout: TrackerLayout,
    },
    SubscribeDoubleRestream {
        restream: String,
        runner1: String,
        runner2: String,
        layout: DoubleTrackerLayout,
    },
    ClickRestream {
        restream: String,
        runner: String,
        layout: TrackerLayout,
        cell_id: u8,
        right: bool,
    },
    SubscribeRoom {
        room: String,
        layout: TrackerLayout,
    },
    ClickRoom {
        room: String,
        layout: TrackerLayout,
        cell_id: u8,
        right: bool,
        /// Token required if the room has authorization enabled.
        token: Option<RoomToken>,
    },
    SubscribeRaw {
        room: String,
    },
    SetRaw {
        room: String,
        state: ModelState,
        /// Token required if the room has authorization enabled.
        token: Option<RoomToken>,
    },
    MwCreateRoom {
        room: String,
        worlds: Vec<(Option<Save>, Vec<MwItem>)>,
        /// Optional token to protect the room. If set, write operations require this token.
        token: Option<RoomToken>,
    },
    MwDeleteRoom {
        room: String,
        /// Token required if the room has authorization enabled.
        token: Option<RoomToken>,
    },
    MwResetPlayer {
        room: String,
        world: NonZeroU8,
        save: Save,
        /// Token required if the room has authorization enabled.
        token: Option<RoomToken>,
    },
    /// No longer supported. Use `MwQueueItem` instead.
    #[deprecated]
    MwGetItem {
        room: String,
        world: NonZeroU8,
        item: u16,
    },
    ClickMw {
        room: String,
        world: NonZeroU8,
        layout: TrackerLayout,
        cell_id: u8,
        right: bool,
        /// Token required if the room has authorization enabled.
        token: Option<RoomToken>,
    },
    SubscribeMw {
        room: String,
        world: NonZeroU8,
        layout: TrackerLayout,
    },
    /// No longer supported. Use `MwQueueItem` instead.
    #[deprecated]
    MwGetItemAll {
        room: String,
        item: u16,
    },
    MwQueueItem {
        room: String,
        source_world: NonZeroU8,
        key: u32,
        kind: u16,
        target_world: NonZeroU8,
        /// Token required if the room has authorization enabled.
        token: Option<RoomToken>,
    },
}

#[derive(Protocol)]
pub enum ServerMessage {
    Ping,
    Error {
        debug: String,
        display: String,
    },
    /// Authorization failed - the client did not provide a valid token for this operation.
    Unauthorized {
        room: String,
    },
    Init(Vec<CellRender>),
    Update {
        cell_id: u8,
        new_cell: CellRender,
    },
    InitRaw(ModelState),
    UpdateRaw(ModelDelta),
}

impl ServerMessage {
    pub fn from_error(e: impl fmt::Debug + fmt::Display) -> ServerMessage {
        ServerMessage::Error {
            debug: format!("{e:?}"),
            display: e.to_string(),
        }
    }
}
