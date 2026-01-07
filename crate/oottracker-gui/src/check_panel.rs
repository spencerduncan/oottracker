//! Collapsible check panel widget for displaying checked locations grouped by region.
//!
//! This module provides a side panel that displays checked locations from the
//! `CheckedLocationsSummary`, grouped by region (dungeons, overworld areas).
//! Each region section is collapsible with expand/collapse buttons.

use {
    iced::{
        widget::{
            button::{self, Button},
            scrollable::{self, Scrollable},
            Column, Row, Space, Text,
        },
        Background, Color, Element, Length,
    },
    oottracker::flag_mapping::{CheckStatus, CheckedLocationsSummary, LocationCheckResult},
    std::collections::{BTreeMap, HashSet},
};

/// Status icons for location check status.
const ICON_CHECKED: &str = "\u{25CF}"; // ● - filled circle
const ICON_UNCHECKED: &str = "\u{2713}"; // ✓ - check mark (accessible, not yet checked)
const ICON_UNKNOWN: &str = "?"; // ? - unknown/unmapped

/// Message types for the check panel widget.
#[derive(Debug, Clone)]
pub enum Message {
    /// Toggle the expanded/collapsed state of a region section.
    ToggleRegion(String),
    /// Toggle the overall panel visibility.
    TogglePanel,
}

/// State for the check panel widget.
#[derive(Debug, Default)]
pub struct CheckPanelState {
    /// Whether the panel is expanded (visible).
    pub panel_expanded: bool,
    /// Set of collapsed region names.
    pub collapsed_regions: HashSet<String>,
    /// Scrollable state for the panel content.
    pub scroll_state: scrollable::State,
    /// Button state for the panel toggle.
    pub toggle_panel_btn: button::State,
    /// Button states for each region toggle (region name -> button state).
    /// Using a Vec since BTreeMap doesn't work well with button::State.
    pub region_toggle_btns: Vec<(String, button::State)>,
}

impl CheckPanelState {
    /// Creates a new check panel state with the panel initially collapsed.
    pub fn new() -> Self {
        Self {
            panel_expanded: false,
            collapsed_regions: HashSet::new(),
            scroll_state: scrollable::State::new(),
            toggle_panel_btn: button::State::new(),
            region_toggle_btns: Vec::new(),
        }
    }

