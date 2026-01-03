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

use crate::mm_save;

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

/// OoT combo context address offset (0x80006584 - 0x80000000)
pub const OOT_COMBO_CONTEXT_ADDR: u32 = 0x6584;
/// MM combo context address offset (0x80098280 - 0x80000000)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
        }
    }
}

/// The difference between two RAM states.
#[derive(Debug, Clone, Protocol)]
pub struct Delta {
    save: save::Delta,
    input_p1_raw_pad: Pad,
    current_scene_data: Option<(u8, u32, u32, u32)>,
    text_box_data: Option<(u16, [u8; TEXT_LEN])>,
    pause_data: Option<(u16, bool, u16)>,
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
// MM RAM Decoding
// ============================================================================

/// Decodes Majora's Mask RAM data into an `MmSave` structure.
///
/// This function can accept data in multiple formats:
/// - Full N64 RAM dump (8MB): Extracts MM save context from `MM_SAVE_ADDR`
/// - Pre-extracted MM ranges: Exactly `mm_save::MM_SIZE` bytes of save context
/// - Range buffer format: Vector of ranges matching `MM_RANGES`
///
/// # Arguments
/// * `data` - Raw RAM data bytes
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save state
/// * `Err(DecodeError)` - If the data is invalid or wrong size
///
/// # Examples
/// ```ignore
/// // From full RAM dump
/// let ram_data = read_n64_ram(); // 8MB dump
/// let mm_save = decode_mm_ranges(&ram_data)?;
///
/// // From pre-extracted save context
/// let save_ctx = &ram_data[MM_SAVE_ADDR as usize..][..mm_save::MM_SIZE];
/// let mm_save = decode_mm_ranges(save_ctx)?;
/// ```
pub fn decode_mm_ranges(data: &[u8]) -> Result<mm_save::MmSave, DecodeError> {
    // Case 1: Exact MM save context size
    if data.len() == mm_save::MM_SIZE {
        return Ok(mm_save::MmSave::from_save_data(data)?);
    }

    // Case 2: Full N64 RAM dump (8MB)
    if data.len() == SIZE {
        let start = MM_SAVE_ADDR as usize;
        let end = start + mm_save::MM_SIZE;
        let save_data = data.get(start..end).ok_or(DecodeError::IndexRange {
            start: MM_SAVE_ADDR,
            end: MM_SAVE_ADDR + mm_save::MM_SIZE as u32,
        })?;
        return Ok(mm_save::MmSave::from_save_data(save_data)?);
    }

    // Case 3: Data larger than MM_SIZE but not full RAM - try extracting from start
    // This handles cases where only partial RAM was captured but includes the save area
    if data.len() > mm_save::MM_SIZE {
        // Try to use the first MM_SIZE bytes as the save context
        let save_data = &data[..mm_save::MM_SIZE];
        return Ok(mm_save::MmSave::from_save_data(save_data)?);
    }

    // Data is too small to contain MM save context
    Err(DecodeError::Size(data.len()))
}

/// Decodes MM RAM from pre-extracted range buffers.
///
/// This is the equivalent of `Ram::from_range_bufs` for Majora's Mask.
/// It expects ranges matching the `MM_RANGES` array layout.
///
/// # Arguments
/// * `ranges` - Iterator of byte vectors, one for each range in `MM_RANGES`
///
/// # Returns
/// * `Ok(MmSave)` - Successfully decoded MM save state
/// * `Err(DecodeError)` - If the ranges are invalid
pub fn decode_mm_range_bufs(
    ranges: impl IntoIterator<Item = Vec<u8>>,
) -> Result<mm_save::MmSave, DecodeError> {
    let ranges: Vec<_> = ranges.into_iter().collect();

    // MM_RANGES currently only has one range (the save context)
    if ranges.len() != MM_NUM_RANGES {
        return Err(DecodeError::Ranges);
    }

    // First (and only) range should be the MM save context
    let save_data = ranges.into_iter().next().ok_or(DecodeError::Ranges)?;

    if save_data.len() != mm_save::MM_SIZE {
        return Err(DecodeError::Size(save_data.len()));
    }

    Ok(mm_save::MmSave::from_save_data(&save_data)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // decode_mm_ranges Tests
    // ========================================================================

    /// Creates sample MM save data with specific values set for testing
    fn create_sample_mm_save_data() -> Vec<u8> {
        let mut data = vec![0u8; mm_save::MM_SIZE];

        // Set player form to Goron (1) at offset 0x0020
        data[0x0020] = 1;

        // Set health capacity to 0x0140 (5 hearts) at offset 0x002C
        data[0x002C] = 0x01;
        data[0x002D] = 0x40;

        // Set current health to 0x0100 (4 hearts) at offset 0x002E
        data[0x002E] = 0x01;
        data[0x002F] = 0x00;

        // Set magic level to 2 (double magic) at offset 0x0032
        data[0x0032] = 2;

        // Set rupees to 500 (0x01F4) at offset 0x0034
        data[0x0034] = 0x01;
        data[0x0035] = 0xF4;

        // Set sword to Gilded (3) and shield to Mirror (2) at offset 0x0044
        data[0x0044] = 0x03 | (0x02 << 4);

        // Set double defense at offset 0x003B
        data[0x003B] = 1;

        // Set quest items: Odolwa remains + Song of Time at offset 0x00A4
        // REMAINS_ODOLWA = 1 << 0, SONG_TIME = 1 << 12 = 0x1001
        let quest_bits: u32 = 0x00001001;
        data[0x00A4..0x00A8].copy_from_slice(&quest_bits.to_be_bytes());

        // Set stray fairies at offset 0x00D0
        data[0x00D0] = 1; // Clock Town
        data[0x00D1] = 15; // Woodfall
        data[0x00D2] = 10; // Snowhead
        data[0x00D3] = 5; // Great Bay
        data[0x00D4] = 0; // Stone Tower

        // Set skulltula tokens at offset 0x00D8 and 0x00DA
        data[0x00D8] = 0x00;
        data[0x00D9] = 0x14; // Swamp: 20
        data[0x00DA] = 0x00;
        data[0x00DB] = 0x0A; // Ocean: 10

        // Set day to 2 at offset 0x0048
        data[0x0048..0x004C].copy_from_slice(&2u32.to_be_bytes());

        // Set time to 0x8000 at offset 0x004C
        data[0x004C..0x004E].copy_from_slice(&0x8000u16.to_be_bytes());

        // Set is_night to true at offset 0x0050
        data[0x0050] = 1;

        data
    }

    #[test]
    fn test_decode_mm_ranges_exact_size() {
        let data = create_sample_mm_save_data();
        let result = decode_mm_ranges(&data);

        assert!(result.is_ok());
        let save = result.unwrap();

        assert_eq!(save.player_form, mm_save::PlayerForm::Goron);
        assert_eq!(save.health_capacity, 0x0140);
        assert_eq!(save.health, 0x0100);
        assert_eq!(save.magic, mm_save::MmMagicCapacity::Double);
        assert_eq!(save.rupees, 500);
        assert_eq!(save.sword, mm_save::MmSword::GildedSword);
        assert_eq!(save.shield, mm_save::MmShield::MirrorShield);
        assert!(save.double_defense);
    }

    #[test]
    fn test_decode_mm_ranges_quest_items() {
        let data = create_sample_mm_save_data();
        let save = decode_mm_ranges(&data).unwrap();

        assert!(save
            .quest_items
            .contains(mm_save::MmQuestItems::REMAINS_ODOLWA));
        assert!(!save
            .quest_items
            .contains(mm_save::MmQuestItems::REMAINS_GOHT));
        assert!(save.quest_items.contains(mm_save::MmQuestItems::SONG_TIME));
        assert_eq!(save.quest_items.num_remains(), 1);
    }

    #[test]
    fn test_decode_mm_ranges_stray_fairies() {
        let data = create_sample_mm_save_data();
        let save = decode_mm_ranges(&data).unwrap();

        assert_eq!(save.stray_fairies.clock_town, 1);
        assert_eq!(save.stray_fairies.woodfall, 15);
        assert_eq!(save.stray_fairies.snowhead, 10);
        assert_eq!(save.stray_fairies.great_bay, 5);
        assert_eq!(save.stray_fairies.stone_tower, 0);
        assert_eq!(save.stray_fairies.dungeon_total(), 30);
    }

    #[test]
    fn test_decode_mm_ranges_skulltulas() {
        let data = create_sample_mm_save_data();
        let save = decode_mm_ranges(&data).unwrap();

        assert_eq!(save.skull_tokens_swamp, 20);
        assert_eq!(save.skull_tokens_ocean, 10);
    }

    #[test]
    fn test_decode_mm_ranges_time_state() {
        let data = create_sample_mm_save_data();
        let save = decode_mm_ranges(&data).unwrap();

        assert_eq!(save.day, 2);
        assert_eq!(save.time, 0x8000);
        assert!(save.is_night);
    }

    #[test]
    fn test_decode_mm_ranges_from_full_ram() {
        // Create full 8MB RAM dump with MM save context at correct offset
        let mut full_ram = vec![0u8; SIZE];
        let save_data = create_sample_mm_save_data();

        // Copy save data to MM_SAVE_ADDR offset
        let start = MM_SAVE_ADDR as usize;
        full_ram[start..start + mm_save::MM_SIZE].copy_from_slice(&save_data);

        let result = decode_mm_ranges(&full_ram);
        assert!(result.is_ok());

        let save = result.unwrap();
        assert_eq!(save.player_form, mm_save::PlayerForm::Goron);
        assert_eq!(save.health_capacity, 0x0140);
        assert_eq!(save.rupees, 500);
    }

    #[test]
    fn test_decode_mm_ranges_larger_than_mm_size() {
        // Data larger than MM_SIZE but not full RAM
        let mut data = vec![0u8; mm_save::MM_SIZE + 1000];

        // Set some values in the first MM_SIZE bytes
        data[0x0034] = 0x00;
        data[0x0035] = 0x64; // Rupees: 100

        let result = decode_mm_ranges(&data);
        assert!(result.is_ok());

        let save = result.unwrap();
        assert_eq!(save.rupees, 100);
    }

    #[test]
    fn test_decode_mm_ranges_too_small() {
        let data = vec![0u8; 100]; // Way too small
        let result = decode_mm_ranges(&data);

        assert!(matches!(result, Err(DecodeError::Size(100))));
    }

    #[test]
    fn test_decode_mm_ranges_empty() {
        let data: Vec<u8> = vec![];
        let result = decode_mm_ranges(&data);

        assert!(matches!(result, Err(DecodeError::Size(0))));
    }

    #[test]
    fn test_decode_mm_ranges_zeroed_data() {
        // All zeros should parse without errors (default state)
        let data = vec![0u8; mm_save::MM_SIZE];
        let result = decode_mm_ranges(&data);

        assert!(result.is_ok());
        let save = result.unwrap();

        // Check default values
        assert_eq!(save.player_form, mm_save::PlayerForm::FierceDeity); // 0 maps to FierceDeity
        assert_eq!(save.health_capacity, 0);
        assert_eq!(save.health, 0);
        assert_eq!(save.rupees, 0);
        assert_eq!(save.day, 0);
    }

    // ========================================================================
    // decode_mm_range_bufs Tests
    // ========================================================================

    #[test]
    fn test_decode_mm_range_bufs_success() {
        let save_data = create_sample_mm_save_data();
        let ranges = vec![save_data];

        let result = decode_mm_range_bufs(ranges);
        assert!(result.is_ok());

        let save = result.unwrap();
        assert_eq!(save.player_form, mm_save::PlayerForm::Goron);
        assert_eq!(save.rupees, 500);
    }

    #[test]
    fn test_decode_mm_range_bufs_wrong_range_count() {
        // Empty ranges
        let ranges: Vec<Vec<u8>> = vec![];
        let result = decode_mm_range_bufs(ranges);
        assert!(matches!(result, Err(DecodeError::Ranges)));

        // Too many ranges
        let ranges = vec![vec![0u8; mm_save::MM_SIZE], vec![0u8; 100]];
        let result = decode_mm_range_bufs(ranges);
        assert!(matches!(result, Err(DecodeError::Ranges)));
    }

    #[test]
    fn test_decode_mm_range_bufs_wrong_size() {
        let ranges = vec![vec![0u8; 100]]; // Wrong size
        let result = decode_mm_range_bufs(ranges);
        assert!(matches!(result, Err(DecodeError::Size(100))));
    }

    // ========================================================================
    // Integration with MM_RANGES constants
    // ========================================================================

    #[test]
    fn test_mm_ranges_constants() {
        // Verify MM_RANGES has expected values
        assert_eq!(MM_NUM_RANGES, 1);
        assert_eq!(MM_RANGES[0], MM_SAVE_ADDR);
        assert_eq!(MM_RANGES[1], mm_save::MM_SIZE as u32);
    }

    #[test]
    fn test_mm_save_addr_value() {
        // Verify MM_SAVE_ADDR matches the expected address
        assert_eq!(MM_SAVE_ADDR, 0x1ef670);
    }

    #[test]
    fn test_mm_size_constant() {
        // Verify MM_SIZE matches expected value
        assert_eq!(mm_save::MM_SIZE, 0x48d0);
    }

    // ========================================================================
    // Endianness Tests (N64 is big-endian)
    // ========================================================================

    #[test]
    fn test_big_endian_u16_parsing() {
        let mut data = vec![0u8; mm_save::MM_SIZE];

        // Set rupees to 0x1234 using big-endian
        data[0x0034] = 0x12; // High byte
        data[0x0035] = 0x34; // Low byte

        let save = decode_mm_ranges(&data).unwrap();
        assert_eq!(save.rupees, 0x1234);
    }

    #[test]
    fn test_big_endian_u32_parsing() {
        let mut data = vec![0u8; mm_save::MM_SIZE];

        // Set quest items using big-endian with known valid bits:
        // REMAINS_ODOLWA | REMAINS_GOHT | SONG_TIME = 0x1003
        let quest_bits: u32 = 0x00001003;
        data[0x00A4..0x00A8].copy_from_slice(&quest_bits.to_be_bytes());

        let save = decode_mm_ranges(&data).unwrap();
        assert_eq!(save.quest_items.bits(), 0x00001003);
    }

    // ========================================================================
    // Roundtrip Tests
    // ========================================================================

    #[test]
    fn test_decode_encode_roundtrip() {
        let original_data = create_sample_mm_save_data();
        let save = decode_mm_ranges(&original_data).unwrap();

        // Encode back to bytes
        let encoded = save.to_save_data();

        // Decode again
        let save2 = decode_mm_ranges(&encoded).unwrap();

        // Key fields should match
        assert_eq!(save.player_form, save2.player_form);
        assert_eq!(save.health_capacity, save2.health_capacity);
        assert_eq!(save.health, save2.health);
        assert_eq!(save.magic, save2.magic);
        assert_eq!(save.rupees, save2.rupees);
        assert_eq!(save.sword, save2.sword);
        assert_eq!(save.shield, save2.shield);
        assert_eq!(save.double_defense, save2.double_defense);
        assert_eq!(save.quest_items, save2.quest_items);
        assert_eq!(save.stray_fairies, save2.stray_fairies);
        assert_eq!(save.skull_tokens_swamp, save2.skull_tokens_swamp);
        assert_eq!(save.skull_tokens_ocean, save2.skull_tokens_ocean);
        assert_eq!(save.day, save2.day);
        assert_eq!(save.time, save2.time);
        assert_eq!(save.is_night, save2.is_night);
    }

    // ========================================================================
    // DecodeError Tests
    // ========================================================================

    #[test]
    fn test_decode_error_display() {
        let err = DecodeError::Size(100);
        let msg = format!("{}", err);
        assert!(msg.contains("error decoding RAM"));
    }

    #[test]
    fn test_decode_error_from_mm_decode_error() {
        let mm_err = mm_save::MmDecodeError::Size(50);
        let err: DecodeError = mm_err.into();
        assert!(matches!(err, DecodeError::MmSave(_)));
    }
}
