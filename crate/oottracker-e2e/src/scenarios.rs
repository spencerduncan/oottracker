//! Test scenarios for E2E testing.
//!
//! This module provides test scenarios that exercise the tracker's event detection
//! and state tracking capabilities. Each scenario represents a sequence of game
//! state changes that should trigger specific tracker events.

use std::time::Duration;

use crate::fixtures::{BossDefeats, Equipment, GameStateFixture, ItemId, ItemSlot, QuestStatus};

/// Types of events the tracker should detect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackerEvent {
    /// Item was collected.
    ItemCollected(String),
    /// Equipment was obtained.
    EquipmentObtained(String),
    /// Boss was defeated.
    BossDefeated(String),
    /// Dungeon reward obtained.
    DungeonReward(String),
    /// Age changed (child/adult).
    AgeChanged(bool), // true = adult
    /// Health changed.
    HealthChanged { current: u16, max: u16 },
    /// Magic obtained or changed.
    MagicChanged { current: u8, double: bool },
    /// Skulltula count changed.
    SkulltulaCountChanged(u8),
}

/// A step in a test scenario.
#[derive(Debug, Clone)]
pub struct ScenarioStep {
    /// Description of what happens in this step.
    pub description: &'static str,
    /// The game state after this step.
    pub state: GameStateFixture,
    /// Expected events that should be detected.
    pub expected_events: Vec<TrackerEvent>,
    /// Optional delay before checking for events (for timing-sensitive tests).
    pub delay: Option<Duration>,
}

impl ScenarioStep {
    /// Creates a new scenario step.
    pub fn new(description: &'static str, state: GameStateFixture) -> Self {
        Self {
            description,
            state,
            expected_events: Vec::new(),
            delay: None,
        }
    }

    /// Adds an expected event to this step.
    pub fn expect_event(mut self, event: TrackerEvent) -> Self {
        self.expected_events.push(event);
        self
    }

    /// Adds multiple expected events to this step.
    pub fn expect_events(mut self, events: Vec<TrackerEvent>) -> Self {
        self.expected_events.extend(events);
        self
    }

    /// Sets a delay before checking for events.
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
}

/// A test scenario consisting of multiple steps.
#[derive(Debug, Clone)]
pub struct TestScenario {
    /// Unique identifier for the scenario.
    pub id: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Steps in the scenario.
    pub steps: Vec<ScenarioStep>,
    /// Total expected duration for the scenario.
    pub timeout: Duration,
}

impl TestScenario {
    /// Creates a new test scenario.
    pub fn new(id: &'static str, description: &'static str) -> Self {
        Self {
            id,
            description,
            steps: Vec::new(),
            timeout: Duration::from_secs(60),
        }
    }

    /// Adds a step to the scenario.
    pub fn step(mut self, step: ScenarioStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Sets the scenario timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Returns the number of steps in the scenario.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Returns all expected events across all steps.
    pub fn all_expected_events(&self) -> Vec<&TrackerEvent> {
        self.steps
            .iter()
            .flat_map(|s| s.expected_events.iter())
            .collect()
    }
}

// ============================================================================
// Pre-defined Scenarios
// ============================================================================

/// Scenario: Player collects Kokiri Sword and Deku Shield.
pub fn kokiri_forest_start() -> TestScenario {
    let new_game = GameStateFixture::new("new_game", "Fresh start");

    let got_sword =
        GameStateFixture::new("got_sword", "Found Kokiri Sword").with_equipment(Equipment {
            kokiri_sword: true,
            ..Default::default()
        });

    let got_shield =
        GameStateFixture::new("got_shield", "Bought Deku Shield").with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        });

    TestScenario::new(
        "kokiri_forest_start",
        "Player collects initial equipment in Kokiri Forest",
    )
    .step(ScenarioStep::new("Start new game", new_game))
    .step(
        ScenarioStep::new("Find Kokiri Sword", got_sword)
            .expect_event(TrackerEvent::EquipmentObtained("Kokiri Sword".to_string())),
    )
    .step(
        ScenarioStep::new("Buy Deku Shield", got_shield)
            .expect_event(TrackerEvent::EquipmentObtained("Deku Shield".to_string())),
    )
    .with_timeout(Duration::from_secs(30))
}

