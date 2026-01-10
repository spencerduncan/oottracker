#[cfg(feature = "firebase")]
use crate::firebase;
use {
    crate::{
        proto::{self, Packet, TCP_PORT},
        ram,
        websocket, ModelState,
    },
    async_proto::Protocol as _,
    derive_more::From,
    futures::{
        future::Future,
        stream::{self, SplitSink, SplitStream, Stream, StreamExt as _, TryStreamExt as _},
    },
    std::{
        any::TypeId,
        collections::hash_map::DefaultHasher,
        fmt,
        hash::{Hash, Hasher as _},
        io,
        net::Ipv4Addr,
        pin::Pin,
        sync::Arc,
    },
    tokio::{
        net::{TcpListener, TcpStream},
        sync::Mutex,
    },
    tokio_stream::wrappers::TcpListenerStream,
    tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream},
    wheel::FromArc,
};

#[derive(Debug, From, FromArc, Clone)]
pub enum Error {
    CannotChangeState,
    #[cfg(feature = "firebase")]
    Firebase(firebase::Error),
    #[from_arc]
    Io(Arc<io::Error>),
    Protocol(proto::ReadError),
    RamDecode(ram::DecodeError),
    UnexpectedWebsocketMessage,
    Websocket {
        debug: String,
        display: String,
    },
    #[from_arc]
    Write(Arc<async_proto::WriteError>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotChangeState => write!(f, "this type of connection is read-only"),
            #[cfg(feature = "firebase")]
            Error::Firebase(e) => e.fmt(f),
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Protocol(e) => e.fmt(f),
            Error::RamDecode(e) => write!(f, "error decoding game RAM: {:?}", e),
            Error::UnexpectedWebsocketMessage => {
                write!(f, "unexpected WebSocket message kind from server")
            }
            Error::Websocket { display, .. } => display.fmt(f),
            Error::Write(e) => e.fmt(f),
        }
    }
}

pub trait Connection: fmt::Debug + Send + Sync {
    fn hash(&self) -> u64;
    fn can_change_state(&self) -> bool;
    fn display_kind(&self) -> &'static str;
    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>>;
    fn set_state(
        &self,
        model: &ModelState,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

    #[cfg(feature = "firebase")]
    fn firebase_app(&self) -> Option<&dyn firebase::App> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NullConnection;

impl Connection for NullConnection {
    fn hash(&self) -> u64 {
        let mut state = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut state);
        state.finish()
    }

    fn can_change_state(&self) -> bool {
        false
    }
    fn display_kind(&self) -> &'static str {
        "nothing"
    }

    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>> {
        Box::pin(stream::pending())
    }

    fn set_state(&self, _: &ModelState) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        Box::pin(async { Err(Error::CannotChangeState) })
    }
}

type WsStream = Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>;
type WsSink =
    Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>>>;

pub struct WebConnection {
    room: String,
    sink: WsSink,
    stream: WsStream,
}

impl WebConnection {
    pub async fn new(room: impl ToString) -> Result<WebConnection, async_proto::WriteError> {
        let (mut sink, stream) =
            tokio_tungstenite::connect_async("wss://oottracker.fenhl.net/websocket")
                .await?
                .0
                .split();
        websocket::ClientMessage::SubscribeRaw {
            room: room.to_string(),
        }
        .write_ws(&mut sink)
        .await?;
        Ok(WebConnection {
            room: room.to_string(),
            sink: Arc::new(Mutex::new(sink)),
            stream: Arc::new(Mutex::new(stream)),
        })
    }
}

impl fmt::Debug for WebConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebConnection {{ room: {:?}, .. }}", self.room) //TODO finish_non_exhaustive
    }
}

