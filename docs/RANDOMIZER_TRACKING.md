# OoTMM Randomizer Tracking

This document describes the randomizer location tracking system for OoT/MM combo randomizer seeds (OoTMM).

## Overview

The tracking system reads game memory in real-time to detect which randomizer checks have been collected. It evaluates logic expressions from OoTMM world data against the current game state to determine location accessibility.

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Emulator      │────▶│   oottracker     │────▶│  oottracker-web │
│  (BizHawk/PJ64) │     │   (core crate)   │     │   (web server)  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
        │                       │                        │
        │ Memory Read           │ Flag Mapping           │ WebSocket
        ▼                       ▼                        ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Game RAM       │     │     ootmm        │     │   Web Browser   │
│  (Save Context) │     │ (logic/settings) │     │   (Tracker UI)  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
```

## Key Components

### 1. Flag Mapping (`crate/oottracker/src/flag_mapping.rs`)

Maps randomizer location IDs to memory addresses in game RAM.

**Flag Types:**
- `Chest` - Opened chest flags (scene flags offset 0x00)
- `Collectible` - Freestanding items (scene flags offset 0x0C)
- `GoldSkulltula` - Skulltula token flags (0x0E9C)
- `EventChkInf` - Global event flags (0x0ED4)
- `ItemGetInf` - Item acquisition flags (0x0EF0)
- `InfTable` - NPC/misc flags (0x0EF8)
- `Shop` / `Scrub` - Purchase tracking

**Example Usage:**
```rust
use oottracker::flag_mapping::{get_mapping, get_all_oot_mappings};

// Get mapping for a specific location
if let Some(mapping) = get_mapping("OOT Deku Tree Map Chest") {
    println!("Scene: {:?}, Bit: {}", mapping.scene_id, mapping.bit);
}

// Iterate all mappings
for mapping in get_all_oot_mappings() {
    println!("{}: {:?}", mapping.location_id, mapping.flag_type);
}
```

### 2. Event System (`crate/ootmm/src/events/`)

Handles game event tracking for both OoT and MM.

**OoT Events (`events/oot.rs`):**
- Scene flags per dungeon/area
- Global event check flags
- Item get tracking

**MM Events (`events/mm.rs`, `events/mm_flags.rs`):**
- Stray fairy collection (62 fairies across 5 areas)
- Owl statue activation (10 statues)
- Chest and collectible flags
- Quest item status

### 3. Settings Schema (`crate/ootmm/src/settings.rs`)

Defines randomizer configuration that affects logic evaluation.

**Setting Categories:**

| Category | Examples |
|----------|----------|
| Logic Mode | `glitchless`, `glitched`, `noLogic` |
| Open Dungeons (OoT) | `DC`, `BotW`, `JJ`, `Shadow`, `Water`, `fireChild`, `wellAdult` |
| Open Dungeons (MM) | `ST`, `WF` |
| MQ Dungeons | All 12 OoT dungeons can be Master Quest |
| World State | `dekuTree`, `doorOfTime`, `kakarikoGate`, `ganonBossKey`, `lacs` |
| MM Settings | `majoraChild`, `moonCrash`, `bossWarpPads`, `shufflePotsMm` |
| Age/Time | `ageChange`, `agelessBoots`, `agelessHookshot`, `agelessStrength` |
| Entrance Rando | `erOverworld`, `erGrottos`, `erIndoorsMajor`, `erMoon` |
| Key Shuffle | `smallKeyShuffleOot` (`vanilla`, `dungeon`, `anywhere`) |

**Usage:**
```rust
use ootmm::settings::{RandomizerSettings, OotDungeon, MqDungeon};

let mut settings = RandomizerSettings::default();
settings.open_dungeons_oot.insert(OotDungeon::DodongosCavern);
settings.mq_dungeons.insert(MqDungeon::SpiritTemple);
settings.ageless_boots = true;
```

### 4. Expression Evaluator (`crate/ootmm/src/expr/`)

Evaluates OoTMM logic expressions against game state.

**Core Trait:**
```rust
pub trait EvalContext {
    fn has_item(&self, item: &str, count: u32) -> bool;
    fn event(&self, name: &str) -> bool;
    fn setting(&self, name: &str) -> Option<bool>;
    fn setting_value(&self, name: &str, value: &str) -> bool;
    fn trick(&self, name: &str) -> bool;
    fn is_adult(&self) -> bool;
    fn is_child(&self) -> bool;
    fn mm_time(&self) -> u32;
    // ... more methods
}
```

**Implementations:**
- `OotEvalContext` - OoT-specific evaluation with inventory/event state
- `MmEvalContext` - MM-specific evaluation with mask/time state

**Helper Macros (70+):**
```rust
// Item combination helpers
fn can_use_bow(&self) -> bool;
fn has_explosives(&self) -> bool;
fn can_hookshot(&self) -> bool;

