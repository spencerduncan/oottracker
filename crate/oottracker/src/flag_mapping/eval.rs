//! OoT Evaluation Context for logic expressions.

use ootmm::expr::EvalContext;

use crate::save::{
    Bottle, Equipment, Hookshot, MagicCapacity, Ocarina, QuestItems, Save, Upgrades,
};

/// Context for evaluating OoT logic expressions from save data.
///
/// This struct provides an [`EvalContext`] implementation that reads item
/// possession, age, and other state from OoT save data.
pub struct OotEvalContext<'a> {
    save: &'a Save,
}

impl<'a> OotEvalContext<'a> {
    /// Creates a new evaluation context from save data.
    pub fn new(save: &'a Save) -> Self {
        Self { save }
    }
}

impl EvalContext for OotEvalContext<'_> {
    fn has_item(&self, item: &str, count: u32) -> bool {
        let item_upper = item.to_uppercase();
        let has = match item_upper.as_str() {
            // Inventory items
            "BOW" => self.save.inv.bow,
            "FIRE_ARROW" | "FIRE_ARROWS" => self.save.inv.fire_arrows,
            "ICE_ARROW" | "ICE_ARROWS" => self.save.inv.ice_arrows,
            "LIGHT_ARROW" | "LIGHT_ARROWS" => self.save.inv.light_arrows,
            "DINS_FIRE" | "DIN" => self.save.inv.dins_fire,
            "FARORES_WIND" | "FARORE" => self.save.inv.farores_wind,
            "NAYRUS_LOVE" | "NAYRU" => self.save.inv.nayrus_love,
            "SLINGSHOT" | "FAIRY_SLINGSHOT" => self.save.inv.slingshot,
            "OCARINA" | "OCARINA_OF_TIME" | "FAIRY_OCARINA" => {
                self.save.inv.ocarina != Ocarina::None
            }
            "BOMBCHU" | "BOMBCHUS" => self.save.inv.bombchus,
            "HOOKSHOT" => matches!(
                self.save.inv.hookshot,
                Hookshot::Hookshot | Hookshot::Longshot
            ),
            "LONGSHOT" => matches!(self.save.inv.hookshot, Hookshot::Longshot),
            "BOOMERANG" => self.save.inv.boomerang,
            "LENS" | "LENS_OF_TRUTH" => self.save.inv.lens,
            "MAGIC_BEAN" | "BEANS" => self.save.inv.beans,
            "HAMMER" | "MEGATON_HAMMER" => self.save.inv.hammer,

            // Equipment
            "KOKIRI_SWORD" => self.save.equipment.contains(Equipment::KOKIRI_SWORD),
            "MASTER_SWORD" => self.save.equipment.contains(Equipment::MASTER_SWORD),
            "GIANTS_KNIFE" | "BIGGORON_SWORD" | "SWORD_BIGGORON" => {
                self.save.biggoron_sword || self.save.equipment.contains(Equipment::GIANTS_KNIFE)
            }
            "DEKU_SHIELD" => self.save.equipment.contains(Equipment::DEKU_SHIELD),
            "HYLIAN_SHIELD" => self.save.equipment.contains(Equipment::HYLIAN_SHIELD),
            "MIRROR_SHIELD" => self.save.equipment.contains(Equipment::MIRROR_SHIELD),
            "GORON_TUNIC" => self.save.equipment.contains(Equipment::GORON_TUNIC),
            "ZORA_TUNIC" => self.save.equipment.contains(Equipment::ZORA_TUNIC),
            "IRON_BOOTS" => self.save.equipment.contains(Equipment::IRON_BOOTS),
            "HOVER_BOOTS" => self.save.equipment.contains(Equipment::HOVER_BOOTS),

            // Upgrades
            "GORON_BRACELET" | "STRENGTH" => self.save.upgrades.contains(Upgrades::GORON_BRACELET),
            "SILVER_GAUNTLETS" => self.save.upgrades.contains(Upgrades::SILVER_GAUNTLETS),
            "GOLD_GAUNTLETS" | "GOLDEN_GAUNTLETS" => {
                self.save.upgrades.contains(Upgrades::GOLD_GAUNTLETS)
            }
            "SILVER_SCALE" | "SCALE" => self.save.upgrades.contains(Upgrades::SILVER_SCALE),
            "GOLD_SCALE" | "GOLDEN_SCALE" => self.save.upgrades.contains(Upgrades::GOLD_SCALE),
            "BOMB_BAG" | "BOMBS" => self
                .save
                .upgrades
                .intersects(Upgrades::BOMB_BAG_20 | Upgrades::BOMB_BAG_30 | Upgrades::BOMB_BAG_40),
            "QUIVER" => self.save.upgrades.intersects(
                Upgrades::QUIVER_50 | Upgrades::QUIVER_40 | Upgrades::from_bits_truncate(0x1),
            ),
            "BULLET_BAG" => self.save.upgrades.intersects(
                Upgrades::BULLET_BAG_30 | Upgrades::BULLET_BAG_40 | Upgrades::BULLET_BAG_50,
            ),
            "WALLET" | "ADULTS_WALLET" | "GIANTS_WALLET" => {
                self.save.upgrades.intersects(Upgrades::WALLET_MASK)
            }

            // Magic
            "MAGIC" | "MAGIC_METER" => self.save.magic != MagicCapacity::None,
            "DOUBLE_MAGIC" => self.save.magic == MagicCapacity::Large,

            // Quest items - songs
            "ZELDAS_LULLABY" | "LULLABY" => {
                self.save.quest_items.contains(QuestItems::ZELDAS_LULLABY)
            }
            "EPONAS_SONG" | "EPONA" => self.save.quest_items.contains(QuestItems::EPONAS_SONG),
            "SARIAS_SONG" | "SARIA" => self.save.quest_items.contains(QuestItems::SARIAS_SONG),
            "SUNS_SONG" | "SUN" => self.save.quest_items.contains(QuestItems::SUNS_SONG),
            "SONG_OF_TIME" | "TIME" => self.save.quest_items.contains(QuestItems::SONG_OF_TIME),
            "SONG_OF_STORMS" | "STORMS" => {
                self.save.quest_items.contains(QuestItems::SONG_OF_STORMS)
            }
            "MINUET_OF_FOREST" | "MINUET" => {
                self.save.quest_items.contains(QuestItems::MINUET_OF_FOREST)
            }
            "BOLERO_OF_FIRE" | "BOLERO" => {
                self.save.quest_items.contains(QuestItems::BOLERO_OF_FIRE)
            }
            "SERENADE_OF_WATER" | "SERENADE" => self
                .save
                .quest_items
                .contains(QuestItems::SERENADE_OF_WATER),
            "REQUIEM_OF_SPIRIT" | "REQUIEM" => self
                .save
                .quest_items
                .contains(QuestItems::REQUIEM_OF_SPIRIT),
            "NOCTURNE_OF_SHADOW" | "NOCTURNE" => self
                .save
                .quest_items
                .contains(QuestItems::NOCTURNE_OF_SHADOW),
            "PRELUDE_OF_LIGHT" | "PRELUDE" => {
                self.save.quest_items.contains(QuestItems::PRELUDE_OF_LIGHT)
            }

            // Quest items - stones and medallions
            "KOKIRI_EMERALD" => self.save.quest_items.contains(QuestItems::KOKIRI_EMERALD),
            "GORON_RUBY" => self.save.quest_items.contains(QuestItems::GORON_RUBY),
            "ZORA_SAPPHIRE" => self.save.quest_items.contains(QuestItems::ZORA_SAPPHIRE),
            "FOREST_MEDALLION" => self.save.quest_items.contains(QuestItems::FOREST_MEDALLION),
            "FIRE_MEDALLION" => self.save.quest_items.contains(QuestItems::FIRE_MEDALLION),
            "WATER_MEDALLION" => self.save.quest_items.contains(QuestItems::WATER_MEDALLION),
            "SPIRIT_MEDALLION" => self.save.quest_items.contains(QuestItems::SPIRIT_MEDALLION),
            "SHADOW_MEDALLION" => self.save.quest_items.contains(QuestItems::SHADOW_MEDALLION),
            "LIGHT_MEDALLION" => self.save.quest_items.contains(QuestItems::LIGHT_MEDALLION),

            // Other quest items
            "GERUDO_CARD" | "GERUDO_MEMBERSHIP_CARD" => {
                self.save.quest_items.contains(QuestItems::GERUDO_CARD)
            }
            "STONE_OF_AGONY" => self.save.quest_items.contains(QuestItems::STONE_OF_AGONY),

            // Bottles (check if any bottle exists)
            "BOTTLE" | "EMPTY_BOTTLE" => self.save.inv.bottles.iter().any(|b| *b != Bottle::None),

            // Sticks/Nuts (check if capacity upgrade exists)
            "DEKU_STICK" | "STICKS" => self
                .save
                .upgrades
                .intersects(Upgrades::DEKU_STICK_CAPACITY_MASK),
            "DEKU_NUT" | "NUTS" => self
                .save
                .upgrades
                .intersects(Upgrades::DEKU_NUT_CAPACITY_MASK),

            // Default - unknown item
            _ => false,
        };

        // For count-based checks, we only support count=1 for now
        // Most logic just checks has(ITEM) which defaults to count=1
        has && count <= 1
    }

    fn event(&self, _name: &str) -> bool {
        // Events require more complex tracking (event_chk_inf flags)
        // For now, return false - events are harder to map without full flag analysis
        false
    }

    fn setting(&self, _name: &str) -> Option<bool> {
        // Settings are not stored in save data
        // Return None to indicate unknown
        None
    }

    fn setting_value(&self, _name: &str, _value: &str) -> bool {
        // Settings are not stored in save data
        false
    }

    fn trick(&self, _name: &str) -> bool {
        // Tricks are user preferences, not stored in save
        false
    }

    fn is_adult(&self) -> bool {
        self.save.is_adult
    }

    fn is_child(&self) -> bool {
        !self.save.is_adult
    }

    fn mm_time(&self) -> u32 {
        // OoT doesn't have MM time
        0
    }
}
