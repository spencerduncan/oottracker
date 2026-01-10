/**
 * Checked Locations Display Module
 *
 * This module fetches and displays the checked locations status
 * from the API endpoint and updates the UI accordingly.
 * Locations are grouped by region with collapsible sections.
 *
 * Updates are triggered via WebSocket events from proto.js rather than
 * polling, providing immediate refresh when tracker state changes.
 */

// Checked locations state
let checkedLocationsState = {
    locations: {},       // location_id -> { status, accessibility }
    logic: {}, // Map of location_id -> logic expression
    lastUpdate: null,
    isLoading: false,
    error: null,
    collapsedRegions: new Set(), // Track which regions are collapsed
    pendingSkipToggles: new Set(), // Track locations with pending skip toggle requests
    hideUnavailable: false, // Filter to hide inaccessible locations
    hideMqLocations: false, // Filter to hide Master Quest dungeon checks
    // Auto-scroll state
    lastSceneId: null,           // Track last seen scene ID for detecting changes
    lastGame: null,              // Track which game was active ('oot' or 'mm')
    autoScrollEnabled: localStorage.getItem('oottracker_autoscroll_enabled') !== 'false' // User preference
};

// Status display element
let statusElement = null;

/**
 * Region name mapping from location ID prefixes to human-readable names.
 * The order here determines display order (dungeons first, then overworld).
 */
const REGION_DISPLAY_ORDER = [
    // OoT Child Dungeons
    'deku_tree',
    'dodongo_cavern',
    'jabu_jabu',
    // OoT Adult Dungeons
    'forest_temple',
    'fire_temple',
    'water_temple',
    'spirit_temple',
    'shadow_temple',
    // OoT Mini Dungeons
    'bottom_of_the_well',
    'ice_cavern',
    'gerudo_training',
    'ganon_castle',
    'treasure_chest_game',
    // OoT Overworld - Kokiri/Forest
    'kokiri_forest',
    'kf',
    'lw',
    'sfm',
    // OoT Overworld - Hyrule
    'hf',
    'lon_lon_ranch',
    'market',
    'hyrule_castle',
    'temple_of_time',
    // OoT Overworld - Kakariko
    'kak',
    'graveyard',
    // OoT Overworld - Death Mountain
    'dmt',
    'death_mountain_trail',
    'goron_city',
    'dmc',
    'death_mountain_crater',
    // OoT Overworld - Zora
    'zr',
    'zora_domain',
    'zora_fountain',
    'lake_hylia',
    // OoT Overworld - Gerudo
    'gerudo_valley',
    'gerudo_fortress',
    'haunted_wasteland',
    'desert_colossus',
    // MM Main Dungeons
    'woodfall_temple',
    'snowhead_temple',
    'great_bay_temple',
    'stone_tower_temple',
    'stone_tower_temple_inverted',
    // MM Mini Dungeons
    'beneath_the_well',
    'ancient_castle_of_ikana',
    'pirates_fortress',
    'secret_shrine',
    'swamp_spider_house',
    'ocean_spider_house',
    // MM Clock Town
    'clock_town',
    'post_office',
    'swordsman_school',
    'mayors_office',
    'chest_game',
    'stock_pot_inn',
    'curiosity_shop',
    'milk_bar',
    'astral_observatory',
    // MM Termina Field & Swamp
    'termina_field',
    'road_to_southern_swamp',
    'southern_swamp',
    'deku_palace',
    'deku_shrine',
    'woodfall',
    // MM Mountain
    'mountain_village',
    'goron_village',
    'goron_shrine',
    'path_to_snowhead',
    'snowhead',
    // MM Great Bay
    'great_bay_coast',
    'pinnacle_rock',
    'zora_cape',
    'zora_hall',
    'laboratory',
    // MM Ikana
    'ikana_canyon',
    'ikana_valley',
    'ikana_graveyard',
    'beneath_the_graveyard',
    'ghost_hut',
    'stone_tower',
    'music_box_house',
    'road_to_ikana',
    // MM Romani Ranch
    'romani_ranch',
    'milk_road',
    'doggy_racetrack',
    'cucco_shack',
    'gorman_track',
    // MM Moon
    'moon_trial_deku',
    'moon_trial_goron',
    'moon_trial_zora',
    'moon_trial_link',
    'moon'
];