// Combat helpers
fn can_kill_deku_baba(&self) -> bool;
fn can_stun_deku(&self) -> bool;

// Navigation helpers
fn can_ride_epona(&self) -> bool;
fn can_play_sun(&self) -> bool;
```

### 5. Web API (`crate/oottracker-web/src/http.rs`)

REST endpoint for checked locations.

**Endpoint:**
```
GET /api/room/{room_name}/checked-locations
```

**Response:**
```json
{
  "total_locations": 500,
  "checked_count": 127,
  "unchecked_count": 350,
  "unknown_count": 23,
  "locations": [
    {
      "location_id": "OOT Deku Tree Map Chest",
      "status": "checked"
    },
    {
      "location_id": "OOT Kokiri Sword Chest",
      "status": "unchecked"
    }
  ]
}
```

### 6. Frontend (`assets/web/static/`)

**Files:**
- `checked-locations.js` - Fetches and renders location status
- `checked-locations.css` - Styling for location indicators
- `settings.js` - Settings configuration UI
- `settings.css` - Settings page styling

## Memory Layout

### OoT Save Context (Base: 0x11A5D0)

| Offset | Size | Description |
|--------|------|-------------|
| 0x00D4 | 0x1C × 101 | Scene flags (per scene) |
| 0x0E9C | 0x18 | Gold Skulltula flags |
| 0x0ED4 | 0x1C | Event check flags (event_chk_inf) |
| 0x0EF0 | 0x08 | Item get flags (item_get_inf) |
| 0x0EF8 | 0x3C | Info table flags (inf_table) |

### Scene Flag Structure (0x1C bytes per scene)

| Offset | Type | Description |
|--------|------|-------------|
| 0x00 | u32 | Chest flags |
| 0x04 | u32 | Switch flags |
| 0x08 | u32 | Room clear flags |
| 0x0C | u32 | Collectible flags |
| 0x10 | u32 | Unused |
| 0x14 | u32 | Visited rooms |
| 0x18 | u32 | Visited floors |

### MM Save Context

| Offset | Size | Description |
|--------|------|-------------|
| 0x0000 | varies | Scene flags |
| 0x00F8 | 0x14 | Owl statue flags |
| 0x0100 | varies | Stray fairy counts |

## Settings UI

Access the settings configuration at `/settings` on the web tracker.

**Features:**
- Load settings from JSON file (OoTMM seed settings export)
- Save settings to file
- Copy as JSON
- Reset to defaults
- Real-time validation

**Supported Settings:**
- Logic mode selection
- Open dungeon checkboxes
- MQ dungeon selection
- World state dropdowns
- Ageless item toggles
- Entrance randomizer options
- Glitch settings
- Logic tricks (comma-separated)

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p ootmm
cargo test -p oottracker

# Run with logging
RUST_LOG=debug cargo test
```

### Adding New Locations

1. Add the location ID to `flag_mapping.rs`:
```rust
FlagMapping {
    location_id: "OOT New Location".to_string(),
    game: Game::Oot,
    scene_id: Some(SceneId::DekuTree),
    flag_type: FlagType::Chest,
    bit: 5,
}
```

2. Add event handling if needed in `events/oot.rs` or `events/mm.rs`

3. Update tests to cover the new location

### Adding New Settings

1. Add the setting to `settings.rs`:
```rust
pub struct RandomizerSettings {
    // ...
    pub new_setting: bool,
}
```

2. Implement parsing in `setting_value()` method

3. Add UI element in `settings.html`

4. Update `settings.js` to handle the new setting

## References

- [OoTMM Randomizer](https://ootmm.com/)
- [OoT Randomizer](https://ootrandomizer.com/)
- [CloudModding Wiki - OoT](https://wiki.cloudmodding.com/oot)
- [CloudModding Wiki - MM](https://wiki.cloudmodding.com/mm)
