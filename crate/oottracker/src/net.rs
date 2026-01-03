#[cfg(feature = "firebase")]
use crate::firebase;
use {
    crate::{
        game_detection::{ActiveGame, GameDetector, GameType},
        proto::{self, Packet, TCP_PORT},
        ram::{self, Ram},
        websocket, ModelState,
    },
    async_proto::Protocol as _,
    derive_more::From,
    futures::{
        future::Future,
        stream::{self, SplitSink, SplitStream, Stream, StreamExt as _, TryStreamExt as _},
    },
    itertools::Itertools as _,
    std::{
        any::TypeId,
        collections::hash_map::DefaultHasher,
        fmt,
        hash::{Hash, Hasher as _},
        io::{self, prelude::*},
        net::Ipv4Addr,
        pin::Pin,
        sync::Arc,
        time::Duration,
    },
    tokio::{
        net::{TcpListener, TcpStream, UdpSocket},
        sync::Mutex,
        time::sleep,
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
            websocket::ClientMessage::SetRaw { room, state }
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

#[derive(Debug, Clone, Copy)]
pub struct RetroArchConnection {
    pub port: u16,
}

/// State carried between tracking iterations for RetroArch connections
struct RetroArchTrackingState {
    sock: UdpSocket,
    game_detector: GameDetector,
    last_active_game: Option<ActiveGame>,
}

impl Connection for RetroArchConnection {
    fn hash(&self) -> u64 {
        let mut state = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut state);
        self.port.hash(&mut state);
        state.finish()
    }

    fn can_change_state(&self) -> bool {
        false
    }
    fn display_kind(&self) -> &'static str {
        "RetroArch"
    }

    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>> {
        let port = self.port;
        Box::pin(stream::try_unfold(
            Box::pin(async move {
                let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await?;
                sock.connect((Ipv4Addr::LOCALHOST, port)).await?;
                Ok::<_, Error>(RetroArchTrackingState {
                    sock,
                    game_detector: GameDetector::new(),
                    last_active_game: None,
                })
            }) as Pin<Box<dyn Future<Output = _> + Send>>,
            |state_fut| async move {
                sleep(Duration::from_secs(1)).await;
                let mut state: RetroArchTrackingState = state_fut.await?;

                // Detect game type at the start of each iteration
                let ram = retroarch_read_ram_with_detection(&state.sock, &mut state.game_detector)
                    .await?;

                // Check for game transitions in combo mode
                let detection_result = state.game_detector.active_game();
                if let Some(last_game) = state.last_active_game {
                    if last_game != detection_result {
                        // Game transition detected in combo mode
                        // The game_detector already tracks this internally
                    }
                }
                state.last_active_game = Some(detection_result);

                Ok(Some((
                    Packet::RamInit(ram),
                    Box::pin(async move { Ok(state) }) as Pin<Box<dyn Future<Output = _> + Send>>,
                )))
            },
        ))
    }

    fn set_state(&self, _: &ModelState) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        Box::pin(async { Err(Error::CannotChangeState) })
    }
}

/// The RetroArch UDP API does not seem to be documented,
/// but there is a Python implementation at
/// <https://github.com/eadmaster/console_hiscore/blob/master/tools/retroarchpythonapi.py>
#[allow(dead_code)]
async fn retroarch_read_ram(sock: &UdpSocket) -> Result<Ram, Error> {
    let ranges = stream::iter(ram::RANGES.iter().copied().tuples())
        .then(|(start, len)| async move { retroarch_read_memory_range(sock, start, len).await })
        .try_collect::<Vec<_>>()
        .await?;
    Ok(Ram::from_range_bufs(ranges)?)
}