const REGION_NAMES = {
    // OoT Child Dungeons
    'deku_tree': 'Deku Tree',
    'dodongo_cavern': "Dodongo's Cavern",
    'jabu_jabu': "Jabu Jabu's Belly",
    // OoT Adult Dungeons
    'forest_temple': 'Forest Temple',
    'fire_temple': 'Fire Temple',
    'water_temple': 'Water Temple',
    'spirit_temple': 'Spirit Temple',
    'shadow_temple': 'Shadow Temple',
    // OoT Mini Dungeons
    'bottom_of_the_well': 'Bottom of the Well',
    'ice_cavern': 'Ice Cavern',
    'gerudo_training': 'Gerudo Training Ground',
    'ganon_castle': "Ganon's Castle",
    'treasure_chest_game': 'Treasure Chest Game',
    // OoT Overworld - Kokiri/Forest
    'kokiri_forest': 'Kokiri Forest',
    'kf': 'Kokiri Forest',
    'lw': 'Lost Woods',
    'sfm': 'Sacred Forest Meadow',
    // OoT Overworld - Hyrule
    'hf': 'Hyrule Field',
    'lon_lon_ranch': 'Lon Lon Ranch',
    'market': 'Market',
    'hyrule_castle': 'Hyrule Castle',
    'temple_of_time': 'Temple of Time',
    // OoT Overworld - Kakariko
    'kak': 'Kakariko Village',
    'graveyard': 'Graveyard',
    // OoT Overworld - Death Mountain
    'dmt': 'Death Mountain Trail',
    'death_mountain_trail': 'Death Mountain Trail',
    'goron_city': 'Goron City',
    'dmc': 'Death Mountain Crater',
    'death_mountain_crater': 'Death Mountain Crater',
    // OoT Overworld - Zora
    'zr': "Zora's River",
    'zora_domain': "Zora's Domain",
    'zora_fountain': "Zora's Fountain",
    'lake_hylia': 'Lake Hylia',
    // OoT Overworld - Gerudo
    'gerudo_valley': 'Gerudo Valley',
    'gerudo_fortress': 'Gerudo Fortress',
    'haunted_wasteland': 'Haunted Wasteland',
    'desert_colossus': 'Desert Colossus',
    // MM Main Dungeons
    'woodfall_temple': 'Woodfall Temple',
    'snowhead_temple': 'Snowhead Temple',
    'great_bay_temple': 'Great Bay Temple',
    'stone_tower_temple': 'Stone Tower Temple',
    'stone_tower_temple_inverted': 'Stone Tower Temple (Inverted)',
    // MM Mini Dungeons
    'beneath_the_well': 'Beneath the Well',
    'ancient_castle_of_ikana': 'Ancient Castle of Ikana',
    'pirates_fortress': "Pirates' Fortress",
    'secret_shrine': 'Secret Shrine',
    'swamp_spider_house': 'Swamp Spider House',
    'ocean_spider_house': 'Ocean Spider House',
    // MM Clock Town
    'clock_town': 'Clock Town',
    'post_office': 'Post Office',
    'swordsman_school': 'Swordsman School',
    'mayors_office': "Mayor's Office",
    'chest_game': 'Chest Game',
    'stock_pot_inn': 'Stock Pot Inn',
    'curiosity_shop': 'Curiosity Shop',
    'milk_bar': 'Milk Bar',
    'astral_observatory': 'Astral Observatory',
    // MM Termina Field & Swamp
    'termina_field': 'Termina Field',
    'road_to_southern_swamp': 'Road to Southern Swamp',
    'southern_swamp': 'Southern Swamp',
    'deku_palace': 'Deku Palace',
    'deku_shrine': 'Deku Shrine',
    'woodfall': 'Woodfall',
    // MM Mountain
    'mountain_village': 'Mountain Village',
    'goron_village': 'Goron Village',
    'goron_shrine': 'Goron Shrine',
    'path_to_snowhead': 'Path to Snowhead',
    'snowhead': 'Snowhead',
    // MM Great Bay
    'great_bay_coast': 'Great Bay Coast',
    'pinnacle_rock': 'Pinnacle Rock',
    'zora_cape': 'Zora Cape',
    'zora_hall': 'Zora Hall',
    'laboratory': 'Marine Research Lab',
    // MM Ikana
    'ikana_canyon': 'Ikana Canyon',
    'ikana_valley': 'Ikana Valley',
    'ikana_graveyard': 'Ikana Graveyard',
    'beneath_the_graveyard': 'Beneath the Graveyard',
    'ghost_hut': 'Ghost Hut',
    'stone_tower': 'Stone Tower',
    'music_box_house': 'Music Box House',
    'road_to_ikana': 'Road to Ikana',
    // MM Romani Ranch
    'romani_ranch': 'Romani Ranch',
    'milk_road': 'Milk Road',
    'doggy_racetrack': 'Doggy Racetrack',
    'cucco_shack': 'Cucco Shack',
    'gorman_track': 'Gorman Track',
    // MM Moon
    'moon_trial_deku': 'Moon Trial (Deku)',
    'moon_trial_goron': 'Moon Trial (Goron)',
    'moon_trial_zora': 'Moon Trial (Zora)',
    'moon_trial_link': 'Moon Trial (Link)',
    'moon': 'The Moon'
};

// Pre-sorted region keys for efficient matching (longest first)
const SORTED_REGION_KEYS = Object.keys(REGION_NAMES).sort((a, b) => b.length - a.length);

/**
 * Mapping from OoT scene IDs to region keys.
 * Scene IDs are from game_detection.rs.
 */
