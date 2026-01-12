use {
    crate::{
        save::{self, Save},
        scene::{Scene, SceneFlags},
    },
    async_proto::{Protocol, ReadError, WriteError},
    bitflags::bitflags,
    byteorder::{BigEndian, ByteOrder as _},
    derive_more::From,
    itertools::Itertools as _,
    serde::{Deserialize, Serialize},
    std::{
        array::TryFromSliceError,
        borrow::Borrow,
        fmt,
        future::Future,
        io::prelude::*,
        ops::{AddAssign, Sub},
        pin::Pin,
    },
    tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _},
};

use crate::mm_save::{self, MmSave};

pub const SIZE: usize = 0x80_0000;
pub const TEXT_LEN: usize = 0xc0;
pub const PAUSE_CTX_LEN: usize = 0x16;

// ============================================================================
// OoT RAM Ranges
// ============================================================================

pub const OOT_NUM_RANGES: usize = 8;
pub static OOT_RANGES: [u32; OOT_NUM_RANGES * 2] = [
    save::ADDR,
    save::SIZE as u32,
    0x1c84b4,
    2, // buttons currently pressed on controller 1
    0x1c8545,
    1, // current scene ID
    0x1ca1c8,
    4, // current scene's switch flags
    0x1ca1d8,
    8, // current scene's chest and room clear flags
    0x1d8870,
    2, // current text box ID
    0x1d887e,
    TEXT_LEN as u32, // current/most recent text box contents
    0x1d8dd4,
    PAUSE_CTX_LEN as u32, // relevant parts of z64_game.pause_ctxt
];

// ============================================================================
// MM RAM Ranges
// ============================================================================

/// MM SaveContext offset in RDRAM (0x801ef670 - 0x80000000)
pub const MM_SAVE_ADDR: u32 = 0x1ef670;

pub const MM_NUM_RANGES: usize = 1;
pub static MM_RANGES: [u32; MM_NUM_RANGES * 2] = [
    MM_SAVE_ADDR,
    mm_save::MM_SIZE as u32, // MM SaveContext (0x48d0 bytes)
];

// ============================================================================
// Combo Context Addresses (for OoTMM game detection)
// ============================================================================
//
// IMPORTANT: These addresses are for the ComboContext struct (24 bytes) used
// for world-switch coordination and combo mode detection ONLY.
// They should NOT be used for reading MM save data.
//
// In OoTMM combo mode:
// - MM save data at MM_SAVE_ADDR (0x1ef670) is only valid when MM engine runs
// - When OoT engine runs, MM save is in gMmSave (dynamic address, not accessible)

/// OoT combo context address offset (0x80006584 - 0x80000000)
/// Used for detecting if OoTMM combo mode is active (non-zero = combo detected)
pub const OOT_COMBO_CONTEXT_ADDR: u32 = 0x6584;
/// MM combo context address offset (0x80098280 - 0x80000000)
/// Used for detecting if OoTMM combo mode is active (non-zero = combo detected)
/// NOTE: This is NOT the MM save context address - do not use for reading MM save data.
pub const MM_COMBO_CONTEXT_ADDR: u32 = 0x98280;

// ============================================================================
// Game Type Detection
// ============================================================================

/// Detected game type for auto-tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameType {
    #[default]
    Unknown,
    /// Ocarina of Time (standalone or OoTR)
    OcarinaOfTime,
    /// Majora's Mask (standalone or MMR)
    MajorasMask,
    /// OoTMM combo randomizer
    Combo,
}

impl GameType {
    /// Check if the game is OoT or combo mode (supports OoT tracking)
    pub fn supports_oot(&self) -> bool {
        matches!(self, GameType::OcarinaOfTime | GameType::Combo)
    }

    /// Check if the game is MM or combo mode (supports MM tracking)
    pub fn supports_mm(&self) -> bool {
        matches!(self, GameType::MajorasMask | GameType::Combo)
    }
}

// ============================================================================
// Legacy aliases for backwards compatibility
// ============================================================================

pub const NUM_RANGES: usize = OOT_NUM_RANGES;
pub static RANGES: [u32; NUM_RANGES * 2] = OOT_RANGES;

#[derive(Debug, From, Clone)]
pub enum DecodeError {
    Index(u32),
    IndexRange {
        start: u32,
        end: u32,
    },
    Ranges,
    #[from]
    Save(save::DecodeError),
    #[from]
    MmSave(mm_save::MmDecodeError),
    Size(usize),
    #[from]
    TextSize(TryFromSliceError),
    UnexpectedValue {
        offset: u32,
        field: &'static str,
        value: u8,
    },
    UnexpectedValueRange {
        start: u32,
        end: u32,
        field: &'static str,
        value: Vec<u8>,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error decoding RAM: {:?}", self)
    }
}

impl std::error::Error for DecodeError {} //TODO use thiserror?

bitflags! {
    #[derive(Default)]
    pub struct Pad: u16 {
        const A = 0x8000;
        const B = 0x4000;
        const Z = 0x2000;
        const START = 0x1000;
        const D_UP = 0x0800;
        const D_DOWN = 0x0400;
        const D_LEFT = 0x0200;
        const D_RIGHT = 0x0100;
        const L = 0x0020;
        const R = 0x0010;
        const C_UP = 0x0008;
        const C_DOWN = 0x0004;
        const C_LEFT = 0x0002;
        const C_RIGHT = 0x0001;
    }
}