impl Connection for WebConnection {
    fn hash(&self) -> u64 {
        let mut state = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut state);
        self.room.hash(&mut state);
        state.finish()
    }

    fn can_change_state(&self) -> bool {
        true
    } //TODO support for read-only (passwordless) connections?
    fn display_kind(&self) -> &'static str {
        "web"
    }

    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>> {
        let stream = Arc::clone(&self.stream);
        Box::pin(stream::unfold(stream, |stream| async move {
            loop {
                let stream_clone = Arc::clone(&stream);
                break match websocket::ServerMessage::read_ws(&mut *stream_clone.lock().await).await
                {
                    Ok(websocket::ServerMessage::Ping) => continue,
                    Ok(websocket::ServerMessage::Error { debug, display }) => {
                        Some((Err(Error::Websocket { debug, display }), stream))
                    }
                    Ok(websocket::ServerMessage::Unauthorized { room }) => Some((
                        Err(Error::Websocket {
                            debug: format!("unauthorized for room {room:?}"),
                            display: format!(
                                "unauthorized: invalid or missing token for room '{room}'"
                            ),
                        }),
                        stream,
                    )),
                    Ok(websocket::ServerMessage::Init(_))
                    | Ok(websocket::ServerMessage::Update { .. }) => {
                        Some((Err(Error::UnexpectedWebsocketMessage), stream))
                    }
                    Ok(websocket::ServerMessage::InitRaw(model)) => {
                        Some((Ok(Packet::ModelInit(model)), stream))
                    }
                    Ok(websocket::ServerMessage::UpdateRaw(delta)) => {
                        Some((Ok(Packet::ModelDelta(delta)), stream))
                    }
                    Err(e) => Some((
                        Err(Error::Protocol(proto::ReadError::Packet(Arc::new(e)))),
                        stream,
                    )),
                };
            }
        }))
    }

    fn set_state(
        &self,
        model: &ModelState,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let room = self.room.clone();
        let state = model.clone();
        let sink = Arc::clone(&self.sink);
        Box::pin(async move {
            websocket::ClientMessage::SetRaw {
                room,
                state,
                token: None, // No authentication token provided for legacy clients
            }
            .write_ws(&mut *sink.lock().await)
            .await?;
            Ok(())
        })
    }
}

#[cfg(feature = "firebase")]
#[derive(Debug)]
pub struct FirebaseConnection {
    app: Box<dyn firebase::App>,
    room: firebase::DynRoom,
}

#[cfg(feature = "firebase")]
impl FirebaseConnection {
    pub fn new<A: firebase::App + Default + Clone + Send>(
        room: firebase::Room<A>,
    ) -> FirebaseConnection {
        FirebaseConnection {
            app: Box::new(A::default()),
            room: room.to_dyn(),
        }
    }
}

#[cfg(feature = "firebase")]
impl Connection for FirebaseConnection {
    fn hash(&self) -> u64 {
        let mut state = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut state);
        self.room.hash(&mut state);
        state.finish()
    }

    fn can_change_state(&self) -> bool {
        true
    } //TODO support for read-only (passwordless) connections?
    fn display_kind(&self) -> &'static str {
        "Firebase"
    }

    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>> {
        Box::pin(
            self.room
                .subscribe()
                .map_ok(|(cell, new_value)| Packet::UpdateCell(cell, new_value))
                .err_into(),
        )
    }

    fn set_state(
        &self,
        model: &ModelState,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let room = self.room.clone();
        let model = model.clone();
        Box::pin(async move { Ok(room.set_state(&model).await?) })
    }

    fn firebase_app(&self) -> Option<&dyn firebase::App> {
        Some(&self.app)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TcpConnection;

impl Connection for TcpConnection {
    fn hash(&self) -> u64 {
        let mut state = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut state);
        state.finish()
    }

    fn can_change_state(&self) -> bool {
        false
    } //TODO support for two-way TCP connections?
    fn display_kind(&self) -> &'static str {
        "TCP"
    }

    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>> {
        Box::pin(
            stream::once(async { TcpListener::bind((Ipv4Addr::LOCALHOST, TCP_PORT)).await })
                .map_ok(|listener| TcpListenerStream::new(listener).err_into::<Error>())
                .try_flatten()
                .map_ok(|tcp_stream| proto::read(tcp_stream).err_into::<Error>())
                .try_flatten(),
        )
    }

    fn set_state(&self, _: &ModelState) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        Box::pin(async { Err(Error::CannotChangeState) })
    }
}
