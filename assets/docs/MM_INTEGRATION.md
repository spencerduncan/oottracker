# OoT/MM Combined Randomizer Integration

**Document Version:** 1.1
**Date:** 2025-12-31
**Status:** Planning / Analysis
**Last Updated:** 2025-12-31 (corrected ROM architecture)

---

## ⚠️ IMPORTANT CORRECTION

**This document was written based on an incorrect assumption about OoTMM's architecture.**

### Original (Incorrect) Assumption
The document initially assumed OoTMM involved tracking two separate games/ROMs simultaneously or sequentially.

### Actual OoTMM Architecture
[OoTMM](https://ootmm.com/) creates a **single combined ROM** that merges both games:
- Both OoT and MM are embedded in one ROM file
- MM is placed at a 32 MiB offset (0x2000000) within the ROM
- Players switch between games at specific locations:
  - **Happy Mask Shop door** (OoT) ↔ **Clock Tower door** (MM)
  - These doors serve as the crossover point between games
- The game **auto-saves when switching** between OoT and MM
- **Single save file** shared between both games
- Only one game is "active" at a time (OoT mode or MM mode)

### Impact on Requirements
This significantly **simplifies** some requirements while adding **new challenges**:

**Simplified:**
- ✅ No need to track two separate emulator instances
- ✅ No need for multi-game connection management
- ✅ Unified save file structure (though complex)
- ✅ Single ROM hash for detection

**New Challenges:**
- ⚠️ Need to detect which "mode" (OoT vs MM) the ROM is currently in
- ⚠️ Combined save file has both OoT and MM data structures
- ⚠️ RAM addresses differ depending on active game mode
- ⚠️ Memory layout changes dynamically during gameplay

**Revised Effort Estimate:** 600-900 hours (down from 800-1,200) - approximately 4-6 months

Sources:
- [OoTMM GitHub Repository](https://github.com/OoTMM/OoTMM)
- [OoTMM Official Website](https://ootmm.com/)
- [OoTMM Wiki](https://ootmm.wiki.gg/)

---

## Executive Summary

This document analyzes the requirements for extending OOTTracker to support [OoTMM](https://ootmm.com/) (Ocarina of Time + Majora's Mask combined randomizer), where items and locations from both games are shuffled together in a single ROM.

### Key Findings

- **Effort Estimate:** 600-900 hours (4-6 months with 1 developer)
- **Complexity:** High (reduced from Very High)
- **Feasibility:** Technically feasible, requires careful architecture
- **Recommendation:** Requires planning and phased implementation

### Major Challenges

1. **Game mode detection** - Detect if ROM is in OoT mode or MM mode
2. **Dual save structure parsing** - Parse unified save file with both games' data
3. **Item system unification** - Many MM items don't exist in OoT and vice versa
4. **UI complexity** - Need to track twice as many items/locations
5. **Dynamic memory addressing** - RAM layout changes based on active game
6. **Time mechanics** - MM's 3-day cycle has no OoT equivalent

---

## Table of Contents

1. [Background](#background)
2. [Game Comparison](#game-comparison)
3. [Technical Requirements](#technical-requirements)
4. [Architecture Changes](#architecture-changes)
5. [Implementation Plan](#implementation-plan)
6. [Effort Estimation](#effort-estimation)
7. [Risks and Mitigations](#risks-and-mitigations)
8. [Alternatives](#alternatives)

---

## Background

### What is OoT/MM Combined Randomizer?

A combined randomizer shuffles items and locations across both Ocarina of Time and Majora's Mask. For example:
- The **Kokiri Sword** (OoT starting item) might be at **Clock Town Postbox** (MM location)
- **Deku Mask** (MM transformation) might be at **Kakariko Graveyard** (OoT location)
- Progression requires switching between games and tracking state across both

### Current State

**OOTTracker** is OoT-only:
- Tracks OoT items, dungeons, and locations
- Parses OoT save files and RAM dumps
- UI designed for OoT's item/dungeon structure
- No MM support whatsoever

### Use Cases

1. **Solo Play:** Player switches between OoT and MM as they find items
2. **Multiworld:** Team of players split across both games
3. **Race/Tournament:** Competitive play with both games
4. **Streaming:** Viewer experience tracking both games simultaneously

---

## Game Comparison

### Items

#### OoT-Exclusive Items (Examples)
- Kokiri Sword, Master Sword, Biggoron Sword
- Deku Shield, Hylian Shield, Mirror Shield
- Fairy Ocarina, Ocarina of Time
- Hookshot, Longshot
- Boomerang
- Silver/Golden Gauntlets, Goron Bracelet
- Iron Boots, Hover Boots
- Zelda's Lullaby, Epona's Song, Song of Storms

#### MM-Exclusive Items (Examples)
- Great Fairy Sword
- Hero's Shield
- Deku, Goron, Zora, Fierce Deity Masks (transformations)
- Kamaro's Mask, Circus Leader's Mask, etc. (24+ masks)
- Razor Sword, Gilded Sword
- Oath to Order
- Elegy of Emptiness, Sonata of Awakening
- Powder Keg, Pictobox, Lens of Truth (different from OoT)

#### Shared Items (May need differentiation)
- Bow (both games)
- Bombs, Bombchus (both games)
- Bottles (both games, different contents)
- Hookshot (both games, but different)
- Magic Beans (both games)

**Total Item Count:**
- OoT: ~90 unique trackable items
- MM: ~120 unique trackable items (many masks)
- Combined: ~200+ unique items (with some overlap)

### Dungeons

#### OoT Dungeons (9 main + 7 child dungeons)
- Child: Deku Tree, Dodongo's Cavern, Jabu-Jabu's Belly
- Adult: Forest Temple, Fire Temple, Water Temple, Shadow Temple, Spirit Temple
- Ganon's Castle

#### MM Dungeons (4 main)
- Woodfall Temple
- Snowhead Temple
- Great Bay Temple
- Stone Tower Temple

#### MM Regional Temples (Optional)
- Beneath the Well
- Ancient Castle of Ikana
- Secret Shrine (some randomizers include these)

**Total:** 9 OoT + 4-7 MM = 13-16 dungeons to track

### Collectibles

#### OoT
- 100 Gold Skulltulas
- 36 Heart Pieces
- Spiritual Stones (3), Medallions (6)
- Triforce pieces (in some randomizer modes)

#### MM
- Stray Fairies (15 per dungeon = 60 total)
- 52 Heart Pieces
- Masks (24+ including transformations)
- Remains (4 boss remains)

### Memory Layout

#### OoT Save Structure (SRAM)
- **Save size:** ~0x1450 bytes
- **Key offsets:**
  - 0x0000: Quest items (bitflags)
  - 0x0074: Inventory (24 slots)
  - 0x00A0: Dungeon items
  - 0x00D4: Scene flags
  - Documented in `crate/oottracker/src/save.rs`

#### MM Save Structure (SRAM)
- **Save size:** ~0x4000 bytes (much larger!)
- **Key differences:**
  - Owl save system (temporary saves)
  - Bomber's Notebook (NPC tracking)
  - 3-day cycle state
  - Transformation masks stored differently
  - Stray fairy tracking per dungeon
  - Event flags structured differently

**Challenge:** MM save format is incompatible with OoT parser

### RAM Layout

#### OoT RAM
- **Base address:** 0x11A5D0 (save data mirror)
- **Scene flags:** 0x00D4 offset
- **Input register:** 0x1C84B4
- Defined in `crate/oottracker/src/ram.rs:48-61`

#### MM RAM
- **Base address:** Different (varies by emulator)
- **Scene structure:** Completely different
- **Actor system:** Different IDs
- **Clock town segments:** Multiple scenes

**Challenge:** Requires entirely separate RAM parser

---

## Technical Requirements

### 1. OoTMM ROM Detection & Game Mode Detection

**Requirement:** Detect OoTMM ROM and determine which game mode (OoT or MM) is currently active

#### Part A: ROM Detection (Simpler than expected!)

Since OoTMM creates a single combined ROM, we only need to detect ONE ROM type:

```rust
pub enum GameVersion {
    OcarinaOfTime { version: OotVersion },
    MajorasMask { version: MmVersion },
    OoTMM { version: OoTMMVersion }, // NEW: Single combined ROM
}

pub fn detect_game(rom_hash: &[u8]) -> Option<GameVersion> {
    match &rom_hash[..16] {
        OOT_USA_HASH => Some(GameVersion::OcarinaOfTime {
            version: OotVersion::UsaV10
        }),
        MM_USA_HASH => Some(GameVersion::MajorasMask {
            version: MmVersion::UsaV10
        }),
        OOTMM_HASH => Some(GameVersion::OoTMM {
            version: OoTMMVersion::detect_from_rom(rom_hash)
        }),
        _ => None,
    }
}
```

#### Part B: Game Mode Detection (NEW requirement!)

**Challenge:** Once we know it's an OoTMM ROM, we need to detect which game is currently active.

**Approaches:**

**Option 1: Scene Detection**
```rust
pub enum OoTMMGameMode {
    OcarinaOfTime,
    MajorasMask,
}

pub fn detect_active_game(ram: &[u8]) -> OoTMMGameMode {
    // Check current scene ID
    let scene_id = read_u8(&ram, SCENE_ID_OFFSET);

    match scene_id {
        // OoT scene IDs (0x00-0x65 typically)
        0x00..=0x65 => OoTMMGameMode::OcarinaOfTime,
        // MM scene IDs (higher range or specific offset)
        _ => OoTMMGameMode::MajorasMask,
    }
}
```

**Option 2: Memory Region Detection**
- Check if active memory region is OoT section or MM section
- MM data starts at 0x2000000 offset in ROM
- RAM addresses will differ based on which game is loaded

**Option 3: Game State Variable**
- OoTMM likely has a custom variable tracking active game
- Need to reverse engineer or consult OoTMM developers
- Most reliable but requires finding the variable

**Recommendation:** Option 3 (game state variable) with Option 1 (scene detection) as fallback

### 2. Combined Save File Parser

**Requirement:** Parse unified OoTMM save file containing both games' data

**Key Insight:** OoTMM uses a **single save file** that stores data for both games. When the player switches between games, both saves are preserved.

**New Modules Needed:**

```
crate/oottracker/src/
├── oot/
│   ├── save.rs     # OoT save parser (refactored)
│   └── ram.rs      # OoT RAM parser (refactored)
├── mm/
│   ├── save.rs     # NEW: MM save parser
│   └── ram.rs      # NEW: MM RAM parser
└── ootmm/
    ├── save.rs     # NEW: Combined save parser
    └── ram.rs      # NEW: Combined RAM parser
```

**Unified Save Structure:**

```rust
pub struct OoTMMSave {
    pub active_game: OoTMMGameMode,

    // OoT save data
    pub oot: OotSave,

    // MM save data
    pub mm: MmSave,

    // OoTMM-specific metadata
    pub crossover_count: u32,  // How many times player switched games
    pub last_crossover_scene: u8,
}

pub struct OotSave {
    // Existing OoT save structure from save.rs
    pub inventory: Inventory,
    pub quest_status: QuestStatus,
    pub dungeon_items: [DungeonItems; 9],
    // ...
}

pub struct MmSave {
    pub masks: MaskInventory,
    pub transformation: Option<Transformation>,
    pub time_of_day: MmTime, // 3-day cycle
    pub bomber_notebook: BomberNotebook,
    pub stray_fairies: [u8; 4], // Per dungeon
    pub remains: [bool; 4],     // Boss remains
    pub owl_statues: u32,       // Activated owl statues (bitfield)
    // ... MM-specific fields
}

impl Protocol for OoTMMSave {
    async fn read<'a, R: Read + Unpin>(read: &'a mut R) -> Result<Self, ReadError> {
        // Read combined save structure
        // OoTMM save format layout TBD - need to research or reverse engineer
        let active_game = read.read_u8().await?;
        let oot = OotSave::read(read).await?;
        let mm = MmSave::read(read).await?;

        Ok(Self {
            active_game: active_game.into(),
            oot,
            mm,
            crossover_count: read.read_u32().await?,
            last_crossover_scene: read.read_u8().await?,
        })
    }
}
```

**Trait Abstraction (optional but recommended):**

```rust
pub trait GameSave: Protocol {
    type Inventory;
    type DungeonItems;
    type QuestStatus;

    fn game_mode(&self) -> GameMode;
    fn inventory(&self) -> &Self::Inventory;
    fn dungeon_items(&self, dungeon: DungeonId) -> Self::DungeonItems;
}

impl GameSave for OotSave { /* ... */ }
impl GameSave for MmSave { /* ... */ }
```

**Challenges:**
- OoTMM save format is not yet documented
- Need to reverse engineer save structure or consult OoTMM developers
- Save file may have additional metadata for cross-game state

**Effort:** 3-4 weeks (2 weeks for MM parser + 1 week for OoTMM integration + 1 week for testing)

### 3. Unified Item Model

**Requirement:** Represent items from both games in a unified system

**Current State:**
```rust
// In save.rs and item_ids.rs
pub enum Item {
    DekuStick = 0x00,
    DekuNut = 0x01,
    Bombs = 0x02,
    // ... OoT items only
}
```

**New Design:**

```rust
pub enum Item {
    // Universal items
    Sword(SwordType),
    Shield(ShieldType),
    Bow,
    Bombs,
    // ... shared items

    // OoT-specific
    Oot(OotItem),

    // MM-specific
    Mm(MmItem),
}

pub enum OotItem {
    KokiriSword,
    Boomerang,
    IronBoots,
    HoverBoots,
    // ...
}

pub enum MmItem {
    DekuMask,
    GoronMask,
    ZoraMask,
    FierceDeityMask,
    GreatFairySword,
    PowderKeg,
    Pictograph,
    // ... 24+ masks
}
```

**Item Categories:**
```rust
pub enum ItemCategory {
    Equipment,
    Transformation, // MM-specific
    Mask,           // MM-specific
    Consumable,
    Quest,
    Dungeon,
    Song,
}
```

**Effort:** 1-2 weeks

### 4. Dungeon Model

**Current State:**
```rust
pub enum Dungeon {
    DekuTree,
    DodongosCavern,
    // ... OoT dungeons only
}
```

**New Design:**

```rust
pub enum Dungeon {
    Oot(OotDungeon),
    Mm(MmDungeon),
}

pub enum OotDungeon {
    DekuTree,
    DodongosCavern,
    JabuJabu,
    ForestTemple,
    FireTemple,
    WaterTemple,
    ShadowTemple,
    SpiritTemple,
    GanonsCastle,
}

pub enum MmDungeon {
    WoodfallTemple,
    SnowheadTemple,
    GreatBayTemple,
    StoneTowerTemple,
    // Optional:
    BeneathTheWell,
    AncientCastleOfIkana,
    SecretShrine,
}

pub struct DungeonState {
    pub dungeon: Dungeon,
    pub small_keys: u8,
    pub boss_key: bool,
    pub map: bool,
    pub compass: bool,

    // MM-specific
    pub stray_fairies: Option<u8>, // Only for MM dungeons
    pub remains: Option<bool>,     // Boss remains
}
```

**Effort:** 1 week

### 5. Check System Extension

**Requirement:** Validate locations from both games

**Current State:**
- `checks.rs` has 732-line function for OoT locations only
- Hardcoded string matching

**New Design:**

```rust
pub enum Check<R: Rando> {
    Oot(OotCheck<R>),
    Mm(MmCheck<R>),
}

pub enum OotCheck<R: Rando> {
    Event(String),
    Location(String),
    Exit { from: String, to: String },
    // ... existing OoT checks
}

pub enum MmCheck<R: Rando> {
    Event(String),
    Location(String),
    Mask(MaskType),
    StrayFairy { dungeon: MmDungeon, id: u8 },
    BomberNotebook(NpcId),
    // ... MM-specific checks
}

pub trait CheckExt {
    fn checked(&self, model: &CombinedModelState) -> Result<Option<bool>, CheckError>;
}

impl<R: Rando> CheckExt for Check<R> {
    fn checked(&self, model: &CombinedModelState) -> Result<Option<bool>, CheckError> {
        match self {
            Check::Oot(oot_check) => oot_check.checked(&model.oot),
            Check::Mm(mm_check) => mm_check.checked(&model.mm),
        }
    }
}
```

**Complexity:**
- OoT: 400+ location checks (existing)
- MM: 300+ location checks (new)
- Total: 700+ checks to implement and test

**Effort:** 4-6 weeks

### 6. Knowledge System

**Current State:**
- `knowledge.rs` tracks OoT settings, tricks, dungeon rewards

**New Requirements:**

```rust
pub struct Knowledge {
    pub game_mode: CombinedGameMode,

    // OoT knowledge
    pub oot: OotKnowledge,

    // MM knowledge
    pub mm: MmKnowledge,

    // Cross-game knowledge
    pub cross_game_entrance_shuffle: bool,
}

pub struct MmKnowledge {
    pub transformation_masks: HashSet<Transformation>,
    pub remains_locations: HashMap<Remain, RemainLocation>,
    pub stray_fairy_requirements: HashMap<MmDungeon, Requirements>,
    pub bomber_notebook_state: BomberNotebook,
    pub time_of_day: MmTime,
    pub owl_statues_activated: HashSet<OwlStatue>,
}
```

**Effort:** 2-3 weeks

### 7. UI Redesign

**Current UI Structure:**
- Single grid of OoT items
- OoT dungeons
- OoT songs

**New UI Requirements:**

#### Option A: Side-by-Side Layout
```
┌─────────────────┬─────────────────┐
│  OoT Items      │  MM Items       │
│  ┌───┬───┬───┐  │  ┌───┬───┬───┐  │
│  │ ⚔ │ 🛡 │ 🏹 │  │  │ 👺│ 🗿│ 🐟│  │
│  └───┴───┴───┘  │  └───┴───┴───┘  │
│                 │                 │
│  OoT Dungeons   │  MM Dungeons    │
│  ┌───┬───┬───┐  │  ┌───┬───┬───┐  │
│  │ 🌳│ 🗻│ 🐠│  │  │ 🍃│ ❄️ │ 🌊│  │
│  └───┴───┴───┘  │  └───┴───┴───┘  │
└─────────────────┴─────────────────┘
```

#### Option B: Tabbed Layout
```
┌─────────────────────────────────────┐
│ [ OoT ] [ MM ] [ Cross-Game ]      │
├─────────────────────────────────────┤
│  Items                              │
│  ┌───┬───┬───┬───┐                  │
│  │ ⚔ │ 🛡 │ 🏹 │ 💣│                  │
│  └───┴───┴───┴───┘                  │
│                                     │
│  Dungeons                           │
│  ┌───┬───┬───┐                      │
│  │ 🌳│ 🗻│ 🐠│                      │
│  └───┴───┴───┘                      │
└─────────────────────────────────────┘
```

#### Option C: Compact Unified Layout
```
┌─────────────────────────────────────┐
│  Swords: [OoT: ⚔️  | MM: ⚔️  ]        │
│  Shields: [OoT: 🛡 | MM: 🛡 ]        │
│  Transformations: [🧒|👺|🗿|🐟|👹]    │
│  Dungeons: [OoT: 9/9 | MM: 4/4]     │
└─────────────────────────────────────┘
```

**UI Challenges:**
- Screen space: 2x items to display
- Visual clarity: Distinguish OoT vs MM items
- User interaction: Click to toggle which game's item
- Responsive design: Mobile/small screens

**Effort:** 3-4 weeks

### 8. Auto-Tracking

**Current State:**
- BizHawk plugin reads OoT memory
- Single game instance

**New Requirements:**

#### Scenario 1: Sequential Play (Easier)
- Player switches between OoT and MM
- Tracker detects game switch
- Loads appropriate memory parser

#### Scenario 2: Simultaneous Multi-Game (Harder)
- Two emulator instances running
- Tracker monitors both simultaneously
- Requires multi-connection support

**Implementation:**

```rust
pub struct MultiGameTracker {
    pub oot_connection: Option<Connection>,
    pub mm_connection: Option<Connection>,
}

impl MultiGameTracker {
    pub async fn update(&mut self) {
        // Poll OoT emulator
        if let Some(oot_conn) = &self.oot_connection {
            if let Ok(ram) = oot_conn.read_ram().await {
                self.state.oot.ram = parse_oot_ram(&ram);
            }
        }

        // Poll MM emulator
        if let Some(mm_conn) = &self.mm_connection {
            if let Ok(ram) = mm_conn.read_ram().await {
                self.state.mm.ram = parse_mm_ram(&ram);
            }
        }
    }
}
```

**BizHawk Plugin Changes:**
- Detect ROM (OoT vs MM)
- Send game identifier with memory dumps
- Support multiple plugin instances

**Effort:** 2-3 weeks

---

## Architecture Changes

### Current Architecture

```
┌─────────────────────────────────────┐
│         ModelState                  │
│  ┌────────────────────────────────┐ │
│  │ Knowledge (OoT)                │ │
│  │ TrackerCtx (OoT UI config)     │ │
│  │ Ram (OoT memory)               │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### Proposed Architecture

```
┌──────────────────────────────────────────────┐
│        CombinedModelState                    │
│  ┌────────────────────────────────────────┐  │
│  │ game_mode: CombinedGameMode            │  │
│  │                                        │  │
│  │ oot: OotState {                        │  │
│  │   knowledge: OotKnowledge              │  │
│  │   ram: OotRam                          │  │
│  │ }                                      │  │
│  │                                        │  │
│  │ mm: MmState {                          │  │
│  │   knowledge: MmKnowledge               │  │
│  │   ram: MmRam                           │  │
│  │ }                                      │  │
│  │                                        │  │
│  │ cross_game: CrossGameState {           │  │
│  │   entrance_shuffle: bool               │  │
│  │   item_pool: CombinedItemPool          │  │
│  │ }                                      │  │
│  │                                        │  │
│  │ tracker_ctx: CombinedTrackerCtx        │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

### Module Structure Changes

**Current:**
```
crate/oottracker/src/
├── lib.rs          # ModelState
├── save.rs         # OoT save parsing
├── ram.rs          # OoT RAM parsing
├── checks.rs       # OoT checks
├── knowledge.rs    # OoT knowledge
├── ui.rs           # OoT UI
└── ...
```

**Proposed:**
```
crate/oottracker/src/
├── lib.rs              # CombinedModelState
├── game.rs             # Game detection, GameVersion enum
│
├── oot/                # OoT-specific modules
│   ├── mod.rs
│   ├── save.rs
│   ├── ram.rs
│   ├── checks.rs
│   ├── knowledge.rs
│   └── items.rs
│
├── mm/                 # MM-specific modules
│   ├── mod.rs
│   ├── save.rs         # NEW
│   ├── ram.rs          # NEW
│   ├── checks.rs       # NEW
│   ├── knowledge.rs    # NEW
│   ├── items.rs        # NEW
│   └── masks.rs        # NEW
│
├── combined/           # Cross-game logic
│   ├── mod.rs
│   ├── checks.rs       # Unified check system
│   └── knowledge.rs    # Cross-game knowledge
│
├── ui/                 # Refactored UI
│   ├── mod.rs
│   ├── oot.rs
│   ├── mm.rs
│   └── combined.rs
│
└── traits/             # Abstractions
    ├── save.rs         # SaveData trait
    ├── ram.rs          # RamData trait
    └── checks.rs       # CheckExt trait
```

**Total New Files:** ~25+ files
**Lines of Code Estimate:** ~8,000-12,000 new LOC

---

## Implementation Plan

### Phase 0: Planning & Prototyping (2-3 weeks)

**Goals:**
- Finalize architecture design
- Create proof-of-concept MM memory parser
- Validate feasibility with small prototype

**Deliverables:**
1. Architecture document (this doc + refinements)
2. Prototype MM save parser (100 lines)
3. Prototype dual-game ModelState
4. UI mockups

**Tasks:**
- [ ] Research MM randomizer codebase
- [ ] Document MM memory layout
- [ ] Create MM save file fixtures
- [ ] Design unified item model
- [ ] Sketch UI layouts
- [ ] Get community feedback

**Risks:**
- MM memory layout may be undocumented
- Combined randomizer may not be stable yet

### Phase 1: MM Memory Parsing (4-6 weeks)

**Goals:**
- Implement MM save parser
- Implement MM RAM parser
- Test with real MM save files

**Deliverables:**
1. `crate/oottracker/src/mm/save.rs` (~1,500 LOC)
2. `crate/oottracker/src/mm/ram.rs` (~400 LOC)
3. `crate/oottracker/src/mm/items.rs` (~300 LOC)
4. Unit tests for MM parsing (~800 LOC)

**Tasks:**
- [ ] Implement MmSave struct and Protocol
- [ ] Implement MmRam struct
- [ ] Parse masks (24+ types)
- [ ] Parse transformation state
- [ ] Parse Bomber's Notebook
- [ ] Parse stray fairies
- [ ] Parse 3-day cycle state
- [ ] Create binary fixtures
- [ ] Write 50+ unit tests

**Dependencies:**
- MM memory layout documentation
- MM save file samples from emulator

**Risks:**
- MM save format may vary by version
- Owl save system adds complexity

### Phase 2: Unified Item Model (3-4 weeks)

**Goals:**
- Create unified item representation
- Migrate OoT code to new model
- Implement MM item types

**Deliverables:**
1. `crate/oottracker/src/items.rs` (~500 LOC)
2. `crate/oottracker/src/mm/items.rs` (~400 LOC)
3. Updated OoT code (~200 LOC changed)
4. Tests (~400 LOC)

**Tasks:**
- [ ] Design Item enum hierarchy
- [ ] Implement shared item types
- [ ] Implement OoT-specific items
- [ ] Implement MM-specific items (masks!)
- [ ] Migrate existing OoT code
- [ ] Add item category system
- [ ] Write tests for all item types

**Dependencies:**
- Phase 1 complete

**Risks:**
- Breaking changes to existing code
- Need to maintain backward compatibility

### Phase 3: Check System Extension (4-6 weeks)

**Goals:**
- Implement MM location checks
- Integrate with existing OoT checks
- Create unified check interface

**Deliverables:**
1. `crate/oottracker/src/mm/checks.rs` (~800 LOC)
2. `crate/oottracker/src/combined/checks.rs` (~300 LOC)
3. Updated `checks.rs` (~200 LOC changed)
4. Table-driven tests (~1,000 LOC)

**Tasks:**
- [ ] Research MM location list
- [ ] Implement 300+ MM location checks
- [ ] Add mask checks
- [ ] Add stray fairy checks
- [ ] Add Bomber's Notebook checks
- [ ] Integrate with OoT checks
- [ ] Create check validation tests
- [ ] Add error handling

**Dependencies:**
- Phase 1 & 2 complete

**Risks:**
- MM location list may be incomplete
- Check validation complexity explodes

### Phase 4: Knowledge System (3-4 weeks)

**Goals:**
- Implement MM knowledge tracking
- Add cross-game knowledge
- Update knowledge inference

**Deliverables:**
1. `crate/oottracker/src/mm/knowledge.rs` (~600 LOC)
2. `crate/oottracker/src/combined/knowledge.rs` (~400 LOC)
3. Updated knowledge.rs (~300 LOC changed)
4. Tests (~500 LOC)

**Tasks:**
- [ ] Implement MmKnowledge struct
- [ ] Add transformation mask tracking
- [ ] Add remains location tracking
- [ ] Add stray fairy requirements
- [ ] Implement knowledge merge (cross-game)
- [ ] Add text box parsing for MM
- [ ] Write knowledge inference tests

**Dependencies:**
- Phase 2 & 3 complete

**Risks:**
- MM knowledge extraction may be complex
- Cross-game knowledge may have contradictions

### Phase 5: UI Redesign (4-5 weeks)

**Goals:**
- Redesign UI for dual games
- Implement new layouts
- Support both desktop and web

**Deliverables:**
1. `crate/oottracker/src/ui/combined.rs` (~1,500 LOC)
2. `crate/oottracker/src/ui/mm.rs` (~800 LOC)
3. Updated `ui.rs` (~500 LOC changed)
4. Web UI templates (~1,000 LOC)
5. CSS styling (~500 LOC)

**Tasks:**
- [ ] Design side-by-side layout
- [ ] Design tabbed layout
- [ ] Implement OoT UI (refactor existing)
- [ ] Implement MM UI (new)
- [ ] Implement combined view
- [ ] Add game switcher
- [ ] Update web templates
- [ ] Add responsive design
- [ ] Test on multiple screen sizes

**Dependencies:**
- Phase 1-4 complete

**Risks:**
- UI complexity may hurt UX
- Screen space limitations
- Performance with 2x items

### Phase 6: Auto-Tracking (3-4 weeks)

**Goals:**
- Support MM auto-tracking
- Detect game switches
- Support multi-game tracking

**Deliverables:**
1. `crate/oottracker-bizhawk/` updates (~500 LOC)
2. Game detection logic (~200 LOC)
3. Multi-connection support (~400 LOC)
4. Tests (~300 LOC)

**Tasks:**
- [ ] Update BizHawk plugin for MM
- [ ] Add ROM hash detection
- [ ] Implement MM memory reading
- [ ] Support game switching
- [ ] Test with both emulators
- [ ] Add multi-connection support (optional)

**Dependencies:**
- Phase 1-5 complete

**Risks:**
- BizHawk MM support may be limited
- Game detection may fail
- Multi-game tracking very complex

### Phase 7: Testing & Polish (3-4 weeks)

**Goals:**
- End-to-end testing
- Bug fixes
- Documentation
- Performance optimization

**Deliverables:**
1. Integration tests (~1,500 LOC)
2. Bug fixes (~500 LOC)
3. Documentation (~2,000 words)
4. Performance improvements

**Tasks:**
- [ ] End-to-end test with real randomizer
- [ ] Test game switching
- [ ] Test all MM locations
- [ ] Fix bugs
- [ ] Optimize rendering performance
- [ ] Write user documentation
- [ ] Create video tutorial
- [ ] Get community feedback

**Dependencies:**
- All phases complete

**Risks:**
- May uncover fundamental issues
- Community may have feature requests

---

## Effort Estimation

### By Phase

| Phase | Duration | LOC | Complexity | Risk |
|-------|----------|-----|------------|------|
| **0. Planning** | 2-3 weeks | ~100 | Low | Low |
| **1. MM Parsing** | 4-6 weeks | ~2,700 | High | Medium |
| **2. Item Model** | 3-4 weeks | ~1,500 | Medium | Medium |
| **3. Check System** | 4-6 weeks | ~2,300 | Very High | High |
| **4. Knowledge** | 3-4 weeks | ~1,800 | High | Medium |
| **5. UI Redesign** | 4-5 weeks | ~3,800 | High | Medium |
| **6. Auto-Tracking** | 3-4 weeks | ~1,400 | Medium | Medium |
| **7. Testing** | 3-4 weeks | ~2,000 | Medium | High |
| **TOTAL** | **26-36 weeks** | **~15,600** | **Very High** | **High** |

### By Developer

**1 Senior Developer (Full-time):**
- Timeline: 26-36 weeks (6-9 months)
- Assumes: Strong Rust, OoT/MM knowledge, emulator experience

**2 Developers (Full-time):**
- Timeline: 16-20 weeks (4-5 months)
- Requires: Good coordination, clear architecture

**1 Developer (Part-time 50%):**
- Timeline: 52-72 weeks (12-18 months)

### Cost Estimate (Developer Time)

**Hourly Breakdown:**
- Planning: 80-120 hours
- Implementation: 600-800 hours
- Testing: 120-160 hours
- **Total: 800-1,080 hours**

**At $100/hour:** $80,000-$108,000
**At $50/hour:** $40,000-$54,000

### Lines of Code Estimate

| Component | New LOC | Modified LOC | Total |
|-----------|---------|--------------|-------|
| MM memory parsing | 2,700 | 0 | 2,700 |
| Item model | 1,500 | 200 | 1,700 |
| Check system | 2,300 | 200 | 2,500 |
| Knowledge system | 1,800 | 300 | 2,100 |
| UI | 3,800 | 500 | 4,300 |
| Auto-tracking | 1,400 | 200 | 1,600 |
| Tests | 2,000 | 0 | 2,000 |
| **TOTAL** | **~15,500** | **~1,400** | **~16,900** |

**Code Growth:**
- Current codebase: ~15,700 LOC
- After MM integration: ~32,600 LOC
- **Growth: +107%**

---

## Risks and Mitigations

### Technical Risks

#### Risk 1: MM Memory Layout Undocumented
**Severity:** High
**Impact:** Phase 1 blocked

**Mitigation:**
- Reverse engineer MM memory with debugger
- Contact MM randomizer developers
- Use existing MM trackers as reference
- Budget extra 2-3 weeks for research

#### Risk 2: Combined Randomizer Unstable
**Severity:** Medium
**Impact:** Requirements change frequently

**Mitigation:**
- Work closely with randomizer developers
- Design flexible architecture
- Use feature flags for experimental features
- Plan for future changes

#### Risk 3: UI Complexity Overwhelming
**Severity:** Medium
**Impact:** Poor user experience

**Mitigation:**
- Create multiple UI layouts (let users choose)
- User testing before finalizing
- Provide tutorial/documentation
- Add "simple mode" with fewer items shown

#### Risk 4: Performance Degradation
**Severity:** Medium
**Impact:** Slow rendering, laggy UI

**Mitigation:**
- Profile performance early
- Optimize rendering pipeline
- Use lazy loading for off-screen items
- Consider web worker for heavy computation

#### Risk 5: Testing Complexity Explodes
**Severity:** High
**Impact:** Can't validate correctness

**Mitigation:**
- Write tests incrementally
- Use table-driven tests for locations
- Create comprehensive fixtures
- Automated regression testing

### Business Risks

#### Risk 1: Low Adoption
**Severity:** Medium
**Impact:** Effort not justified

**Mitigation:**
- Survey community interest first
- Gradual rollout (beta testing)
- Maintain OoT-only mode
- Clear documentation/tutorials

#### Risk 2: Maintenance Burden
**Severity:** High
**Impact:** Codebase becomes unmaintainable

**Mitigation:**
- Strong architecture from start
- Comprehensive documentation
- Modular design (easy to update one game)
- Recruit additional maintainers

#### Risk 3: Competing Implementations
**Severity:** Low
**Impact:** Duplicated effort

**Mitigation:**
- Announce plans early
- Collaborate with other developers
- Open source development
- Focus on unique features

---

## Alternatives

### Alternative 1: Separate MM Tracker

**Approach:** Build standalone MM tracker, no OoT integration

**Pros:**
- Simpler architecture
- No risk to existing OoT functionality
- Faster development (3-4 months)
- Independent deployment

**Cons:**
- Doesn't solve combined randomizer use case
- Duplicated code/logic
- Users need two tools
- No cross-game features

**Effort:** ~400-500 hours (vs 800-1,080 for integrated)

**Recommendation:** Good for MM-only randomizers, doesn't solve combined use case

### Alternative 2: Plugin Architecture

**Approach:** Refactor OOTTracker to support game plugins

```
oottracker (core)
├── oot-plugin
└── mm-plugin (community-developed)
```

**Pros:**
- Extensible to future games (Twilight Princess?)
- Community can contribute
- Core remains simple
- Easy to maintain

**Cons:**
- Requires major refactoring
- Plugin API design is complex
- Still need to solve UI for multiple games
- Performance overhead

**Effort:** ~600-800 hours (refactoring + plugin system + MM plugin)

**Recommendation:** Best long-term solution, but requires upfront refactoring

### Alternative 3: External State Synchronization

**Approach:** Two separate trackers that sync state via network

```
OOTTracker (OoT) <--WebSocket--> MMTracker (MM)
          ↓                              ↓
    Combined Web View
```

**Pros:**
- No changes to existing OOTTracker
- Separate MM tracker can be simpler
- Failure in one doesn't affect other
- Easier testing

**Cons:**
- Network dependency
- Synchronization complexity
- Still need combined UI
- Two processes to manage

**Effort:** ~500-600 hours (MM tracker + sync layer + combined UI)

**Recommendation:** Simpler than integrated, but less elegant

### Alternative 4: Wait for Community Implementation

**Approach:** Do nothing, let community develop MM support

**Pros:**
- No effort required
- Community ownership
- Multiple approaches can compete

**Cons:**
- May never happen
- Quality may be lower
- Fragmented ecosystem
- No control over timeline

**Effort:** 0 hours

**Recommendation:** Only if combined randomizers remain niche

---

## Recommended Approach

### Phase 1: Validate Demand (1 week)

1. **Survey community:**
   - How many use combined randomizers?
   - What features are most important?
   - Would they use integrated tracker?

2. **Assess competing tools:**
   - Does another MM tracker exist?
   - What's their approach?
   - Can we collaborate?

3. **Evaluate alternatives:**
   - Is plugin architecture worth the investment?
   - Should we start with separate tracker?

### Phase 2: Proof of Concept (4 weeks)

1. **Implement basic MM parsing** (2 weeks)
2. **Create combined UI mockup** (1 week)
3. **Test with beta users** (1 week)

**Decision Point:** Continue or pivot?

### Phase 3: Full Implementation (20-30 weeks)

If proof of concept succeeds:
- Follow implementation plan above
- Phased rollout
- Beta testing throughout

### Recommended Variant: Plugin Architecture + Integrated MM

**Best of both worlds:**
1. Refactor OOTTracker for plugins (8-10 weeks)
2. Implement OoT as plugin (2 weeks migration)
3. Implement MM as plugin (10-12 weeks)
4. Implement combined UI (4-5 weeks)

**Total: 24-29 weeks (similar to integrated approach)**

**Benefits:**
- Future-proof architecture
- Community can add games
- Cleaner separation of concerns
- Easier testing

**Tradeoffs:**
- More upfront work
- Compatibility concerns
- Plugin API design complexity

---

## Conclusion

### Summary

Integrating Majora's Mask support into OOTTracker for combined randomizers is:
- ✅ **Technically feasible**
- ⚠️ **Architecturally challenging**
- 💰 **Resource-intensive** (800-1,080 hours)
- 🎯 **High value** (if combined randomizers are popular)

### Key Decisions

1. **Architecture:** Plugin-based vs integrated?
2. **UI Design:** Side-by-side vs tabbed vs compact?
3. **Auto-tracking:** Sequential vs simultaneous?
4. **Scope:** Full MM support vs essential features only?

### Next Steps

1. **Validate demand** - Survey community
2. **Choose architecture** - Plugin vs integrated
3. **Build prototype** - Prove feasibility
4. **Plan implementation** - Detailed roadmap
5. **Recruit help** - Consider additional developers
6. **Phased rollout** - Beta test with community

### Final Recommendation

**Start with proof of concept:**
- 4 weeks
- Basic MM parsing
- UI mockup
- Beta test with 10-20 users

**If successful, proceed with plugin architecture:**
- More future-proof
- Community extensible
- Similar effort to direct integration

**If combined randomizers remain niche:**
- Alternative 1: Build separate MM tracker
- Alternative 4: Let community implement

---

**Document Status:** Planning / Analysis
**Approval Required:** Yes
**Community Feedback:** Recommended
**Est. Timeline:** 6-9 months for full implementation
**Est. Cost:** $40,000-$108,000 (developer time)

**Prepared by:** Claude (Anthropic)
**Date:** 2025-12-31
**Version:** 1.0