const OOT_SCENE_TO_REGION = {
    // Dungeons
    0x00: 'deku_tree',           // Deku Tree
    0x01: 'dodongo_cavern',      // Dodongo's Cavern
    0x02: 'jabu_jabu',           // Jabu Jabu's Belly
    0x03: 'forest_temple',       // Forest Temple
    0x04: 'fire_temple',         // Fire Temple
    0x05: 'water_temple',        // Water Temple
    0x06: 'spirit_temple',       // Spirit Temple
    0x07: 'shadow_temple',       // Shadow Temple
    0x08: 'bottom_of_the_well',  // Bottom of the Well
    0x09: 'ice_cavern',          // Ice Cavern
    0x0A: 'ganon_castle',        // Ganon's Castle Tower
    0x0B: 'gerudo_training',     // Gerudo Training Ground
    0x0C: 'gerudo_fortress',     // Thieves' Hideout
    0x0D: 'ganon_castle',        // Ganon's Castle

    // Interior locations
    0x2D: 'market',              // Happy Mask Shop
    0x43: 'temple_of_time',      // Temple of Time Exterior

    // Overworld areas
    0x51: 'hf',                  // Hyrule Field
    0x52: 'kak',                 // Kakariko Village
    0x53: 'graveyard',           // Graveyard
    0x54: 'zr',                  // Zora's River
    0x55: 'kokiri_forest',       // Kokiri Forest
    0x57: 'lake_hylia',          // Lake Hylia
    0x58: 'zora_domain',         // Zora's Domain
    0x59: 'zora_fountain',       // Zora's Fountain
    0x5A: 'gerudo_valley',       // Gerudo Valley
    0x5B: 'lw',                  // Lost Woods
    0x5C: 'desert_colossus',     // Desert Colossus
    0x5D: 'gerudo_fortress',     // Gerudo Fortress
    0x5E: 'haunted_wasteland',   // Haunted Wasteland
    0x60: 'dmt',                 // Death Mountain Trail
    0x61: 'dmc',                 // Death Mountain Crater
    0x62: 'goron_city',          // Goron City

    // Additional interior scenes
    0x10: 'market',              // Market Day
    0x1D: 'lon_lon_ranch',       // Lon Lon Ranch
    0x34: 'hyrule_castle',       // Hyrule Castle
    0x42: 'temple_of_time',      // Temple of Time
    0x56: 'sfm',                 // Sacred Forest Meadow
};

/**
 * Mapping from MM scene IDs to region keys.
 * Scene IDs are from game_detection.rs.
 */
const MM_SCENE_TO_REGION = {
    // Main Dungeons
    0x07: 'woodfall_temple',     // Woodfall Temple
    0x1B: 'snowhead_temple',     // Snowhead Temple
    0x37: 'great_bay_temple',    // Great Bay Temple
    0x12: 'stone_tower_temple',  // Stone Tower Temple
    0x13: 'stone_tower_temple_inverted', // Stone Tower Temple (Inverted)
    0x01: 'moon',                // Majora's Lair

    // Clock Town
    0x6E: 'clock_town',          // Clock Town South
    0x6F: 'clock_town',          // Clock Town North
    0x70: 'clock_town',          // Clock Town East
    0x71: 'clock_town',          // Clock Town West
    0x6C: 'clock_town',          // Clock Tower

    // Overworld
    0x54: 'termina_field',       // Termina Field
    0x35: 'romani_ranch',        // Romani Ranch
    0x55: 'southern_swamp',      // Southern Swamp
    0x5A: 'mountain_village',    // Mountain Village
    0x57: 'great_bay_coast',     // Great Bay Coast
    0x5B: 'ikana_canyon',        // Ikana Canyon

    // Mini Dungeons
    0x02: 'beneath_the_graveyard', // Beneath the Graveyard
    0x00: 'mayors_office',       // Mayor's Residence
};

/**
 * Get the region key for a given scene ID and game.
 * @param {number} sceneId - The scene ID from RAM
 * @param {string} game - Either 'oot' or 'mm'
 * @returns {string|null} The region key, or null if not found
 */
function getRegionFromSceneId(sceneId, game) {
    if (game === 'mm') {
        return MM_SCENE_TO_REGION[sceneId] || null;
    }
    return OOT_SCENE_TO_REGION[sceneId] || null;
}

/**
 * Get the current room name from the URL
 * Supports both /room/<name> and /room/<name>/<layout> patterns
 */
function getRoomName() {
    const match = window.location.pathname.match(/^\/room\/([0-9A-Za-z-]+)(?:\/[0-9A-Za-z-]+)?\/?$/);
    return match ? match[1] : null;
}

/**
 * Extract region from a location ID.
 * Location IDs are in format: oot_<region>_<location_specific>
 * Region may be 1-3 words separated by underscores.
 */
function extractRegion(locationId) {
    // Remove 'oot_' or 'mm_' prefix
    const withoutPrefix = locationId.replace(/^(oot|mm)_/, '');

    // Try to match known regions (longest match first, using pre-sorted keys)
    for (const region of SORTED_REGION_KEYS) {
        if (withoutPrefix.startsWith(region + '_') || withoutPrefix === region) {
            return region;
        }
    }

    // Fallback: use first two words as region guess
    const parts = withoutPrefix.split('_');
    if (parts.length >= 2) {
        // Try two-word region first
        const twoWord = parts.slice(0, 2).join('_');
        if (REGION_NAMES[twoWord]) {
            return twoWord;
        }
        // Try three-word region
        if (parts.length >= 3) {
            const threeWord = parts.slice(0, 3).join('_');
            if (REGION_NAMES[threeWord]) {
                return threeWord;
            }
        }
        // Return first word as fallback
        return parts[0];
    }

    return 'unknown';
}

/**
 * Get human-readable region name
 */
function getRegionDisplayName(regionKey) {
    return REGION_NAMES[regionKey] || formatRegionKey(regionKey);
}

/**
 * Format a region key into a readable name (fallback)
 */