/// Scenario: Player completes Inside the Deku Tree.
pub fn deku_tree_completion() -> TestScenario {
    let entering = GameStateFixture::new("entering_deku_tree", "Entering Deku Tree")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        });

    let got_slingshot = GameStateFixture::new("got_slingshot", "Found Slingshot")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot);

    let defeated_gohma = GameStateFixture::new("defeated_gohma", "Defeated Queen Gohma")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            ..Default::default()
        })
        .with_health(16, 16);

    let got_emerald = GameStateFixture::new("got_emerald", "Got Kokiri Emerald")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            ..Default::default()
        })
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            ..Default::default()
        })
        .with_health(16, 16);

    TestScenario::new(
        "deku_tree_completion",
        "Player completes Inside the Deku Tree dungeon",
    )
    .step(ScenarioStep::new("Enter Deku Tree", entering))
    .step(
        ScenarioStep::new("Get Slingshot", got_slingshot)
            .expect_event(TrackerEvent::ItemCollected("Slingshot".to_string())),
    )
    .step(
        ScenarioStep::new("Defeat Queen Gohma", defeated_gohma).expect_events(vec![
            TrackerEvent::BossDefeated("Queen Gohma".to_string()),
            TrackerEvent::HealthChanged {
                current: 16,
                max: 16,
            },
        ]),
    )
    .step(
        ScenarioStep::new("Collect Kokiri Emerald", got_emerald)
            .expect_event(TrackerEvent::DungeonReward("Kokiri Emerald".to_string())),
    )
    .with_timeout(Duration::from_secs(60))
}

/// Scenario: Player collects all three spiritual stones.
pub fn spiritual_stones_collection() -> TestScenario {
    let start = GameStateFixture::new("child_start", "Starting as child")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_item(ItemSlot::FairyOcarina, ItemId::FairyOcarina);

    let got_emerald = GameStateFixture::new("got_emerald", "Got Kokiri Emerald")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_item(ItemSlot::FairyOcarina, ItemId::FairyOcarina)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            ..Default::default()
        });

    let got_ruby = GameStateFixture::new("got_ruby", "Got Goron Ruby")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_item(ItemSlot::FairyOcarina, ItemId::FairyOcarina)
        .with_item(ItemSlot::Bombs, ItemId::Bomb)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            ..Default::default()
        });

    let got_sapphire = GameStateFixture::new("got_sapphire", "Got Zora Sapphire")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_item(ItemSlot::FairyOcarina, ItemId::FairyOcarina)
        .with_item(ItemSlot::Bombs, ItemId::Bomb)
        .with_item(ItemSlot::Boomerang, ItemId::Boomerang)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            ..Default::default()
        });

    TestScenario::new(
        "spiritual_stones_collection",
        "Player collects all three spiritual stones",
    )
    .step(ScenarioStep::new("Start with equipment", start))
    .step(
        ScenarioStep::new("Get Kokiri Emerald", got_emerald).expect_events(vec![
            TrackerEvent::DungeonReward("Kokiri Emerald".to_string()),
            TrackerEvent::BossDefeated("Queen Gohma".to_string()),
        ]),
    )
    .step(
        ScenarioStep::new("Get Goron Ruby", got_ruby).expect_events(vec![
            TrackerEvent::DungeonReward("Goron Ruby".to_string()),
            TrackerEvent::BossDefeated("King Dodongo".to_string()),
            TrackerEvent::ItemCollected("Bombs".to_string()),
            TrackerEvent::EquipmentObtained("Hylian Shield".to_string()),
        ]),
    )
    .step(
        ScenarioStep::new("Get Zora Sapphire", got_sapphire).expect_events(vec![
            TrackerEvent::DungeonReward("Zora Sapphire".to_string()),
            TrackerEvent::BossDefeated("Barinade".to_string()),
            TrackerEvent::ItemCollected("Boomerang".to_string()),
        ]),
    )
    .with_timeout(Duration::from_secs(90))
}

