/**
 * Checked Locations Display Module
 *
 * This module fetches and displays the checked locations status
 * from the API endpoint and updates the UI accordingly.
 * Locations are grouped by region with collapsible sections.
 */

// Checked locations state
let checkedLocationsState = {
    locations: {},
    lastUpdate: null,
    isLoading: false,
    error: null,
    collapsedRegions: new Set(), // Track which regions are collapsed
    pendingSkipToggles: new Set() // Track locations with pending skip toggle requests
};

// Status display element
let statusElement = null;

/**
 * Region name mapping from location ID prefixes to human-readable names.
 * The order here determines display order (dungeons first, then overworld).
 */
const REGION_DISPLAY_ORDER = [
    // Child Dungeons
    'deku_tree',
    'dodongo_cavern',
    'jabu_jabu',
    // Adult Dungeons
    'forest_temple',
    'fire_temple',
    'water_temple',
    'spirit_temple',
    'shadow_temple',
    // Mini Dungeons
    'bottom_of_the_well',
    'ice_cavern',
    'gerudo_training',
    'ganon_castle',
    'treasure_chest_game',
    // Overworld - Kokiri/Forest
    'kokiri_forest',
    'kf',
    'lw',
    'sfm',
    // Overworld - Hyrule
    'hf',
    'lon_lon_ranch',
    'market',
    'hyrule_castle',
    'temple_of_time',
    // Overworld - Kakariko
    'kak',
    'graveyard',
    // Overworld - Death Mountain
    'dmt',
    'death_mountain_trail',
    'goron_city',
    'dmc',
    'death_mountain_crater',
    // Overworld - Zora
    'zr',
    'zora_domain',
    'zora_fountain',
    'lake_hylia',
    // Overworld - Gerudo
    'gerudo_valley',
    'gerudo_fortress',
    'haunted_wasteland',
    'desert_colossus'
];

const REGION_NAMES = {
    // Child Dungeons
    'deku_tree': 'Deku Tree',
    'dodongo_cavern': "Dodongo's Cavern",
    'jabu_jabu': "Jabu Jabu's Belly",
    // Adult Dungeons
    'forest_temple': 'Forest Temple',
    'fire_temple': 'Fire Temple',
    'water_temple': 'Water Temple',
    'spirit_temple': 'Spirit Temple',
    'shadow_temple': 'Shadow Temple',
    // Mini Dungeons
    'bottom_of_the_well': 'Bottom of the Well',
    'ice_cavern': 'Ice Cavern',
    'gerudo_training': 'Gerudo Training Ground',
    'ganon_castle': "Ganon's Castle",
    'treasure_chest_game': 'Treasure Chest Game',
    // Overworld - Kokiri/Forest
    'kokiri_forest': 'Kokiri Forest',
    'kf': 'Kokiri Forest',
    'lw': 'Lost Woods',
    'sfm': 'Sacred Forest Meadow',
    // Overworld - Hyrule
    'hf': 'Hyrule Field',
    'lon_lon_ranch': 'Lon Lon Ranch',
    'market': 'Market',
    'hyrule_castle': 'Hyrule Castle',
    'temple_of_time': 'Temple of Time',
    // Overworld - Kakariko
    'kak': 'Kakariko Village',
    'graveyard': 'Graveyard',
    // Overworld - Death Mountain
    'dmt': 'Death Mountain Trail',
    'death_mountain_trail': 'Death Mountain Trail',
    'goron_city': 'Goron City',
    'dmc': 'Death Mountain Crater',
    'death_mountain_crater': 'Death Mountain Crater',
    // Overworld - Zora
    'zr': "Zora's River",
    'zora_domain': "Zora's Domain",
    'zora_fountain': "Zora's Fountain",
    'lake_hylia': 'Lake Hylia',
    // Overworld - Gerudo
    'gerudo_valley': 'Gerudo Valley',
    'gerudo_fortress': 'Gerudo Fortress',
    'haunted_wasteland': 'Haunted Wasteland',
    'desert_colossus': 'Desert Colossus'
};