function formatRegionKey(key) {
    return key
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

/**
 * Format a location name for display
 */
function formatLocationName(locationId) {
    // Remove prefix and region
    const withoutPrefix = locationId.replace(/^(oot|mm)_/, '');
    const region = extractRegion(locationId);

    // Remove the region prefix from the location name
    let name = withoutPrefix;
    if (withoutPrefix.startsWith(region + '_')) {
        name = withoutPrefix.slice(region.length + 1);
    }

    // Format the remaining name
    return name
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

/**
 * Fetch checked locations from the API
 */
async function fetchCheckedLocations() {
    const roomName = getRoomName();
    if (!roomName) {
        return null;
    }

    try {
        checkedLocationsState.isLoading = true;
        const response = await fetch(`/api/room/${encodeURIComponent(roomName)}/checked-locations`);

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }

        const data = await response.json();
        checkedLocationsState.locations = {};
        checkedLocationsState.logic = {};

        // Build maps of location_id -> { status, accessibility } and location_id -> logic
        for (const loc of data.locations) {
            checkedLocationsState.locations[loc.location_id] = {
                status: loc.status,
                accessibility: loc.accessibility || 'Unknown'
            };
            if (loc.logic) {
                checkedLocationsState.logic[loc.location_id] = loc.logic;
            }
        }

        checkedLocationsState.lastUpdate = new Date();
        checkedLocationsState.error = null;
        checkedLocationsState.isLoading = false;

        return data;
    } catch (error) {
        checkedLocationsState.error = error.message;
        checkedLocationsState.isLoading = false;
        console.error('Failed to fetch checked locations:', error);
        return null;
    }
}

/**
 * Update the status display
 */
function updateStatusDisplay(data) {
    if (!statusElement) return;

    if (data) {
        // Done includes both checked and skipped
        const doneCount = data.checked_count + (data.skipped_count || 0);
        const percent = data.total_mapped > 0
            ? Math.round((doneCount / data.total_mapped) * 100)
            : 0;
        let html = `
            <span class="checked-count">${data.checked_count}</span>`;
        // Show skipped count if any locations are skipped
        if (data.skipped_count && data.skipped_count > 0) {
            html += `<span class="skipped-indicator"> (+${data.skipped_count} skipped)</span>`;
        }
        html += `
            <span class="checked-separator">/</span>
            <span class="total-count">${data.total_mapped}</span>
            <span class="checked-percent">(${percent}%)</span>
        `;
        // Show available count if there are accessible unchecked locations
        if (data.available_count !== undefined && data.available_count > 0) {
            html += `<span class="available-indicator"> [${data.available_count} available]</span>`;
        }
        statusElement.innerHTML = html;
        statusElement.classList.remove('error');
    } else if (checkedLocationsState.error) {
        statusElement.innerHTML = `<span class="error-text">Error: ${checkedLocationsState.error}</span>`;
        statusElement.classList.add('error');
    }
}

/**
 * Create the status display element
 */
function createStatusElement() {
    if (statusElement) return statusElement;

    const container = document.createElement('div');
    container.id = 'checked-locations-status';
    container.className = 'checked-locations-status';
    container.innerHTML = '<span class="loading">Loading...</span>';

    // Insert after the tracker container or at the top of body
    const tracker = document.querySelector('.items');
    if (tracker) {
        tracker.parentNode.insertBefore(container, tracker);
    } else {
        document.body.insertBefore(container, document.body.firstChild);
    }

    statusElement = container;
    return container;
}

/**
 * Toggle the hide unavailable filter
 */
function toggleHideUnavailable() {
    checkedLocationsState.hideUnavailable = !checkedLocationsState.hideUnavailable;
    // Save preference to localStorage
    localStorage.setItem('oottracker_hide_unavailable', checkedLocationsState.hideUnavailable);
    // Re-render the list
    const data = {
        locations: Object.entries(checkedLocationsState.locations).map(([id, locData]) => ({
            location_id: id,
            status: locData.status,
            accessibility: locData.accessibility
        }))
    };
    updateLocationsList(data);
    // Update checkbox state
    const checkbox = document.getElementById('hide-unavailable-checkbox');
    if (checkbox) {
        checkbox.checked = checkedLocationsState.hideUnavailable;
    }
}

/**
 * Toggle the hide MQ locations filter
 */
function toggleHideMqLocations() {
    checkedLocationsState.hideMqLocations = !checkedLocationsState.hideMqLocations;
    // Save preference to localStorage
    localStorage.setItem('oottracker_hide_mq_locations', checkedLocationsState.hideMqLocations);
    // Re-render the list
    const data = {
        locations: Object.entries(checkedLocationsState.locations).map(([id, locData]) => ({
            location_id: id,
            status: locData.status,
            accessibility: locData.accessibility
        }))
    };
    updateLocationsList(data);
    // Update checkbox state
    const checkbox = document.getElementById('hide-mq-checkbox');
    if (checkbox) {
        checkbox.checked = checkedLocationsState.hideMqLocations;
    }
}

/**
 * Check if a location ID represents a Master Quest dungeon check.
 * MQ locations use the "mq_oot_mq_" prefix to distinguish from vanilla.
 */
function isMqLocation(locationId) {
    return locationId.startsWith('mq_oot_mq_');
}

/**
 * Check if a location should be hidden based on current filter settings
 */
function shouldHideLocation(locationId, status) {
    // Check MQ filter first (applies regardless of check status)
    if (checkedLocationsState.hideMqLocations && isMqLocation(locationId)) {
        return true;
    }

    if (!checkedLocationsState.hideUnavailable) {
        return false;
    }
    // If location is already checked or skipped, never hide it
    if (status === 'Checked' || status === 'Skipped') {
        return false;
    }
    // Check accessibility status from the location data
    const locData = checkedLocationsState.locations[locationId];
    const accessibility = locData ? locData.accessibility : null;
    // Hide if explicitly unavailable
    if (accessibility === 'Unavailable') {
        return true;
    }
    // Show if accessible, unknown, or no accessibility data
    return false;
}

/**
 * Create the checked locations panel
 */
function createCheckedLocationsPanel() {
    const panel = document.createElement('div');
    panel.id = 'checked-locations-panel';
    panel.className = 'checked-locations-panel collapsed';

    // Create header container for toggle button and auto-scroll control
    const header = document.createElement('div');
    header.className = 'panel-header';

    const toggle = document.createElement('button');
    toggle.className = 'panel-toggle';
    toggle.textContent = 'Checked Locations';
    toggle.addEventListener('click', () => {
        panel.classList.toggle('collapsed');
        toggle.textContent = panel.classList.contains('collapsed')
            ? 'Checked Locations'
            : 'Hide Locations';
    });

    // Auto-scroll toggle button
    const autoScrollBtn = document.createElement('button');
    autoScrollBtn.id = 'autoscroll-toggle-btn';
    autoScrollBtn.className = 'autoscroll-toggle' +
        (checkedLocationsState.autoScrollEnabled ? ' enabled' : ' disabled');
    autoScrollBtn.innerHTML = '&#8645;'; // Up-down arrow symbol
    autoScrollBtn.title = checkedLocationsState.autoScrollEnabled
        ? 'Auto-scroll enabled (click to disable)'
        : 'Auto-scroll disabled (click to enable)';
    autoScrollBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleAutoScroll();
    });

    header.appendChild(toggle);
    header.appendChild(autoScrollBtn);

    // Filter controls
    const filterControls = document.createElement('div');
    filterControls.className = 'filter-controls';
    filterControls.id = 'locations-filter-controls';

    const filterLabel = document.createElement('label');
    filterLabel.className = 'filter-checkbox-label';

    const filterCheckbox = document.createElement('input');
    filterCheckbox.type = 'checkbox';
    filterCheckbox.id = 'hide-unavailable-checkbox';
    filterCheckbox.checked = checkedLocationsState.hideUnavailable;
    filterCheckbox.addEventListener('change', toggleHideUnavailable);

    const filterText = document.createElement('span');
    filterText.textContent = 'Hide unavailable';
    filterText.title = 'Hide locations that are currently inaccessible based on items/logic';

    filterLabel.appendChild(filterCheckbox);
    filterLabel.appendChild(filterText);
    filterControls.appendChild(filterLabel);

    // Hide MQ locations checkbox
    const mqLabel = document.createElement('label');
    mqLabel.className = 'filter-checkbox-label';

    const mqCheckbox = document.createElement('input');
    mqCheckbox.type = 'checkbox';
    mqCheckbox.id = 'hide-mq-checkbox';
    mqCheckbox.checked = checkedLocationsState.hideMqLocations;
    mqCheckbox.addEventListener('change', toggleHideMqLocations);

    const mqText = document.createElement('span');
    mqText.textContent = 'Hide MQ checks';
    mqText.title = 'Hide Master Quest dungeon checks (for vanilla dungeon playthroughs)';

    mqLabel.appendChild(mqCheckbox);
    mqLabel.appendChild(mqText);
    filterControls.appendChild(mqLabel);

    const list = document.createElement('div');
    list.className = 'locations-list';
    list.id = 'checked-locations-list';

    // Event delegation for region toggle buttons and skip buttons (prevents XSS from inline onclick)
    list.addEventListener('click', (e) => {
        // Handle region header clicks
        const header = e.target.closest('.region-header');
        if (header) {
            const regionGroup = header.closest('.region-group');
            if (regionGroup && regionGroup.dataset.region) {
                toggleRegion(regionGroup.dataset.region);
            }
            return;
        }

        // Handle skip button clicks
        const skipButton = e.target.closest('.skip-button');
        if (skipButton) {
            const locationItem = skipButton.closest('.location-item');
            if (locationItem && locationItem.dataset.location) {
                toggleSkipLocation(locationItem.dataset.location);
            }
            return;
        }

        // Handle unskip button clicks
        const unskipButton = e.target.closest('.unskip-button');
        if (unskipButton) {
            const locationItem = unskipButton.closest('.location-item');
            if (locationItem && locationItem.dataset.location) {
                toggleSkipLocation(locationItem.dataset.location);
            }
            return;
        }
    });

    panel.appendChild(header);
    panel.appendChild(filterControls);
    panel.appendChild(list);

    // Insert before footer
    const footer = document.querySelector('footer');
    if (footer) {
        footer.parentNode.insertBefore(panel, footer);
    } else {
        document.body.appendChild(panel);
    }

    return panel;
}