    /// Handles a message and updates the state accordingly.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleRegion(region) => {
                if self.collapsed_regions.contains(&region) {
                    self.collapsed_regions.remove(&region);
                } else {
                    self.collapsed_regions.insert(region);
                }
            }
            Message::TogglePanel => {
                self.panel_expanded = !self.panel_expanded;
            }
        }
    }

    /// Gets or creates a button state for a region toggle.
    fn get_region_btn(&mut self, region: &str) -> &mut button::State {
        // Find existing button state or create new one
        let idx = self
            .region_toggle_btns
            .iter()
            .position(|(r, _)| r == region);

        match idx {
            Some(i) => &mut self.region_toggle_btns[i].1,
            None => {
                self.region_toggle_btns
                    .push((region.to_string(), button::State::new()));
                &mut self.region_toggle_btns.last_mut().unwrap().1
            }
        }
    }

    /// Renders the check panel view.
    ///
    /// # Arguments
    ///
    /// * `summary` - Optional summary of checked locations. If None, shows "No data".
    ///
    /// # Returns
    ///
    /// An Element containing the check panel UI.
    pub fn view<'a, M: 'a + Clone>(
        &'a mut self,
        summary: Option<&CheckedLocationsSummary>,
        map_message: impl Fn(Message) -> M + 'a + Copy,
    ) -> Element<'a, M> {
        // Panel toggle button
        let toggle_text = if self.panel_expanded {
            "Checks \u{25B6}" // ▶ (collapse)
        } else {
            "\u{25C0} Checks" // ◀ (expand)
        };

        let toggle_btn = Button::new(&mut self.toggle_panel_btn, Text::new(toggle_text).size(14))
            .on_press(map_message(Message::TogglePanel))
            .padding(4)
            .style(PanelButtonStyle);

        if !self.panel_expanded {
            // Just show the toggle button when collapsed
            return Column::new().push(toggle_btn).width(Length::Shrink).into();
        }

        // Build the panel content
        let content = match summary {
            None => Column::new()
                .push(Text::new("No location data").color([0.7, 0.7, 0.7]))
                .padding(10),
            Some(summary) => self.build_panel_content(summary, map_message),
        };

        // Wrap content in scrollable
        let scrollable_content = Scrollable::new(&mut self.scroll_state)
            .push(content)
            .height(Length::Fill)
            .width(Length::Units(280));

        Column::new()
            .push(
                Row::new()
                    .push(toggle_btn)
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(format!(
                            "{}/{}",
                            summary.map(|s| s.checked_count).unwrap_or(0),
                            summary.map(|s| s.total_mapped).unwrap_or(0)
                        ))
                        .size(14)
                        .color([0.8, 0.8, 0.8]),
                    )
                    .padding(4)
                    .width(Length::Fill),
            )
            .push(scrollable_content)
            .width(Length::Units(300))
            .height(Length::Fill)
            .into()
    }

    /// Builds the panel content with regions and locations.
    fn build_panel_content<'a, M: 'a + Clone>(
        &'a mut self,
        summary: &CheckedLocationsSummary,
        map_message: impl Fn(Message) -> M + 'a + Copy,
    ) -> Column<'a, M> {
        // Group locations by region
        let grouped = group_locations_by_region(&summary.locations);

        let mut content = Column::new().spacing(4).padding(8);

        // Overall summary header
        content = content.push(
            Row::new()
                .push(
                    Text::new(format!(
                        "Checked: {}/{} ({} unknown)",
                        summary.checked_count, summary.total_mapped, summary.unknown_count
                    ))
                    .size(12)
                    .color([0.6, 0.8, 0.6]),
                )
                .padding(4),
        );

        content = content.push(Space::with_height(Length::Units(8)));

        // Build region sections
        // We need to collect regions first to avoid borrowing issues
        let regions: Vec<_> = grouped.keys().cloned().collect();

        for region_name in regions {
            if let Some(locations) = grouped.get(&region_name) {
                let is_collapsed = self.collapsed_regions.contains(&region_name);

                // Count checked/total for this region
                let region_checked = locations
                    .iter()
                    .filter(|l| l.status == CheckStatus::Checked)
                    .count();
                let region_total = locations.len();

                // Region header with expand/collapse button
                let expand_icon = if is_collapsed {
                    "\u{25B6}" // ▶
                } else {
                    "\u{25BC}" // ▼
                };

                let region_btn = Button::new(
                    self.get_region_btn(&region_name),
                    Row::new()
                        .push(Text::new(expand_icon).size(10))
                        .push(Space::with_width(Length::Units(4)))
                        .push(Text::new(&region_name).size(13))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Text::new(format!("{}/{}", region_checked, region_total))
                                .size(11)
                                .color([0.6, 0.6, 0.6]),
                        ),
                )
                .on_press(map_message(Message::ToggleRegion(region_name.clone())))
                .padding(4)
                .width(Length::Fill)
                .style(RegionButtonStyle);

                content = content.push(region_btn);

                // Location list (if expanded)
                if !is_collapsed {
                    let mut locations_col = Column::new().spacing(1).padding([0, 0, 4, 16]);

                    for loc in locations {
                        let (icon, color) = match loc.status {
                            CheckStatus::Checked => (ICON_CHECKED, [0.4, 0.8, 0.4]), // green
                            CheckStatus::Unchecked => (ICON_UNCHECKED, [0.8, 0.8, 0.4]), // yellow
                            CheckStatus::Unknown => (ICON_UNKNOWN, [0.5, 0.5, 0.5]), // gray
                        };

                        // Format location name (remove region prefix, prettify)
                        let display_name = format_location_name(&loc.location_id, &region_name);

                        locations_col = locations_col.push(
                            Row::new()
                                .push(Text::new(icon).size(10).color(color))
                                .push(Space::with_width(Length::Units(4)))
                                .push(Text::new(display_name).size(11).color(
                                    if loc.status == CheckStatus::Checked {
                                        [0.5, 0.5, 0.5]
                                    } else {
                                        [0.8, 0.8, 0.8]
                                    },
                                )),
                        );
                    }

                    content = content.push(locations_col);
                }
            }
        }

        content
    }
}