// Pre-sorted region keys for efficient matching (longest first)
const SORTED_REGION_KEYS = Object.keys(REGION_NAMES).sort((a, b) => b.length - a.length);

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

        // Build a map of location_id -> status
        for (const loc of data.locations) {
            checkedLocationsState.locations[loc.location_id] = loc.status;
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
 * Create the checked locations panel
 */
function createCheckedLocationsPanel() {
    const panel = document.createElement('div');
    panel.id = 'checked-locations-panel';
    panel.className = 'checked-locations-panel collapsed';

    const toggle = document.createElement('button');
    toggle.className = 'panel-toggle';
    toggle.textContent = 'Checked Locations';
    toggle.addEventListener('click', () => {
        panel.classList.toggle('collapsed');
        toggle.textContent = panel.classList.contains('collapsed')
            ? 'Checked Locations'
            : 'Hide Locations';
    });

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

    panel.appendChild(toggle);
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
        locations: Object.entries(checkedLocationsState.locations).map(([id, status]) => ({
            location_id: id,
            status: status
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
        // Update local state immediately
        checkedLocationsState.locations[locationId] = result.skipped ? 'Skipped' : 'Unchecked';

        // Refresh the display
        const data = {
            locations: Object.entries(checkedLocationsState.locations).map(([id, status]) => ({
                location_id: id,
                status: status
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
        const region = extractRegion(loc.location_id);
        if (!regionGroups[region]) {
            regionGroups[region] = {
                checked: [],
                unchecked: [],
                skipped: [],
                unknown: []
            };
        }

        if (loc.status === 'Checked') {
            regionGroups[region].checked.push(loc.location_id);
        } else if (loc.status === 'Unchecked') {
            regionGroups[region].unchecked.push(loc.location_id);
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
        // Total includes checked, unchecked, skipped, and unknown
        const total = group.checked.length + group.unchecked.length + group.skipped.length + group.unknown.length;
        // Progress shows checked + skipped (locations that are "done" either way)
        const done = group.checked.length + group.skipped.length;
        const isCollapsed = checkedLocationsState.collapsedRegions.has(region);
        const collapsedClass = isCollapsed ? 'collapsed' : '';
        const arrow = isCollapsed ? '&#9654;' : '&#9660;'; // Right arrow or down arrow

        html += `<div class="region-group ${collapsedClass}" data-region="${escapeHtml(region)}">`;
        html += `<button class="region-header" aria-expanded="${!isCollapsed}" aria-controls="region-${escapeHtml(region)}">`;
        html += `<span class="region-arrow">${arrow}</span>`;
        html += `<span class="region-name">${escapeHtml(getRegionDisplayName(region))}</span>`;
        html += `<span class="region-count">(${done}/${total})</span>`;
        html += `</button>`;

        html += `<div class="region-locations" id="region-${escapeHtml(region)}">`;

        // Show unchecked first (what the player still needs to get)
        for (const loc of group.unchecked.sort()) {
            const isPending = checkedLocationsState.pendingSkipToggles.has(loc);
            const pendingClass = isPending ? ' pending' : '';
            html += `<div class="location-item location-unchecked${pendingClass}" data-location="${escapeHtml(loc)}">`;
            html += `<span class="location-icon">&#9744;</span>`; // Empty checkbox
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `<button class="skip-button" title="Skip this location" ${isPending ? 'disabled' : ''}>&#10006;</button>`; // X mark to skip
            html += `</div>`;
        }

        // Then skipped (user decided to skip)
        for (const loc of group.skipped.sort()) {
            const isPending = checkedLocationsState.pendingSkipToggles.has(loc);
            const pendingClass = isPending ? ' pending' : '';
            html += `<div class="location-item location-skipped${pendingClass}" data-location="${escapeHtml(loc)}">`;
            html += `<span class="location-icon">&#10060;</span>`; // Red X to indicate skipped
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `<button class="unskip-button" title="Unskip this location" ${isPending ? 'disabled' : ''}>&#8634;</button>`; // Undo/refresh icon
            html += `</div>`;
        }

        // Then checked (completed)
        for (const loc of group.checked.sort()) {
            html += `<div class="location-item location-checked">`;
            html += `<span class="location-icon">&#9745;</span>`; // Checked checkbox
            html += `<span class="location-name">${escapeHtml(formatLocationName(loc))}</span>`;
            html += `</div>`;
        }

        // Unknown status
        for (const loc of group.unknown.sort()) {
            html += `<div class="location-item location-unknown">`;
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
 * Escape HTML special characters
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

/**
 * Refresh checked locations and update display
 */
async function refreshCheckedLocations() {
    const data = await fetchCheckedLocations();
    updateStatusDisplay(data);
    updateLocationsList(data);
    return data;
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

    // Create UI elements
    createStatusElement();
    createCheckedLocationsPanel();

    // Initial fetch
    refreshCheckedLocations();

    // Set up periodic refresh (every 5 seconds)
    setInterval(refreshCheckedLocations, 5000);
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
    isLocationChecked: (locationId) => checkedLocationsState.locations[locationId] === 'Checked',
    isLocationSkipped: (locationId) => checkedLocationsState.locations[locationId] === 'Skipped',
    toggleSkip: toggleSkipLocation
};