/**
 * Toggle region collapse state
 */
function toggleRegion(regionKey) {
    if (checkedLocationsState.collapsedRegions.has(regionKey)) {
        checkedLocationsState.collapsedRegions.delete(regionKey);
    } else {
        checkedLocationsState.collapsedRegions.add(regionKey);
    }
    // Re-render
    const data = {
        locations: Object.entries(checkedLocationsState.locations).map(([id, info]) => ({
            location_id: id,
            status: info.status,
            accessibility: info.accessibility
        }))
    };
    updateLocationsList(data);
}

/**
 * Toggle skip state for a location
 */
async function toggleSkipLocation(locationId) {
    const roomName = getRoomName();
    if (!roomName) return;

    // Mark as pending
    checkedLocationsState.pendingSkipToggles.add(locationId);

    try {
        const response = await fetch(`/api/room/${encodeURIComponent(roomName)}/toggle-skip`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ location_id: locationId })
        });

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }

        const result = await response.json();
        // Update local state immediately (preserve accessibility)
        const currentInfo = checkedLocationsState.locations[locationId] || { accessibility: 'Unknown' };
        checkedLocationsState.locations[locationId] = {
            status: result.skipped ? 'Skipped' : 'Unchecked',
            accessibility: currentInfo.accessibility
        };

        // Refresh the display
        const data = {
            locations: Object.entries(checkedLocationsState.locations).map(([id, info]) => ({
                location_id: id,
                status: info.status,
                accessibility: info.accessibility
            }))
        };
        updateLocationsList(data);
    } catch (error) {
        console.error('Failed to toggle skip state:', error);
    } finally {
        checkedLocationsState.pendingSkipToggles.delete(locationId);
    }
}