/// Groups locations by their region name, extracted from the location_id.
///
/// Location IDs follow the pattern: `oot_<region>_<location_name>`
/// e.g., `oot_deku_tree_compass_chest` -> region: "Deku Tree"
fn group_locations_by_region(
    locations: &[LocationCheckResult],
) -> BTreeMap<String, Vec<&LocationCheckResult>> {
    let mut groups: BTreeMap<String, Vec<&LocationCheckResult>> = BTreeMap::new();

    for loc in locations {
        let region = extract_region_name(&loc.location_id);
        groups.entry(region).or_default().push(loc);
    }

    groups
}

/// Extracts the region name from a location ID.
///
/// Location IDs follow patterns like:
/// - `oot_deku_tree_compass_chest` -> "Deku Tree"
/// - `oot_kokiri_forest_sword_chest` -> "Kokiri Forest"
/// - `oot_fire_temple_boss_key` -> "Fire Temple"
/// - `mq_oot_mq_deku_tree_compass_chest` -> "Deku Tree (MQ)"
fn extract_region_name(location_id: &str) -> String {
    // Check if this is an MQ location
    let is_mq = location_id.starts_with("mq_oot_");

    // Remove game prefix (handle MQ prefixes first, then vanilla)
    // MQ locations have pattern: mq_oot_mq_<dungeon>_<location>
    // or mq_oot_<dungeon>_<location> for regions
    let without_prefix = location_id
        .strip_prefix("mq_oot_mq_")
        .or_else(|| location_id.strip_prefix("mq_oot_"))
        .or_else(|| location_id.strip_prefix("oot_"))
        .or_else(|| location_id.strip_prefix("mm_"))
        .unwrap_or(location_id);

    // Known region patterns (longest matches first)
    let known_regions = [
        // Dungeons
        ("deku_tree", "Deku Tree"),
        ("dodongo_cavern", "Dodongo's Cavern"),
        ("jabu_jabu", "Jabu Jabu's Belly"),
        ("forest_temple", "Forest Temple"),
        ("fire_temple", "Fire Temple"),
        ("water_temple", "Water Temple"),
        ("shadow_temple", "Shadow Temple"),
        ("spirit_temple", "Spirit Temple"),
        ("bottom_of_the_well", "Bottom of the Well"),
        ("ice_cavern", "Ice Cavern"),
        ("gerudo_training_ground", "Gerudo Training Ground"),
        ("ganon_castle", "Ganon's Castle"),
        // Overworld areas
        ("kokiri_forest", "Kokiri Forest"),
        ("lost_woods", "Lost Woods"),
        ("sacred_forest_meadow", "Sacred Forest Meadow"),
        ("hyrule_field", "Hyrule Field"),
        ("lon_lon_ranch", "Lon Lon Ranch"),
        ("kakariko", "Kakariko Village"),
        ("graveyard", "Graveyard"),
        ("death_mountain_trail", "Death Mountain Trail"),
        ("death_mountain_crater", "Death Mountain Crater"),
        ("goron_city", "Goron City"),
        ("zora_river", "Zora's River"),
        ("zora_domain", "Zora's Domain"),
        ("zora_fountain", "Zora's Fountain"),
        ("lake_hylia", "Lake Hylia"),
        ("gerudo_valley", "Gerudo Valley"),
        ("gerudo_fortress", "Gerudo Fortress"),
        ("haunted_wasteland", "Haunted Wasteland"),
        ("desert_colossus", "Desert Colossus"),
        ("market", "Hyrule Castle / Market"),
        ("hyrule_castle", "Hyrule Castle / Market"),
        ("temple_of_time", "Temple of Time"),
        // Gold Skulltulas
        ("gs_", "Gold Skulltulas"),
        // Other
        ("shop", "Shops"),
        ("reward", "Rewards"),
    ];

    // List of dungeon patterns that can have MQ variants
    let dungeon_patterns = [
        "deku_tree",
        "dodongo_cavern",
        "jabu_jabu",
        "forest_temple",
        "fire_temple",
        "water_temple",
        "shadow_temple",
        "spirit_temple",
        "bottom_of_the_well",
        "ice_cavern",
        "gerudo_training",
        "ganon_castle",
    ];

    for (prefix, name) in known_regions {
        if without_prefix.starts_with(prefix) {
            // Add (MQ) suffix for MQ dungeon locations
            if is_mq && dungeon_patterns.iter().any(|d| prefix.starts_with(d)) {
                return format!("{} (MQ)", name);
            }
            return name.to_string();
        }
    }

    // Fallback: capitalize the first part before the last underscore-separated word
    let parts: Vec<&str> = without_prefix.split('_').collect();
    let base_name = if parts.len() > 1 {
        // Take all but the last part as the region name
        let region_parts = &parts[..parts.len().saturating_sub(1)];
        region_parts
            .iter()
            .map(|p| {
                let mut chars = p.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        without_prefix.to_string()
    };

    // Add (MQ) suffix for MQ locations even in fallback case
    if is_mq {
        format!("{} (MQ)", base_name)
    } else {
        base_name
    }
}

/// Formats a location name for display by removing the region prefix and prettifying.
fn format_location_name(location_id: &str, region_name: &str) -> String {
    // Remove game prefix (handle MQ prefixes first, then vanilla)
    let without_prefix = location_id
        .strip_prefix("mq_oot_mq_")
        .or_else(|| location_id.strip_prefix("mq_oot_"))
        .or_else(|| location_id.strip_prefix("oot_"))
        .or_else(|| location_id.strip_prefix("mm_"))
        .unwrap_or(location_id);

    // Convert region name to snake_case for matching (remove MQ suffix if present)
    let region_base = region_name.trim_end_matches(" (MQ)");
    let region_snake = region_base.to_lowercase().replace([' ', '\''], "_");
    let region_snake = region_snake.replace("__", "_");

    // Remove region prefix from location name
    let without_region = without_prefix
        .strip_prefix(&region_snake)
        .map(|s| s.strip_prefix('_').unwrap_or(s))
        .unwrap_or(without_prefix);

    // Prettify: replace underscores with spaces and capitalize words
    without_region
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Button style for the panel toggle.
struct PanelButtonStyle;

impl button::StyleSheet for PanelButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
            border_radius: 4.0,
            border_width: 0.0,
            text_color: Color::from_rgb(0.8, 0.8, 0.8),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
            ..self.active()
        }
    }
}