/// Scenario: Player becomes adult Link.
pub fn time_travel_to_adult() -> TestScenario {
    let child_complete = GameStateFixture::new("child_complete", "Child with all stones")
        .with_equipment(Equipment {
            kokiri_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_item(ItemSlot::Bombs, ItemId::Bomb)
        .with_item(ItemSlot::Boomerang, ItemId::Boomerang)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            ..Default::default()
        });

    let adult_link = GameStateFixture::new("adult_link", "Adult Link awakens")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
        .with_item(ItemSlot::Slingshot, ItemId::FairySlingshot)
        .with_item(ItemSlot::Bombs, ItemId::Bomb)
        .with_item(ItemSlot::Boomerang, ItemId::Boomerang)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            ..Default::default()
        });

    TestScenario::new(
        "time_travel_to_adult",
        "Player pulls Master Sword and becomes adult",
    )
    .step(ScenarioStep::new("Complete child dungeons", child_complete))
    .step(
        ScenarioStep::new("Pull Master Sword", adult_link).expect_events(vec![
            TrackerEvent::AgeChanged(true),
            TrackerEvent::EquipmentObtained("Master Sword".to_string()),
            TrackerEvent::DungeonReward("Light Medallion".to_string()),
        ]),
    )
    .with_timeout(Duration::from_secs(45))
}

/// Scenario: Player completes Forest Temple.
pub fn forest_temple_completion() -> TestScenario {
    let adult_start = GameStateFixture::new("adult_start", "Adult starting Forest Temple")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            ..Default::default()
        });

    let got_hookshot = GameStateFixture::new("got_hookshot", "Got Hookshot from Dampé")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
        .with_item(ItemSlot::Hookshot, ItemId::Hookshot)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            ..Default::default()
        });

    let got_bow = GameStateFixture::new("got_bow", "Got Fairy Bow in Forest Temple")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
        .with_item(ItemSlot::Hookshot, ItemId::Hookshot)
        .with_item(ItemSlot::FairyBow, ItemId::FairyBow)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            ..Default::default()
        });

    let defeated_phantom = GameStateFixture::new("defeated_phantom", "Defeated Phantom Ganon")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            deku_shield: true,
            hylian_shield: true,
            ..Default::default()
        })
        .with_item(ItemSlot::FairyOcarina, ItemId::OcarinaOfTime)
        .with_item(ItemSlot::Hookshot, ItemId::Hookshot)
        .with_item(ItemSlot::FairyBow, ItemId::FairyBow)
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            ..Default::default()
        });

    TestScenario::new(
        "forest_temple_completion",
        "Player completes Forest Temple dungeon",
    )
    .step(ScenarioStep::new("Start as adult", adult_start))
    .step(
        ScenarioStep::new("Get Hookshot", got_hookshot)
            .expect_event(TrackerEvent::ItemCollected("Hookshot".to_string())),
    )
    .step(
        ScenarioStep::new("Get Fairy Bow", got_bow)
            .expect_event(TrackerEvent::ItemCollected("Fairy Bow".to_string())),
    )
    .step(
        ScenarioStep::new("Defeat Phantom Ganon", defeated_phantom).expect_events(vec![
            TrackerEvent::BossDefeated("Phantom Ganon".to_string()),
            TrackerEvent::DungeonReward("Forest Medallion".to_string()),
        ]),
    )
    .with_timeout(Duration::from_secs(90))
}