/**
 * Update the locations list display with region-based grouping
 */
function updateLocationsList(data) {
    const list = document.getElementById('checked-locations-list');
    if (!list || !data) return;

    // Group locations by region
    const regionGroups = {};

    for (const loc of data.locations) {
        // Check if location should be hidden based on filter
        if (shouldHideLocation(loc.location_id, loc.status)) {
            continue;
        }

        const region = extractRegion(loc.location_id);
        if (!regionGroups[region]) {
            regionGroups[region] = {
                checked: [],
                available: [],    // unchecked and available
                unavailable: [],  // unchecked and unavailable
                unchecked: [],    // unchecked with unknown accessibility
                skipped: [],
                unknown: []
            };
        }

        if (loc.status === 'Checked') {
            regionGroups[region].checked.push(loc.location_id);
        } else if (loc.status === 'Unchecked') {
            // Categorize unchecked by accessibility
            if (loc.accessibility === 'Available') {
                regionGroups[region].available.push(loc.location_id);
            } else if (loc.accessibility === 'Unavailable') {
                regionGroups[region].unavailable.push(loc.location_id);
            } else {
                regionGroups[region].unchecked.push(loc.location_id);
            }
        } else if (loc.status === 'Skipped') {
            regionGroups[region].skipped.push(loc.location_id);
        } else {
            regionGroups[region].unknown.push(loc.location_id);
        }
    }

    // Sort regions by display order, then alphabetically for unknown regions
    const sortedRegions = Object.keys(regionGroups).sort((a, b) => {
        const indexA = REGION_DISPLAY_ORDER.indexOf(a);
        const indexB = REGION_DISPLAY_ORDER.indexOf(b);

        if (indexA !== -1 && indexB !== -1) {
            return indexA - indexB;
        }
        if (indexA !== -1) return -1;
        if (indexB !== -1) return 1;

        return getRegionDisplayName(a).localeCompare(getRegionDisplayName(b));
    });

    let html = '';

    for (const region of sortedRegions) {
        const group = regionGroups[region];
        // Total includes all locations
        const total = group.checked.length + group.available.length + group.unavailable.length +
                      group.unchecked.length + group.skipped.length + group.unknown.length;
        // Progress shows checked + skipped (locations that are "done" either way)
        const done = group.checked.length + group.skipped.length;
        // Count of available checks in this region
        const availableInRegion = group.available.length;
        const isCollapsed = checkedLocationsState.collapsedRegions.has(region);
        const collapsedClass = isCollapsed ? 'collapsed' : '';
        const arrow = isCollapsed ? '&#9654;' : '&#9660;'; // Right arrow or down arrow

        html += `<div class="region-group ${collapsedClass}" data-region="${escapeHtml(region)}">`;
        html += `<button class="region-header" aria-expanded="${!isCollapsed}" aria-controls="region-${escapeHtml(region)}">`;
        html += `<span class="region-arrow">${arrow}</span>`;
        html += `<span class="region-name">${escapeHtml(getRegionDisplayName(region))}</span>`;
        html += `<span class="region-count">(${done}/${total})`;
        if (availableInRegion > 0) {
            html += ` <span class="region-available">[${availableInRegion} avail]</span>`;
        }
        html += `</span>`;
        html += `</button>`;

        html += `<div class="region-locations" id="region-${escapeHtml(region)}">`;

        // Show available checks first (player can get these now!)
        for (const loc of group.available.sort()) {
            const isPending = checkedLocationsState.pendingSkipToggles.has(loc);
            const pendingClass = isPending ? ' pending' : '';
            const tooltip = escapeHtml(getLocationTooltip(loc));
            html += `<div class="location-item location-available${pendingClass}" data-location="${escapeHtml(loc)}" title="${tooltip}">`;
            html += `<span class="location-icon">&#9679;</span>`; // Filled circle (go get it!)
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `<button class="skip-button" title="Skip this location" ${isPending ? 'disabled' : ''}>&#10006;</button>`;
            html += `</div>`;
        }

        // Show unavailable checks (player needs more items)
        for (const loc of group.unavailable.sort()) {
            const isPending = checkedLocationsState.pendingSkipToggles.has(loc);
            const pendingClass = isPending ? ' pending' : '';
            const tooltip = escapeHtml(getLocationTooltip(loc));
            html += `<div class="location-item location-unavailable${pendingClass}" data-location="${escapeHtml(loc)}" title="${tooltip}">`;
            html += `<span class="location-icon">&#9675;</span>`; // Empty circle (can't get yet)
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `<button class="skip-button" title="Skip this location" ${isPending ? 'disabled' : ''}>&#10006;</button>`;
            html += `</div>`;
        }

        // Show unchecked with unknown accessibility
        for (const loc of group.unchecked.sort()) {
            const isPending = checkedLocationsState.pendingSkipToggles.has(loc);
            const pendingClass = isPending ? ' pending' : '';
            const tooltip = escapeHtml(getLocationTooltip(loc));
            html += `<div class="location-item location-unchecked${pendingClass}" data-location="${escapeHtml(loc)}" title="${tooltip}">`;
            html += `<span class="location-icon">&#9744;</span>`; // Empty checkbox
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `<button class="skip-button" title="Skip this location" ${isPending ? 'disabled' : ''}>&#10006;</button>`;
            html += `</div>`;
        }

        // Then skipped (user decided to skip)
        for (const loc of group.skipped.sort()) {
            const isPending = checkedLocationsState.pendingSkipToggles.has(loc);
            const pendingClass = isPending ? ' pending' : '';
            const tooltip = escapeHtml(getLocationTooltip(loc));
            html += `<div class="location-item location-skipped${pendingClass}" data-location="${escapeHtml(loc)}" title="${tooltip}">`;
            html += `<span class="location-icon">&#10060;</span>`; // Red X to indicate skipped
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `<button class="unskip-button" title="Unskip this location" ${isPending ? 'disabled' : ''}>&#8634;</button>`;
            html += `</div>`;
        }

        // Then checked (completed)
        for (const loc of group.checked.sort()) {
            const tooltip = escapeHtml(getLocationTooltip(loc));
            html += `<div class="location-item location-checked" title="${tooltip}">`;
            html += `<span class="location-icon">&#9745;</span>`; // Checked checkbox
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `</div>`;
        }

        // Unknown status
        for (const loc of group.unknown.sort()) {
            const tooltip = escapeHtml(getLocationTooltip(loc));
            html += `<div class="location-item location-unknown" title="${tooltip}">`;
            html += `<span class="location-icon">?</span>`;
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `</div>`;
        }

        html += `</div>`; // region-locations
        html += `</div>`; // region-group
    }

    list.innerHTML = html;
}