/// Read RAM with game type detection.
///
/// This function detects the current game type (OoT, MM, or OoTMM combo) and reads
/// the appropriate RAM ranges based on the detected game. It also handles game
/// transitions in combo mode by detecting which game is currently active.
async fn retroarch_read_ram_with_detection(
    sock: &UdpSocket,
    detector: &mut GameDetector,
) -> Result<Ram, Error> {
    // First, detect what game is running by reading detection memory regions
    let detected_game_type = retroarch_detect_game_type(sock).await?;

    // Update the detector's game type if we detected something specific
    if detected_game_type != GameType::StandaloneOoT
        || detector.game_type() == GameType::StandaloneOoT
    {
        detector.set_game_type(detected_game_type);
    }

    // For combo mode, we need to read additional memory to detect active game
    if detected_game_type == GameType::OoTMMCombo {
        // Create a minimal RAM buffer for detection (we need context addresses)
        let mut detection_buffer = vec![0u8; crate::game_detection::RAM_SIZE];
        let oot_ctx_offset = (crate::game_detection::OOTMM_OOT_CONTEXT_ADDR
            - crate::game_detection::RDRAM_BASE) as usize;
        let mm_ctx_offset = (crate::game_detection::OOTMM_MM_CONTEXT_ADDR
            - crate::game_detection::RDRAM_BASE) as usize;

        // Read the combo context addresses directly
        let oot_ctx = retroarch_read_memory_range(sock, oot_ctx_offset as u32, 4).await?;
        let mm_ctx = retroarch_read_memory_range(sock, mm_ctx_offset as u32, 4).await?;

        // Fill in the detection buffer at the appropriate offsets
        if oot_ctx_offset + 4 <= detection_buffer.len() {
            detection_buffer[oot_ctx_offset..oot_ctx_offset + 4].copy_from_slice(&oot_ctx);
        }
        if mm_ctx_offset + 4 <= detection_buffer.len() {
            detection_buffer[mm_ctx_offset..mm_ctx_offset + 4].copy_from_slice(&mm_ctx);
        }

        // Detect which game is active in combo mode
        let _ = detector.detect_from_ram(&detection_buffer);
    }

    // Based on detected game type and active game, read the appropriate RAM ranges
    match detected_game_type {
        GameType::StandaloneOoT => {
            // Read OoT RAM ranges
            let ranges =
                stream::iter(ram::OOT_RANGES.iter().copied().tuples())
                    .then(|(start, len)| async move {
                        retroarch_read_memory_range(sock, start, len).await
                    })
                    .try_collect::<Vec<_>>()
                    .await?;
            Ok(Ram::from_range_bufs(ranges)?)
        }
        GameType::StandaloneMM => {
            // Read MM RAM ranges and create a Ram with mm_save populated
            let mm_ranges =
                stream::iter(ram::MM_RANGES.iter().copied().tuples())
                    .then(|(start, len)| async move {
                        retroarch_read_memory_range(sock, start, len).await
                    })
                    .try_collect::<Vec<_>>()
                    .await?;

            let mm_save = ram::decode_mm_range_bufs(mm_ranges)?;

            // For standalone MM, we still need to create a valid Ram struct
            // Read OoT ranges as well for the base structure (they may be zeroed/invalid)
            let oot_ranges =
                stream::iter(ram::OOT_RANGES.iter().copied().tuples())
                    .then(|(start, len)| async move {
                        retroarch_read_memory_range(sock, start, len).await
                    })
                    .try_collect::<Vec<_>>()
                    .await?;

            let mut ram = Ram::from_range_bufs(oot_ranges)?;
            ram.mm_save = Some(mm_save);
            Ok(ram)
        }
        GameType::OoTMMCombo => {
            // In combo mode, read both OoT and MM ranges
            let active_game = detector.active_game();

            // Always read OoT ranges for the base structure
            let oot_ranges =
                stream::iter(ram::OOT_RANGES.iter().copied().tuples())
                    .then(|(start, len)| async move {
                        retroarch_read_memory_range(sock, start, len).await
                    })
                    .try_collect::<Vec<_>>()
                    .await?;

            let mut ram = Ram::from_range_bufs(oot_ranges)?;

            // If MM is active or we're in combo mode, also read MM ranges
            if active_game == ActiveGame::MajorasMask {
                let mm_ranges = stream::iter(ram::MM_RANGES.iter().copied().tuples())
                    .then(|(start, len)| async move {
                        retroarch_read_memory_range(sock, start, len).await
                    })
                    .try_collect::<Vec<_>>()
                    .await?;

                let mm_save = ram::decode_mm_range_bufs(mm_ranges)?;
                ram.mm_save = Some(mm_save);
            }

            Ok(ram)
        }
    }
}

