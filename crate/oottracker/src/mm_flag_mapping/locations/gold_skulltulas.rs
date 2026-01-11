//! Gold Skulltula / Spider House location mappings.
//!
//! MM has two spider houses (Swamp and Oceanside) with gold skulltulas.
//! These are tracked differently from OoT's skulltulas.

use std::collections::HashMap;

use crate::mm_flag_mapping::MmFlagMapping;

/// Registers gold skulltula/spider house mappings into the provided map.
///
/// Note: Most spider house locations are tracked as scene collectible flags
/// within the respective spider house scenes. The reward items (masks/wallets)
/// are tracked via EventInf or WeekEventReg global flags.
///
/// Spider house chest locations are registered in mini_dungeons.rs.
pub fn register_gold_skulltulas(map: &mut HashMap<&'static str, MmFlagMapping>) {
    // Most spider house token locations are tracked per-scene as collectible flags.
    // The specific token mappings would be added here once researched.
    // For now, the spider house completion rewards are handled in items.rs.
    let _ = map; // Suppress unused warning
}