async_proto::bitflags!(Pad: u16);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "Vec<Vec<u8>>", into = "Vec<Vec<u8>>")]
pub struct Ram {
    pub save: Save,
    pub input_p1_raw_pad: Pad,
    pub current_scene_id: u8,
    pub current_scene_switch_flags: u32,
    pub current_scene_chest_flags: u32,
    pub current_scene_room_clear_flags: u32,
    pub current_text_box_id: u16,
    pub text_box_contents: [u8; TEXT_LEN],
    pub pause_state: u16,
    pub pause_changing: bool,
    pub pause_screen_idx: u16,
    /// Majora's Mask save data (populated when tracking MM or combo rando)
    pub mm_save: Option<MmSave>,
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            save: Save::default(),
            input_p1_raw_pad: Pad::default(),
            current_scene_id: 0,
            current_scene_switch_flags: 0,
            current_scene_chest_flags: 0,
            current_scene_room_clear_flags: 0,
            current_text_box_id: 0,
            text_box_contents: [0; TEXT_LEN],
            pause_state: 0,
            pause_changing: false,
            pause_screen_idx: 0,
            mm_save: None,
        }
    }
}

/// Raw byte data for constructing a `Ram` instance.
///
/// This struct groups the raw memory data required to construct a `Ram`,
/// reducing the number of parameters needed for `Ram::new()`.
struct RawRamData<'a> {
    save: &'a [u8],
    input_p1_raw_pad: &'a [u8],
    current_scene_id: u8,
    current_scene_switch_flags: &'a [u8],
    current_scene_chest_flags: &'a [u8],
    current_scene_room_clear_flags: &'a [u8],
    current_text_box_id: &'a [u8],
    text_box_contents: &'a [u8],
    pause_state: &'a [u8],
    pause_changing: &'a [u8],
    pause_screen_idx: &'a [u8],
}

impl Ram {
    fn new(data: RawRamData<'_>) -> Result<Self, DecodeError> {
        Ok(Self {
            save: Save::from_save_data(data.save)?,
            input_p1_raw_pad: Pad::from_bits_truncate(BigEndian::read_u16(data.input_p1_raw_pad)),
            current_scene_id: data.current_scene_id,
            current_scene_switch_flags: BigEndian::read_u32(data.current_scene_switch_flags),
            current_scene_chest_flags: BigEndian::read_u32(data.current_scene_chest_flags),
            current_scene_room_clear_flags: BigEndian::read_u32(
                data.current_scene_room_clear_flags,
            ),
            current_text_box_id: BigEndian::read_u16(data.current_text_box_id),
            text_box_contents: data.text_box_contents.try_into()?,
            pause_state: BigEndian::read_u16(data.pause_state),
            pause_changing: BigEndian::read_u16(data.pause_changing) != 0,
            pause_screen_idx: BigEndian::read_u16(data.pause_screen_idx),
            mm_save: None,
        })
    }

    pub fn from_range_bufs(ranges: impl IntoIterator<Item = Vec<u8>>) -> Result<Self, DecodeError> {
        if let Some((
            save,
            input_p1_raw_pad,
            current_scene_id,
            current_scene_switch_flags,
            chest_and_room_clear,
            current_text_box_id,
            text_box_contents,
            pause_ctx,
        )) = ranges.into_iter().collect_tuple()
        {
            let current_scene_id = match current_scene_id[..] {
                [current_scene_id] => current_scene_id,
                _ => return Err(DecodeError::Index(RANGES[2])),
            };
            let (chest_flags, room_clear_flags) = chest_and_room_clear.split_at(4);
            Self::new(RawRamData {
                save: &save,
                input_p1_raw_pad: &input_p1_raw_pad,
                current_scene_id,
                current_scene_switch_flags: &current_scene_switch_flags,
                current_scene_chest_flags: chest_flags,
                current_scene_room_clear_flags: room_clear_flags,
                current_text_box_id: &current_text_box_id,
                text_box_contents: &text_box_contents,
                pause_state: pause_ctx
                    .get(0x00..0x02)
                    .ok_or(DecodeError::Index(RANGES[12]))?,
                pause_changing: pause_ctx
                    .get(0x10..0x12)
                    .ok_or(DecodeError::Index(RANGES[12]))?,
                pause_screen_idx: pause_ctx
                    .get(0x14..0x16)
                    .ok_or(DecodeError::Index(RANGES[12]))?,
            })
        } else {
            Err(DecodeError::Ranges)
        }
    }