/**
 * Get the tooltip text for a location (shows logic requirements)
 */
function getLocationTooltip(locationId) {
    const logic = checkedLocationsState.logic[locationId];
    if (!logic || logic === 'true') {
        return 'No special requirements';
    }
    return `Requires: ${logic}`;
}

/**
 * Escape HTML special characters
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

/**
 * Scroll the check tracker panel to show a specific region.
 * @param {string} regionKey - The region key to scroll to
 */
function scrollToRegion(regionKey) {
    const panel = document.getElementById('checked-locations-panel');
    const list = document.getElementById('checked-locations-list');
    if (!panel || !list) return;

    // Find the region group element
    const regionGroup = list.querySelector(`[data-region="${regionKey}"]`);
    if (!regionGroup) return;

    // Expand the panel if collapsed
    if (panel.classList.contains('collapsed')) {
        panel.classList.remove('collapsed');
        const toggle = panel.querySelector('.panel-toggle');
        if (toggle) {
            toggle.textContent = 'Hide Locations';
        }
    }

    // Expand the region if collapsed
    if (checkedLocationsState.collapsedRegions.has(regionKey)) {
        checkedLocationsState.collapsedRegions.delete(regionKey);
        regionGroup.classList.remove('collapsed');
        const header = regionGroup.querySelector('.region-header');
        if (header) {
            header.setAttribute('aria-expanded', 'true');
        }
        const arrow = regionGroup.querySelector('.region-arrow');
        if (arrow) {
            arrow.innerHTML = '&#9660;'; // Down arrow
        }
    }

    // Scroll the region into view with smooth animation
    regionGroup.scrollIntoView({ behavior: 'smooth', block: 'start' });

    // Add a brief highlight effect
    regionGroup.classList.add('auto-scroll-highlight');
    setTimeout(() => {
        regionGroup.classList.remove('auto-scroll-highlight');
    }, 2000);
}

/**
 * Handle scene change detection and auto-scroll.
 * @param {object} data - The API response data containing current_scene_id and current_game
 */
function handleSceneChange(data) {
    if (!data || !checkedLocationsState.autoScrollEnabled) return;

    const newSceneId = data.current_scene_id;
    const newGame = data.current_game || 'oot';

    // Skip if scene hasn't changed
    if (newSceneId === checkedLocationsState.lastSceneId &&
        newGame === checkedLocationsState.lastGame) {
        return;
    }

    // Skip invalid scene IDs
    if (newSceneId === null || newSceneId === undefined || newSceneId === 0xFF) {
        return;
    }

    // Update tracked state
    const oldSceneId = checkedLocationsState.lastSceneId;
    checkedLocationsState.lastSceneId = newSceneId;
    checkedLocationsState.lastGame = newGame;

    // Don't scroll on first load (when oldSceneId is null)
    if (oldSceneId === null) {
        return;
    }

    // Get the region for the new scene
    const regionKey = getRegionFromSceneId(newSceneId, newGame);
    if (regionKey) {
        // Small delay to let the UI update first
        setTimeout(() => {
            scrollToRegion(regionKey);
        }, 100);
    }
}