/// Button style for region headers.
struct RegionButtonStyle;

impl button::StyleSheet for RegionButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
            border_radius: 2.0,
            border_width: 0.0,
            text_color: Color::from_rgb(0.9, 0.9, 0.9),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.25, 0.25, 0.25))),
            ..self.active()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_region_name_deku_tree() {
        assert_eq!(
            extract_region_name("oot_deku_tree_compass_chest"),
            "Deku Tree"
        );
    }

    #[test]
    fn test_extract_region_name_fire_temple() {
        assert_eq!(
            extract_region_name("oot_fire_temple_boss_key"),
            "Fire Temple"
        );
    }

    #[test]
    fn test_extract_region_name_kokiri_forest() {
        assert_eq!(
            extract_region_name("oot_kokiri_forest_sword_chest"),
            "Kokiri Forest"
        );
    }

    #[test]
    fn test_extract_region_name_unknown() {
        // Fallback behavior for unknown regions
        let result = extract_region_name("oot_some_unknown_location_chest");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_location_name_basic() {
        let result = format_location_name("oot_deku_tree_compass_chest", "Deku Tree");
        assert_eq!(result, "Compass Chest");
    }

    #[test]
    fn test_format_location_name_fire_temple() {
        let result = format_location_name("oot_fire_temple_boss_key_chest", "Fire Temple");
        assert_eq!(result, "Boss Key Chest");
    }

    #[test]
    fn test_check_panel_state_new() {
        let state = CheckPanelState::new();
        assert!(!state.panel_expanded);
        assert!(state.collapsed_regions.is_empty());
    }

    #[test]
    fn test_check_panel_state_toggle_panel() {
        let mut state = CheckPanelState::new();
        assert!(!state.panel_expanded);

        state.update(Message::TogglePanel);
        assert!(state.panel_expanded);

        state.update(Message::TogglePanel);
        assert!(!state.panel_expanded);
    }

    #[test]
    fn test_check_panel_state_toggle_region() {
        let mut state = CheckPanelState::new();
        let region = "Deku Tree".to_string();

        assert!(!state.collapsed_regions.contains(&region));

        state.update(Message::ToggleRegion(region.clone()));
        assert!(state.collapsed_regions.contains(&region));

        state.update(Message::ToggleRegion(region.clone()));
        assert!(!state.collapsed_regions.contains(&region));
    }

    #[test]
    fn test_group_locations_by_region() {
        let locations = vec![
            LocationCheckResult {
                location_id: "oot_deku_tree_compass_chest".to_string(),
                status: CheckStatus::Checked,
                is_mapped: true,
            },
            LocationCheckResult {
                location_id: "oot_deku_tree_map_chest".to_string(),
                status: CheckStatus::Unchecked,
                is_mapped: true,
            },
            LocationCheckResult {
                location_id: "oot_fire_temple_boss_key".to_string(),
                status: CheckStatus::Unknown,
                is_mapped: false,
            },
        ];

        let grouped = group_locations_by_region(&locations);

        assert!(grouped.contains_key("Deku Tree"));
        assert!(grouped.contains_key("Fire Temple"));
        assert_eq!(grouped.get("Deku Tree").unwrap().len(), 2);
        assert_eq!(grouped.get("Fire Temple").unwrap().len(), 1);
    }

    #[test]
    fn test_status_icons() {
        assert_eq!(ICON_CHECKED, "\u{25CF}");
        assert_eq!(ICON_UNCHECKED, "\u{2713}");
        assert_eq!(ICON_UNKNOWN, "?");
    }

    // MQ location tests

    #[test]
    fn test_extract_region_name_mq_deku_tree() {
        assert_eq!(
            extract_region_name("mq_oot_mq_deku_tree_compass_chest"),
            "Deku Tree (MQ)"
        );
    }

    #[test]
    fn test_extract_region_name_mq_fire_temple() {
        assert_eq!(
            extract_region_name("mq_oot_mq_fire_temple_boss_key"),
            "Fire Temple (MQ)"
        );
    }

    #[test]
    fn test_extract_region_name_mq_ganon_castle() {
        assert_eq!(
            extract_region_name("mq_oot_mq_ganon_castle_light_trial"),
            "Ganon's Castle (MQ)"
        );
    }

    #[test]
    fn test_format_location_name_mq_basic() {
        let result = format_location_name("mq_oot_mq_deku_tree_compass_chest", "Deku Tree (MQ)");
        assert_eq!(result, "Compass Chest");
    }

    #[test]
    fn test_format_location_name_mq_fire_temple() {
        let result =
            format_location_name("mq_oot_mq_fire_temple_boss_key_chest", "Fire Temple (MQ)");
        assert_eq!(result, "Boss Key Chest");
    }

    #[test]
    fn test_group_locations_by_region_with_mq() {
        let locations = vec![
            LocationCheckResult {
                location_id: "oot_deku_tree_compass_chest".to_string(),
                status: CheckStatus::Checked,
                is_mapped: true,
            },
            LocationCheckResult {
                location_id: "mq_oot_mq_fire_temple_boss_key".to_string(),
                status: CheckStatus::Unchecked,
                is_mapped: true,
            },
        ];

        let grouped = group_locations_by_region(&locations);

        assert!(grouped.contains_key("Deku Tree"));
        assert!(grouped.contains_key("Fire Temple (MQ)"));
        assert_eq!(grouped.get("Deku Tree").unwrap().len(), 1);
        assert_eq!(grouped.get("Fire Temple (MQ)").unwrap().len(), 1);
    }
}