    pub fn from_ranges<'a, R: Borrow<[u8]> + ?Sized + 'a, I: IntoIterator<Item = &'a R>>(
        ranges: I,
    ) -> Result<Self, DecodeError> {
        if let Some((
            save,
            input_p1_raw_pad,
            &[current_scene_id],
            current_scene_switch_flags,
            chest_and_room_clear,
            current_text_box_id,
            text_box_contents,
            pause_ctx,
        )) = ranges.into_iter().map(Borrow::borrow).collect_tuple()
        {
            let (chest_flags, room_clear_flags) = chest_and_room_clear.split_at(4);
            Self::new(RawRamData {
                save,
                input_p1_raw_pad,
                current_scene_id,
                current_scene_switch_flags,
                current_scene_chest_flags: chest_flags,
                current_scene_room_clear_flags: room_clear_flags,
                current_text_box_id,
                text_box_contents,
                pause_state: pause_ctx
                    .get(0x00..0x02)
                    .ok_or(DecodeError::Index(RANGES[12]))?,
                pause_changing: pause_ctx
                    .get(0x10..0x12)
                    .ok_or(DecodeError::Index(RANGES[12]))?,
                pause_screen_idx: pause_ctx
                    .get(0x14..0x16)
                    .ok_or(DecodeError::Index(RANGES[12]))?,
            })
        } else {
            Err(DecodeError::Ranges)
        }
    }

    /// Converts an *Ocarina of Time* RAM dump into a `Ram`.
    ///
    /// # Panics
    ///
    /// This method may panic if `ram_data` doesn't contain a valid OoT RAM dump.
    pub fn from_bytes(ram_data: &[u8]) -> Result<Self, DecodeError> {
        if ram_data.len() != SIZE {
            return Err(DecodeError::Size(ram_data.len()));
        }
        Self::from_ranges(
            RANGES
                .iter()
                .tuples()
                .map(|(&start, &len)| {
                    ram_data.get(start as usize..(start + len) as usize).ok_or(
                        DecodeError::IndexRange {
                            start,
                            end: start + len,
                        },
                    )
                })
                .try_collect::<_, Vec<_>, _>()?,
        )
    }

    pub fn to_ranges(&self) -> [Vec<u8>; NUM_RANGES] {
        let mut chest_and_room_clear = Vec::with_capacity(8);
        chest_and_room_clear.extend_from_slice(&self.current_scene_chest_flags.to_be_bytes());
        chest_and_room_clear.extend_from_slice(&self.current_scene_room_clear_flags.to_be_bytes());
        let mut pause_ctx = vec![0; PAUSE_CTX_LEN];
        pause_ctx.splice(0x00..0x02, self.pause_state.to_be_bytes());
        pause_ctx.splice(
            0x10..0x12,
            if self.pause_changing { 1u16 } else { 0 }.to_be_bytes(),
        );
        pause_ctx.splice(0x14..0x16, self.pause_screen_idx.to_be_bytes());
        [
            self.save.to_save_data(),
            self.input_p1_raw_pad.bits().to_be_bytes().into(),
            vec![self.current_scene_id],
            self.current_scene_switch_flags.to_be_bytes().into(),
            chest_and_room_clear,
            self.current_text_box_id.to_be_bytes().into(),
            self.text_box_contents.into(),
            pause_ctx,
        ]
    }

    /// Returns the scene flags, with flags for the current scene updated properly.
    pub(crate) fn scene_flags(&self) -> SceneFlags {
        let mut flags = self.save.scene_flags;
        if let Some(flags_scene) = Scene::current(self)
            .ok()
            .and_then(|current_scene| flags.get_mut(current_scene))
        {
            flags_scene.set_chests(self.current_scene_chest_flags);
            flags_scene.set_switches(self.current_scene_switch_flags);
            flags_scene.set_room_clear(self.current_scene_room_clear_flags);
            //TODO set collectible flags
            //TODO set unused field? (for triforce pieces; might not be stored separately for current scene at all)
            //TODO set visited rooms (if used)
            //TODO set visited floors (if used)
        }
        flags
    }
}

impl From<Save> for Ram {
    fn from(save: Save) -> Self {
        Self {
            save,
            ..Self::default()
        }
    }
}

impl Protocol for Ram {
    fn read<'a, R: AsyncRead + Unpin + Send + 'a>(
        stream: &'a mut R,
    ) -> Pin<Box<dyn Future<Output = Result<Self, ReadError>> + Send + 'a>> {
        Box::pin(async move {
            let mut ranges = Vec::with_capacity(NUM_RANGES);
            for (_, len) in RANGES.iter().copied().tuples() {
                let mut buf = vec![0; len as usize];
                stream.read_exact(&mut buf).await?;
                ranges.push(buf);
            }
            Self::from_range_bufs(ranges)
                .map_err(|e| ReadError::Custom(format!("failed to decode RAM data: {e:?}")))
        })
    }

    fn write<'a, W: AsyncWrite + Unpin + Send + 'a>(
        &'a self,
        sink: &'a mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), WriteError>> + Send + 'a>> {
        Box::pin(async move {
            for range in self.to_ranges() {
                sink.write_all(&range).await?;
            }
            Ok(())
        })
    }

    fn read_sync(stream: &mut impl Read) -> Result<Self, ReadError> {
        let mut ranges = Vec::with_capacity(NUM_RANGES);
        for (_, len) in RANGES.iter().copied().tuples() {
            let mut buf = vec![0; len as usize];
            stream.read_exact(&mut buf)?;
            ranges.push(buf);
        }
        Self::from_range_bufs(ranges)
            .map_err(|e| ReadError::Custom(format!("failed to decode RAM data: {e:?}")))
    }

    fn write_sync(&self, sink: &mut impl Write) -> Result<(), WriteError> {
        for range in self.to_ranges() {
            sink.write_all(&range)?;
        }
        Ok(())
    }
}