/// Scenario: Player upgrades Hookshot to Longshot in Water Temple.
pub fn hookshot_upgrade() -> TestScenario {
    let with_hookshot = GameStateFixture::new("with_hookshot", "Adult with Hookshot")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            hylian_shield: true,
            zora_tunic: true,
            iron_boots: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Hookshot, ItemId::Hookshot);

    let got_longshot = GameStateFixture::new("got_longshot", "Upgraded to Longshot")
        .as_adult()
        .with_equipment(Equipment {
            kokiri_sword: true,
            master_sword: true,
            hylian_shield: true,
            zora_tunic: true,
            iron_boots: true,
            ..Default::default()
        })
        .with_item(ItemSlot::Hookshot, ItemId::Longshot);

    TestScenario::new(
        "hookshot_upgrade",
        "Player upgrades Hookshot to Longshot in Water Temple",
    )
    .step(ScenarioStep::new(
        "Enter Water Temple with Hookshot",
        with_hookshot,
    ))
    .step(
        ScenarioStep::new("Get Longshot", got_longshot)
            .expect_event(TrackerEvent::ItemCollected("Longshot".to_string())),
    )
    .with_timeout(Duration::from_secs(30))
}

/// Scenario: Player collects all six medallions.
pub fn medallion_collection() -> TestScenario {
    let base = GameStateFixture::new("base", "Adult with spiritual stones")
        .as_adult()
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            ..Default::default()
        });

    let forest_complete = GameStateFixture::new("forest", "Forest Temple complete")
        .as_adult()
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            ..Default::default()
        });

    let fire_complete = GameStateFixture::new("fire", "Fire Temple complete")
        .as_adult()
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            ..Default::default()
        });

    let water_complete = GameStateFixture::new("water", "Water Temple complete")
        .as_adult()
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            water_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            morpha: true,
            ..Default::default()
        });

    let shadow_complete = GameStateFixture::new("shadow", "Shadow Temple complete")
        .as_adult()
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            water_medallion: true,
            shadow_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            morpha: true,
            bongo_bongo: true,
            ..Default::default()
        });

    let spirit_complete = GameStateFixture::new("spirit", "Spirit Temple complete")
        .as_adult()
        .with_quest_status(QuestStatus {
            kokiri_emerald: true,
            goron_ruby: true,
            zora_sapphire: true,
            light_medallion: true,
            forest_medallion: true,
            fire_medallion: true,
            water_medallion: true,
            shadow_medallion: true,
            spirit_medallion: true,
            ..Default::default()
        })
        .with_boss_defeats(BossDefeats {
            queen_gohma: true,
            king_dodongo: true,
            barinade: true,
            phantom_ganon: true,
            volvagia: true,
            morpha: true,
            bongo_bongo: true,
            twinrova: true,
        });

    TestScenario::new("medallion_collection", "Player collects all six medallions")
        .step(ScenarioStep::new("Start with Light Medallion", base))
        .step(
            ScenarioStep::new("Complete Forest Temple", forest_complete)
                .expect_event(TrackerEvent::DungeonReward("Forest Medallion".to_string())),
        )
        .step(
            ScenarioStep::new("Complete Fire Temple", fire_complete)
                .expect_event(TrackerEvent::DungeonReward("Fire Medallion".to_string())),
        )
        .step(
            ScenarioStep::new("Complete Water Temple", water_complete)
                .expect_event(TrackerEvent::DungeonReward("Water Medallion".to_string())),
        )
        .step(
            ScenarioStep::new("Complete Shadow Temple", shadow_complete)
                .expect_event(TrackerEvent::DungeonReward("Shadow Medallion".to_string())),
        )
        .step(
            ScenarioStep::new("Complete Spirit Temple", spirit_complete)
                .expect_event(TrackerEvent::DungeonReward("Spirit Medallion".to_string())),
        )
        .with_timeout(Duration::from_secs(120))
}