/**
 * Toggle auto-scroll feature on/off
 */
function toggleAutoScroll() {
    checkedLocationsState.autoScrollEnabled = !checkedLocationsState.autoScrollEnabled;
    localStorage.setItem('oottracker_autoscroll_enabled',
        checkedLocationsState.autoScrollEnabled ? 'true' : 'false');
    updateAutoScrollButtonState();

    // Start or stop scene polling based on new state
    if (checkedLocationsState.autoScrollEnabled) {
        startScenePolling();
    } else {
        stopScenePolling();
    }
}

/**
 * Update the auto-scroll button visual state
 */
function updateAutoScrollButtonState() {
    const btn = document.getElementById('autoscroll-toggle-btn');
    if (btn) {
        if (checkedLocationsState.autoScrollEnabled) {
            btn.classList.add('enabled');
            btn.classList.remove('disabled');
            btn.title = 'Auto-scroll enabled (click to disable)';
        } else {
            btn.classList.remove('enabled');
            btn.classList.add('disabled');
            btn.title = 'Auto-scroll disabled (click to enable)';
        }
    }
}

/**
 * Refresh checked locations and update display
 */
async function refreshCheckedLocations() {
    const data = await fetchCheckedLocations();
    updateStatusDisplay(data);
    updateLocationsList(data);

    // Handle auto-scroll on scene change
    handleSceneChange(data);

    return data;
}

/**
 * Lightweight scene check - only fetches scene info for auto-scroll.
 * This is used for periodic polling to detect scene changes when
 * WebSocket doesn't trigger updates (e.g., walking between areas
 * without collecting items).
 */
async function checkSceneForAutoScroll() {
    if (!checkedLocationsState.autoScrollEnabled) return;

    try {
        const roomName = getRoomName();
        if (!roomName) return;

        const response = await fetch(`/api/room/${roomName}/checked-locations`);
        if (!response.ok) return;

        const data = await response.json();
        handleSceneChange(data);
    } catch (e) {
        // Silent fail for background polling
        console.debug('Scene check failed:', e);
    }
}

// Scene polling interval ID
let scenePollingInterval = null;

/**
 * Start periodic scene polling for auto-scroll.
 * Polls every 2 seconds to detect scene changes even when
 * WebSocket doesn't send updates.
 */
function startScenePolling() {
    if (scenePollingInterval) return; // Already polling

    scenePollingInterval = setInterval(checkSceneForAutoScroll, 2000);
}

/**
 * Stop periodic scene polling.
 */
function stopScenePolling() {
    if (scenePollingInterval) {
        clearInterval(scenePollingInterval);
        scenePollingInterval = null;
    }
}

/**
 * Initialize the checked locations display
 */
function initCheckedLocations() {
    const roomName = getRoomName();
    if (!roomName) {
        // Not on a room page, skip initialization
        return;
    }

    // Load saved filter preferences from localStorage
    const savedHideUnavailable = localStorage.getItem('oottracker_hide_unavailable');
    if (savedHideUnavailable !== null) {
        checkedLocationsState.hideUnavailable = savedHideUnavailable === 'true';
    }

    const savedHideMq = localStorage.getItem('oottracker_hide_mq_locations');
    if (savedHideMq !== null) {
        checkedLocationsState.hideMqLocations = savedHideMq === 'true';
    }

    // Create UI elements
    createStatusElement();
    createCheckedLocationsPanel();

    // Initial fetch
    refreshCheckedLocations();

    // Listen for WebSocket state changes from proto.js instead of polling
    // This provides immediate updates when the tracker state changes
    window.addEventListener('trackerStateChanged', function(event) {
        // Refresh checked locations when tracker state changes
        refreshCheckedLocations();
    });

    // Start periodic scene polling for auto-scroll
    // This ensures scene changes are detected even when WebSocket
    // doesn't trigger updates (e.g., walking between areas)
    if (checkedLocationsState.autoScrollEnabled) {
        startScenePolling();
    }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initCheckedLocations);
} else {
    initCheckedLocations();
}

// Export for external use
window.checkedLocations = {
    refresh: refreshCheckedLocations,
    getState: () => checkedLocationsState,
    isLocationChecked: (locationId) => {
        const info = checkedLocationsState.locations[locationId];
        return info && info.status === 'Checked';
    },
    isLocationSkipped: (locationId) => {
        const info = checkedLocationsState.locations[locationId];
        return info && info.status === 'Skipped';
    },
    isLocationAvailable: (locationId) => {
        const info = checkedLocationsState.locations[locationId];
        return info && info.accessibility === 'Available';
    },
    toggleSkip: toggleSkipLocation,
    // Auto-scroll API
    scrollToRegion: scrollToRegion,
    toggleAutoScroll: toggleAutoScroll,
    isAutoScrollEnabled: () => checkedLocationsState.autoScrollEnabled,
    getRegionFromSceneId: getRegionFromSceneId
};