impl AddAssign<Delta> for Ram {
    fn add_assign(&mut self, rhs: Delta) {
        let Delta {
            save,
            input_p1_raw_pad,
            current_scene_data,
            text_box_data,
            pause_data,
            mm_save,
        } = rhs;
        self.save = &self.save + &save;
        self.input_p1_raw_pad = input_p1_raw_pad;
        if let Some((
            current_scene_id,
            current_scene_switch_flags,
            current_scene_chest_flags,
            current_scene_room_clear_flags,
        )) = current_scene_data
        {
            self.current_scene_id = current_scene_id;
            self.current_scene_switch_flags = current_scene_switch_flags;
            self.current_scene_chest_flags = current_scene_chest_flags;
            self.current_scene_room_clear_flags = current_scene_room_clear_flags;
        }
        if let Some((current_text_box_id, text_box_contents)) = text_box_data {
            self.current_text_box_id = current_text_box_id;
            self.text_box_contents = text_box_contents;
        }
        if let Some((pause_state, pause_changing, pause_screen_idx)) = pause_data {
            self.pause_state = pause_state;
            self.pause_changing = pause_changing;
            self.pause_screen_idx = pause_screen_idx;
        }
        if let Some(new_mm_save) = mm_save {
            self.mm_save = new_mm_save;
        }
    }
}

impl Sub<&Ram> for &Ram {
    type Output = Delta;

    fn sub(self, rhs: &Ram) -> Delta {
        let Ram {
            ref save,
            input_p1_raw_pad,
            current_scene_id,
            current_scene_switch_flags,
            current_scene_chest_flags,
            current_scene_room_clear_flags,
            current_text_box_id,
            text_box_contents,
            pause_state,
            pause_changing,
            pause_screen_idx,
            ref mm_save,
        } = *self;
        Delta {
            save: save - &rhs.save,
            input_p1_raw_pad,
            current_scene_data: if current_scene_id == rhs.current_scene_id
                && current_scene_switch_flags == rhs.current_scene_switch_flags
                && current_scene_chest_flags == rhs.current_scene_chest_flags
                && current_scene_room_clear_flags == rhs.current_scene_room_clear_flags
            {
                None
            } else {
                Some((
                    current_scene_id,
                    current_scene_switch_flags,
                    current_scene_chest_flags,
                    current_scene_room_clear_flags,
                ))
            },
            text_box_data: if current_text_box_id == rhs.current_text_box_id
                && text_box_contents == rhs.text_box_contents
            {
                None
            } else {
                Some((current_text_box_id, text_box_contents))
            },
            pause_data: if pause_state == rhs.pause_state
                && pause_changing == rhs.pause_changing
                && pause_screen_idx == rhs.pause_screen_idx
            {
                None
            } else {
                Some((pause_state, pause_changing, pause_screen_idx))
            },
            mm_save: if mm_save == &rhs.mm_save {
                None
            } else {
                Some(mm_save.clone())
            },
        }
    }
}

/// The difference between two RAM states.
#[derive(Debug, Clone)]
pub struct Delta {
    save: save::Delta,
    input_p1_raw_pad: Pad,
    current_scene_data: Option<(u8, u32, u32, u32)>,
    text_box_data: Option<(u16, [u8; TEXT_LEN])>,
    pause_data: Option<(u16, bool, u16)>,
    /// Changed MM save data (None = no change, Some(value) = changed to value)
    mm_save: Option<Option<MmSave>>,
}