/// Scenario: Player collects gold skulltulas.
pub fn skulltula_collection() -> TestScenario {
    let zero_skulls =
        GameStateFixture::new("zero", "No skulltulas").with_quest_status(QuestStatus {
            gold_skulltulas: 0,
            ..Default::default()
        });

    let ten_skulls = GameStateFixture::new("ten", "10 skulltulas").with_quest_status(QuestStatus {
        gold_skulltulas: 10,
        ..Default::default()
    });

    let fifty_skulls =
        GameStateFixture::new("fifty", "50 skulltulas").with_quest_status(QuestStatus {
            gold_skulltulas: 50,
            ..Default::default()
        });

    let hundred_skulls =
        GameStateFixture::new("hundred", "100 skulltulas").with_quest_status(QuestStatus {
            gold_skulltulas: 100,
            ..Default::default()
        });

    TestScenario::new(
        "skulltula_collection",
        "Player collects gold skulltulas for rewards",
    )
    .step(ScenarioStep::new("Start with no skulltulas", zero_skulls))
    .step(
        ScenarioStep::new("Collect 10 skulltulas", ten_skulls)
            .expect_event(TrackerEvent::SkulltulaCountChanged(10)),
    )
    .step(
        ScenarioStep::new("Collect 50 skulltulas", fifty_skulls)
            .expect_event(TrackerEvent::SkulltulaCountChanged(50)),
    )
    .step(
        ScenarioStep::new("Collect all 100 skulltulas", hundred_skulls)
            .expect_event(TrackerEvent::SkulltulaCountChanged(100)),
    )
    .with_timeout(Duration::from_secs(60))
}

/// Returns all pre-defined scenarios.
pub fn all_scenarios() -> Vec<TestScenario> {
    vec![
        kokiri_forest_start(),
        deku_tree_completion(),
        spiritual_stones_collection(),
        time_travel_to_adult(),
        forest_temple_completion(),
        hookshot_upgrade(),
        medallion_collection(),
        skulltula_collection(),
    ]
}

/// Returns scenarios suitable for quick smoke tests.
pub fn smoke_test_scenarios() -> Vec<TestScenario> {
    vec![kokiri_forest_start(), hookshot_upgrade()]
}

/// Returns scenarios for full regression testing.
pub fn regression_scenarios() -> Vec<TestScenario> {
    all_scenarios()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_step_count() {
        let scenario = kokiri_forest_start();
        assert_eq!(scenario.step_count(), 3);
    }

    #[test]
    fn test_scenario_expected_events() {
        let scenario = deku_tree_completion();
        let events = scenario.all_expected_events();

        // Should include slingshot, boss defeat, health change, and emerald
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_all_scenarios_have_steps() {
        for scenario in all_scenarios() {
            assert!(
                scenario.step_count() >= 2,
                "Scenario '{}' should have at least 2 steps",
                scenario.id
            );
        }
    }

    #[test]
    fn test_smoke_scenarios_are_subset() {
        let smoke = smoke_test_scenarios();
        let all = all_scenarios();

        for smoke_scenario in smoke {
            assert!(
                all.iter().any(|s| s.id == smoke_scenario.id),
                "Smoke scenario '{}' should be in all_scenarios()",
                smoke_scenario.id
            );
        }
    }

    #[test]
    fn test_spiritual_stones_scenario() {
        let scenario = spiritual_stones_collection();

        // Find the step that gets Zora Sapphire
        let sapphire_step = scenario
            .steps
            .iter()
            .find(|s| s.description == "Get Zora Sapphire")
            .expect("Should have sapphire step");

        // Should expect barinade defeat
        assert!(
            sapphire_step
                .expected_events
                .contains(&TrackerEvent::BossDefeated("Barinade".to_string())),
            "Sapphire step should expect Barinade defeat"
        );
    }

    #[test]
    fn test_time_travel_scenario() {
        let scenario = time_travel_to_adult();

        // Should have exactly 2 steps
        assert_eq!(scenario.step_count(), 2);

        // Second step should expect age change
        let adult_step = &scenario.steps[1];
        assert!(
            adult_step
                .expected_events
                .contains(&TrackerEvent::AgeChanged(true)),
            "Adult step should expect age change to adult"
        );
    }

    #[test]
    fn test_medallion_scenario_complete() {
        let scenario = medallion_collection();
        let events = scenario.all_expected_events();

        // Should have all 5 adult medallions (light is given at start)
        let medallion_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, TrackerEvent::DungeonReward(_)))
            .collect();

        assert_eq!(
            medallion_events.len(),
            5,
            "Should expect exactly 5 medallion events"
        );
    }
}