/// Detect the game type by reading memory signatures via RetroArch UDP API.
///
/// This checks for:
/// - OoT "ZELDAZ" magic at save context
/// - MM save context signatures
/// - OoTMM combo context addresses
async fn retroarch_detect_game_type(sock: &UdpSocket) -> Result<GameType, Error> {
    // Check for OoT "ZELDAZ" magic at save context offset 0x1c
    let oot_magic = retroarch_read_memory_range(sock, crate::save::ADDR + 0x1c, 6).await?;

    if &oot_magic == b"ZELDAZ" {
        // Found OoT magic - check if it's combo mode
        let oot_combo_ctx =
            retroarch_read_memory_range(sock, ram::OOT_COMBO_CONTEXT_ADDR, 4).await?;
        let mm_combo_ctx = retroarch_read_memory_range(sock, ram::MM_COMBO_CONTEXT_ADDR, 4).await?;

        // If either combo context is non-zero, this is combo mode
        if oot_combo_ctx.iter().any(|&b| b != 0) || mm_combo_ctx.iter().any(|&b| b != 0) {
            return Ok(GameType::OoTMMCombo);
        }

        return Ok(GameType::StandaloneOoT);
    }

    // Check for MM by looking at MM save context location
    let mm_check = retroarch_read_memory_range(sock, ram::MM_SAVE_ADDR, 4).await?;
    if mm_check.iter().any(|&b| b != 0) {
        // Check for combo context to be sure it's not combo mode
        let oot_combo_ctx =
            retroarch_read_memory_range(sock, ram::OOT_COMBO_CONTEXT_ADDR, 4).await?;
        let mm_combo_ctx = retroarch_read_memory_range(sock, ram::MM_COMBO_CONTEXT_ADDR, 4).await?;

        if oot_combo_ctx.iter().any(|&b| b != 0) || mm_combo_ctx.iter().any(|&b| b != 0) {
            return Ok(GameType::OoTMMCombo);
        }

        return Ok(GameType::StandaloneMM);
    }

    // Default to OoT if we can't determine the game type
    Ok(GameType::StandaloneOoT)
}

/// Read a single memory range via RetroArch UDP API
/// Converts RDRAM address to system bus address and handles word alignment
async fn retroarch_read_memory_range(
    sock: &UdpSocket,
    start: u32,
    len: u32,
) -> Result<Vec<u8>, Error> {
    let start = 0x8000_0000 + start; // ram::RANGES uses RDRAM addresses but READ_CORE_MEMORY uses system bus addresses
                                     // make sure we're word-aligned on both ends
    let offset_in_word = start & 0x3;
    let mut aligned_start = (start - offset_in_word) as usize;
    let mut aligned_len = len + offset_in_word;
    if !aligned_len.is_multiple_of(0x3) {
        aligned_len += 4 - (aligned_len & 0x3)
    }
    let mut packet_buf = [0; 4096];
    let mut ram_buf = Vec::with_capacity(aligned_len as usize);
    let mut prefix = Vec::with_capacity(21);
    let mut msg = Vec::with_capacity(26);
    while aligned_len > 0 {
        // make sure the hex-encoded response fits into the 4096-byte buffer RetroArch uses
        // each encoded byte requires 3 bytes of buffer space (the whitespace plus the 2-character hex encoding)
        const MAX_ENCODED_BYTES_PER_BUFFER: u32 =
            (4_096 - "READ_CORE_MEMORY ffffffff 9999\n".len() as u32) / 3;

        // using READ_CORE_MEMORY instead of READ_CORE_RAM as suggested in https://github.com/libretro/RetroArch/blob/0357b6c/command.h#L430-L437
        let count = aligned_len.min(MAX_ENCODED_BYTES_PER_BUFFER);
        prefix.clear();
        write!(&mut prefix, "READ_CORE_MEMORY {:x} ", aligned_start)
            .expect("failed to compose packet");
        msg.clear();
        write!(&mut msg, "READ_CORE_MEMORY {:x} ", aligned_start)
            .expect("failed to compose packet");
        writeln!(&mut msg, "{}", count).expect("failed to compose packet");
        sock.send(&msg).await?;
        let packet_len = sock.recv(&mut packet_buf).await?;
        let response = &packet_buf[prefix.len()..packet_len - 1];
        let words = response
            .split(|&sep| sep == b' ')
            .map(|byte| {
                u8::from_str_radix(&String::from_utf8_lossy(byte), 16)
                    .expect("invalid byte representation")
            })
            .tuples();
        for (b3, b2, b1, b0) in words {
            ram_buf.extend_from_slice(&[b0, b1, b2, b3]);
        }
        //if words.into_buffer().next().is_some() { panic!("did not receive a whole number of words") }
        aligned_start += count as usize;
        aligned_len -= count;
    }
    Ok(ram_buf[offset_in_word as usize..(offset_in_word + len) as usize].to_owned())
}