impl Protocol for Delta {
    fn read<'a, R: AsyncRead + Unpin + Send + 'a>(
        stream: &'a mut R,
    ) -> Pin<Box<dyn Future<Output = Result<Self, ReadError>> + Send + 'a>> {
        Box::pin(async move {
            let save = save::Delta::read(stream).await?;
            let input_p1_raw_pad = Pad::read(stream).await?;
            let current_scene_data = Option::<(u8, u32, u32, u32)>::read(stream).await?;
            let text_box_data = Option::<(u16, [u8; TEXT_LEN])>::read(stream).await?;
            let pause_data = Option::<(u16, bool, u16)>::read(stream).await?;
            // mm_save is serialized as Option<Option<Vec<u8>>>
            let mm_save_bytes = Option::<Option<Vec<u8>>>::read(stream).await?;
            let mm_save = match mm_save_bytes {
                None => None,
                Some(None) => Some(None),
                Some(Some(bytes)) => {
                    let save = MmSave::from_save_data(&bytes).map_err(|e| {
                        ReadError::Custom(format!("failed to decode MM save: {e:?}"))
                    })?;
                    Some(Some(save))
                }
            };
            Ok(Delta {
                save,
                input_p1_raw_pad,
                current_scene_data,
                text_box_data,
                pause_data,
                mm_save,
            })
        })
    }

    fn write<'a, W: AsyncWrite + Unpin + Send + 'a>(
        &'a self,
        sink: &'a mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), WriteError>> + Send + 'a>> {
        Box::pin(async move {
            self.save.write(sink).await?;
            self.input_p1_raw_pad.write(sink).await?;
            self.current_scene_data.write(sink).await?;
            self.text_box_data.write(sink).await?;
            self.pause_data.write(sink).await?;
            // Serialize mm_save as Option<Option<Vec<u8>>>
            let mm_save_bytes: Option<Option<Vec<u8>>> = match &self.mm_save {
                None => None,
                Some(None) => Some(None),
                Some(Some(save)) => Some(Some(save.to_save_data())),
            };
            mm_save_bytes.write(sink).await?;
            Ok(())
        })
    }

    fn read_sync(stream: &mut impl Read) -> Result<Self, ReadError> {
        let save = save::Delta::read_sync(stream)?;
        let input_p1_raw_pad = Pad::read_sync(stream)?;
        let current_scene_data = Option::<(u8, u32, u32, u32)>::read_sync(stream)?;
        let text_box_data = Option::<(u16, [u8; TEXT_LEN])>::read_sync(stream)?;
        let pause_data = Option::<(u16, bool, u16)>::read_sync(stream)?;
        let mm_save_bytes = Option::<Option<Vec<u8>>>::read_sync(stream)?;
        let mm_save = match mm_save_bytes {
            None => None,
            Some(None) => Some(None),
            Some(Some(bytes)) => {
                let save = MmSave::from_save_data(&bytes)
                    .map_err(|e| ReadError::Custom(format!("failed to decode MM save: {e:?}")))?;
                Some(Some(save))
            }
        };
        Ok(Delta {
            save,
            input_p1_raw_pad,
            current_scene_data,
            text_box_data,
            pause_data,
            mm_save,
        })
    }

    fn write_sync(&self, sink: &mut impl Write) -> Result<(), WriteError> {
        self.save.write_sync(sink)?;
        self.input_p1_raw_pad.write_sync(sink)?;
        self.current_scene_data.write_sync(sink)?;
        self.text_box_data.write_sync(sink)?;
        self.pause_data.write_sync(sink)?;
        let mm_save_bytes: Option<Option<Vec<u8>>> = match &self.mm_save {
            None => None,
            Some(None) => Some(None),
            Some(Some(save)) => Some(Some(save.to_save_data())),
        };
        mm_save_bytes.write_sync(sink)?;
        Ok(())
    }
}

impl From<Ram> for Vec<Vec<u8>> {
    fn from(ram: Ram) -> Self {
        ram.to_ranges().into()
    }
}

impl TryFrom<Vec<Vec<u8>>> for Ram {
    type Error = DecodeError;

    fn try_from(ranges: Vec<Vec<u8>>) -> Result<Self, DecodeError> {
        Self::from_range_bufs(ranges)
    }
}

// ============================================================================
// MM RAM Decoding Functions
// ============================================================================

/// Decodes Majora's Mask save data from a full RAM dump.
///
/// # Arguments
/// * `ram_data` - Full N64 RAM dump (must be at least `SIZE` bytes, or at least
///   contain the MM save region at `MM_SAVE_ADDR`)
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save data
/// * `Err(DecodeError)` - If the data is too small or parsing fails
///
/// # Example
/// ```ignore
/// let ram_dump = read_ram_from_emulator();
/// let mm_save = decode_mm_ranges(&ram_dump)?;
/// println!("Player has {} rupees", mm_save.rupees);
/// ```
pub fn decode_mm_ranges(ram_data: &[u8]) -> Result<MmSave, DecodeError> {
    let start = MM_SAVE_ADDR as usize;
    let end = start + mm_save::MM_SIZE;

    // Validate we have enough data
    if ram_data.len() < end {
        return Err(DecodeError::IndexRange {
            start: start as u32,
            end: end as u32,
        });
    }

    // Extract the save data slice and parse it
    let save_data = &ram_data[start..end];
    MmSave::from_save_data(save_data).map_err(DecodeError::from)
}

/// Decodes Majora's Mask save data from pre-extracted range buffers.
///
/// This function expects the data to already be extracted from the MM_RANGES
/// addresses. Currently MM only has one range (the SaveContext), so this
/// expects exactly one buffer of `MM_SIZE` bytes.
///
/// # Arguments
/// * `ranges` - Iterator yielding the memory range buffers (currently just one)
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save data
/// * `Err(DecodeError)` - If the ranges are invalid or parsing fails
pub fn decode_mm_range_bufs(
    ranges: impl IntoIterator<Item = Vec<u8>>,
) -> Result<MmSave, DecodeError> {
    let mut iter = ranges.into_iter();

    // Get the first (and only) range - the SaveContext
    let save_data = iter.next().ok_or(DecodeError::Ranges)?;

    // Validate size
    if save_data.len() != mm_save::MM_SIZE {
        return Err(DecodeError::Size(save_data.len()));
    }

    // Parse the save data
    MmSave::from_save_data(&save_data).map_err(DecodeError::from)
}

/// Decodes Majora's Mask save data from pre-extracted range slices.
///
/// Similar to `decode_mm_range_bufs` but works with borrowed slices.
///
/// # Arguments
/// * `ranges` - Iterator yielding references to memory range slices
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save data
/// * `Err(DecodeError)` - If the ranges are invalid or parsing fails
pub fn decode_mm_ranges_from_slices<'a, R, I>(ranges: I) -> Result<MmSave, DecodeError>
where
    R: Borrow<[u8]> + ?Sized + 'a,
    I: IntoIterator<Item = &'a R>,
{
    let mut iter = ranges.into_iter();

    // Get the first (and only) range - the SaveContext
    let save_data: &[u8] = iter.next().ok_or(DecodeError::Ranges)?.borrow();

    // Validate size
    if save_data.len() != mm_save::MM_SIZE {
        return Err(DecodeError::Size(save_data.len()));
    }

    // Parse the save data
    MmSave::from_save_data(save_data).map_err(DecodeError::from)
}

/// Decodes Majora's Mask save data directly from a save data buffer.
///
/// This is a convenience wrapper around `MmSave::from_save_data` that uses
/// the common `DecodeError` type.
///
/// # Arguments
/// * `save_data` - Raw save data bytes (must be exactly `MM_SIZE` bytes)
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save data
/// * `Err(DecodeError)` - If parsing fails
pub fn decode_mm_save_data(save_data: &[u8]) -> Result<MmSave, DecodeError> {
    MmSave::from_save_data(save_data).map_err(DecodeError::from)
}

/// Decodes Majora's Mask save data from pre-extracted range buffers.
///
/// Note: The `is_combo` parameter is deprecated - this tracker only supports OoTMM combo mode.
/// The parameter is kept for API compatibility but is ignored.
///
/// # Arguments
/// * `ranges` - Iterator yielding memory range buffers (expects exactly one buffer of `MM_SIZE` bytes)
/// * `_is_combo` - Deprecated, ignored (always uses OoTMM save offsets)
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save data
/// * `Err(DecodeError)` - If the ranges are invalid or parsing fails
pub fn decode_mm_range_bufs_with_type(
    ranges: impl IntoIterator<Item = Vec<u8>>,
    _is_combo: bool,
) -> Result<MmSave, DecodeError> {
    // Delegate to the regular function - we only support OoTMM now
    decode_mm_range_bufs(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_default_has_no_mm_save() {
        let ram = Ram::default();
        assert!(ram.mm_save.is_none());
    }

    #[test]
    fn test_ram_with_mm_save() {
        let mut ram = Ram::default();
        let mm_save = MmSave::default();
        ram.mm_save = Some(mm_save);
        assert!(ram.mm_save.is_some());
    }

    #[test]
    fn test_delta_no_mm_save_change() {
        let ram1 = Ram::default();
        let ram2 = Ram::default();
        let delta = &ram1 - &ram2;
        assert!(delta.mm_save.is_none());
    }

    #[test]
    fn test_delta_mm_save_change_from_none_to_some() {
        let ram1 = Ram::default();
        let ram2 = Ram {
            mm_save: Some(MmSave::default()),
            ..Default::default()
        };

        let delta = &ram2 - &ram1;
        assert!(delta.mm_save.is_some());
        assert!(delta.mm_save.as_ref().unwrap().is_some());
    }

    #[test]
    fn test_delta_mm_save_change_from_some_to_none() {
        let ram1 = Ram {
            mm_save: Some(MmSave::default()),
            ..Default::default()
        };
        let ram2 = Ram::default();

        let delta = &ram2 - &ram1;
        assert!(delta.mm_save.is_some());
        assert!(delta.mm_save.as_ref().unwrap().is_none());
    }

    #[test]
    fn test_add_assign_delta_with_mm_save() {
        let mut ram = Ram::default();
        assert!(ram.mm_save.is_none());

        // Create new state with mm_save
        let new_ram = Ram {
            mm_save: Some(MmSave::default()),
            ..Default::default()
        };

        // Create delta by subtraction
        let delta = &new_ram - &ram;

        // Apply delta
        ram += delta;
        assert!(ram.mm_save.is_some());
    }

    #[test]
    fn test_add_assign_delta_clears_mm_save() {
        let mut ram = Ram {
            mm_save: Some(MmSave::default()),
            ..Default::default()
        };
        assert!(ram.mm_save.is_some());

        // Create new state without mm_save
        let new_ram = Ram::default();

        // Create delta by subtraction
        let delta = &new_ram - &ram;

        // Apply delta
        ram += delta;
        assert!(ram.mm_save.is_none());
    }

    #[test]
    fn test_add_assign_delta_no_mm_save_change() {
        let ram = Ram {
            mm_save: Some(MmSave::default()),
            ..Default::default()
        };

        // Create identical state
        let new_ram = ram.clone();

        // Delta should have no mm_save change
        let delta = &new_ram - &ram;
        assert!(delta.mm_save.is_none());

        // Apply delta (should not change mm_save)
        let mut ram = ram;
        ram += delta;
        assert!(ram.mm_save.is_some());
    }

    // ========================================================================
    // MM Decode Tests
    // ========================================================================

    #[test]
    fn test_decode_mm_ranges_from_full_ram() {
        use mm_save::ootmm_offsets;
        // Create a simulated RAM dump with MM save data at the correct offset
        let mut ram = vec![0u8; SIZE];

        // Set some test data at the MM save offset
        let save_start = MM_SAVE_ADDR as usize;

        // Set rupees to 500 (0x01F4) at OoTMM offset 0x3A from save start
        ram[save_start + ootmm_offsets::RUPEES] = 0x01;
        ram[save_start + ootmm_offsets::RUPEES + 1] = 0xF4;

        // Set health capacity to 5 hearts (0x0140) at OoTMM offset 0x34
        ram[save_start + ootmm_offsets::HEALTH_CAPACITY] = 0x01;
        ram[save_start + ootmm_offsets::HEALTH_CAPACITY + 1] = 0x40;

        // Set player form to Goron (1) at offset 0x20 (same for OoTMM)
        ram[save_start + ootmm_offsets::PLAYER_FORM] = 0x01;

        let save = decode_mm_ranges(&ram).expect("Failed to decode MM ranges");

        assert_eq!(save.rupees, 500);
        assert_eq!(save.health_capacity, 0x0140);
        assert_eq!(save.player_form, mm_save::PlayerForm::Goron);
    }

    #[test]
    fn test_decode_mm_ranges_too_small() {
        // Create a RAM dump that's too small
        let small_ram = vec![0u8; MM_SAVE_ADDR as usize + 100]; // Not enough for full save

        let result = decode_mm_ranges(&small_ram);
        assert!(matches!(result, Err(DecodeError::IndexRange { .. })));
    }

    #[test]
    fn test_decode_mm_range_bufs() {
        use mm_save::ootmm_offsets;
        // Create save data buffer
        let mut save_data = vec![0u8; mm_save::MM_SIZE];

        // Set day to 2 at OoTMM offset 0x18
        save_data[ootmm_offsets::DAY..ootmm_offsets::DAY + 4].copy_from_slice(&2u32.to_be_bytes());

        // Set sword to Gilded (3) and shield to Mirror (3) at OoTMM offset 0x6C
        // OoTMM uses u16 equipment field - sword in low nibble, shield in next nibble
        // MirrorShield is now value 3 (HylianShield is 2)
        let equipment: u16 = 0x03 | (0x03 << 4);
        save_data[ootmm_offsets::SWORD_SHIELD..ootmm_offsets::SWORD_SHIELD + 2]
            .copy_from_slice(&equipment.to_be_bytes());

        let save = decode_mm_range_bufs(vec![save_data]).expect("Failed to decode MM range bufs");

        assert_eq!(save.day, 2);
        assert_eq!(save.sword, mm_save::MmSword::GildedSword);
        assert_eq!(save.shield, mm_save::MmShield::MirrorShield);
    }

    #[test]
    fn test_decode_mm_range_bufs_empty() {
        let result = decode_mm_range_bufs(Vec::<Vec<u8>>::new());
        assert!(matches!(result, Err(DecodeError::Ranges)));
    }

    #[test]
    fn test_decode_mm_range_bufs_wrong_size() {
        let wrong_size_data = vec![0u8; 100];
        let result = decode_mm_range_bufs(vec![wrong_size_data]);
        assert!(matches!(result, Err(DecodeError::Size(100))));
    }

    #[test]
    fn test_decode_mm_ranges_from_slices() {
        use mm_save::ootmm_offsets;
        let mut save_data = vec![0u8; mm_save::MM_SIZE];

        // Set stray fairies at OoTMM offset 0xD2
        save_data[ootmm_offsets::STRAY_FAIRIES] = 1; // Clock Town
        save_data[ootmm_offsets::STRAY_FAIRIES + 1] = 15; // Woodfall
        save_data[ootmm_offsets::STRAY_FAIRIES + 2] = 8; // Snowhead

        let ranges: Vec<&[u8]> = vec![&save_data];
        let save = decode_mm_ranges_from_slices(ranges).expect("Failed to decode from slices");

        assert_eq!(save.stray_fairies.clock_town, 1);
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert_eq!(save.stray_fairies.snowhead, 8);
    }

    #[test]
    fn test_decode_mm_save_data_direct() {
        use mm_save::ootmm_offsets;
        let mut save_data = vec![0u8; mm_save::MM_SIZE];

        // Set quest items at OoTMM offset 0xBA
        // OoTMM uses the same bit layout as vanilla MM
        let quest_bits: u32 = 0x00001001;
        save_data[ootmm_offsets::QUEST_ITEMS..ootmm_offsets::QUEST_ITEMS + 4]
            .copy_from_slice(&quest_bits.to_be_bytes());

        let save = decode_mm_save_data(&save_data).expect("Failed to decode save data");

        // Just verify parsing works without errors
        assert!(save.quest_items.bits() != 0 || quest_bits == 0);
    }

    #[test]
    fn test_decode_mm_ranges_inventory() {
        use mm_save::ootmm_offsets;
        let mut ram = vec![0u8; SIZE];
        let save_start = MM_SAVE_ADDR as usize;

        // Set inventory items at OoTMM offset 0x6E
        // Ocarina = 0x00, Bow = 0x01, Hookshot = 0x0F
        ram[save_start + ootmm_offsets::INVENTORY] = 0x00; // Ocarina in slot 0
        ram[save_start + ootmm_offsets::INVENTORY + 1] = 0x01; // Bow in slot 1
        ram[save_start + ootmm_offsets::INVENTORY + 15] = 0x0F; // Hookshot in slot 15

        let save = decode_mm_ranges(&ram).expect("Failed to decode");

        assert!(save.inventory.ocarina);
        assert!(save.inventory.bow);
        assert!(save.inventory.hookshot);
        assert!(!save.inventory.bombs); // Not set
    }

    #[test]
    fn test_decode_mm_ranges_masks() {
        use mm_save::ootmm_offsets;
        let mut ram = vec![0u8; SIZE];
        let save_start = MM_SAVE_ADDR as usize;

        // Set masks at OoTMM offset 0x8E
        // Deku = 0x32, Goron = 0x33, Bunny = 0x39 (per zeldaret/mm decomp)
        ram[save_start + ootmm_offsets::MASKS] = 0x32; // Deku mask in slot 0
        ram[save_start + ootmm_offsets::MASKS + 1] = 0x33; // Goron mask in slot 1
        ram[save_start + ootmm_offsets::MASKS + 2] = 0x39; // Bunny mask in slot 2

        let save = decode_mm_ranges(&ram).expect("Failed to decode");

        assert!(save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::DEKU));
        assert!(save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::GORON));
        assert!(save.masks.masks_low.contains(mm_save::MmMasksLow::BUNNY));
        assert!(!save
            .masks
            .transformation
            .contains(mm_save::MmTransformationMasks::ZORA));
    }

    #[test]
    fn test_decode_mm_ranges_dungeon_items() {
        use mm_save::ootmm_offsets;
        let mut ram = vec![0u8; SIZE];
        let save_start = MM_SAVE_ADDR as usize;

        // Set dungeon items at OoTMM offset 0xBE
        // Woodfall: Map + Compass + Boss Key (0xE0 = 0x80 + 0x40 + 0x20)
        // N64 big-endian: BOSS_KEY=0x80, COMPASS=0x40, MAP=0x20
        ram[save_start + ootmm_offsets::DUNGEON_ITEMS] = 0xE0;
        // Snowhead: Map only (0x20)
        ram[save_start + ootmm_offsets::DUNGEON_ITEMS + 1] = 0x20;

        // Set small keys at OoTMM offset 0xC8
        ram[save_start + ootmm_offsets::SMALL_KEYS] = 2; // Woodfall: 2 keys
        ram[save_start + ootmm_offsets::SMALL_KEYS + 1] = 3; // Snowhead: 3 keys

        let save = decode_mm_ranges(&ram).expect("Failed to decode");

        assert!(save
            .dungeon_items
            .woodfall
            .contains(mm_save::MmDungeonItems::MAP));
        assert!(save
            .dungeon_items
            .woodfall
            .contains(mm_save::MmDungeonItems::COMPASS));
        assert!(save
            .dungeon_items
            .woodfall
            .contains(mm_save::MmDungeonItems::BOSS_KEY));
        assert!(save
            .dungeon_items
            .snowhead
            .contains(mm_save::MmDungeonItems::MAP));
        assert!(!save
            .dungeon_items
            .snowhead
            .contains(mm_save::MmDungeonItems::BOSS_KEY));

        assert_eq!(save.small_keys.woodfall, 2);
        assert_eq!(save.small_keys.snowhead, 3);
    }

    #[test]
    fn test_decode_mm_ranges_skulltulas() {
        use mm_save::ootmm_offsets;
        let mut ram = vec![0u8; SIZE];
        let save_start = MM_SAVE_ADDR as usize;

        // Set skulltula counts at OoTMM offsets 0xDC (swamp) and 0xDE (ocean)
        // These are u16 big-endian values
        ram[save_start + ootmm_offsets::SKULL_SWAMP] = 0x00;
        ram[save_start + ootmm_offsets::SKULL_SWAMP + 1] = 0x1E; // 30 swamp
        ram[save_start + ootmm_offsets::SKULL_OCEAN] = 0x00;
        ram[save_start + ootmm_offsets::SKULL_OCEAN + 1] = 0x14; // 20 ocean

        let save = decode_mm_ranges(&ram).expect("Failed to decode");

        assert_eq!(save.skull_tokens_swamp, 30);
        assert_eq!(save.skull_tokens_ocean, 20);
    }

    #[test]
    fn test_decode_error_from_mm_decode_error() {
        // Test that MmDecodeError converts to DecodeError properly
        let mm_err = mm_save::MmDecodeError::Size(100);
        let decode_err: DecodeError = mm_err.into();
        assert!(matches!(decode_err, DecodeError::MmSave(_)));
    }

    #[test]
    fn test_mm_ranges_constants() {
        // Verify the MM ranges are set up correctly
        assert_eq!(MM_NUM_RANGES, 1);
        assert_eq!(MM_RANGES.len(), 2);
        assert_eq!(MM_RANGES[0], MM_SAVE_ADDR);
        assert_eq!(MM_RANGES[1], mm_save::MM_SIZE as u32);
    }
}
