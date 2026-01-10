/**
 * Location Filter Module
 *
 * Search and filter component for tracker locations.
 * Fetches data from the real checked-locations API and derives
 * game/region/type metadata from location IDs.
 *
 * Note: REGION_NAMES and SORTED_REGION_KEYS are defined in checked-locations.js
 * which is loaded before this file.
 */

// ============================================================================
// Type Inference Patterns
// ============================================================================

/**
 * Patterns for inferring location type from location_id.
 * Ordered by specificity (more specific patterns first).
 */
const TYPE_PATTERNS = [
    { pattern: /_gs_/, type: 'Skulltula' },
    { pattern: /_chest$/, type: 'Chest' },
    { pattern: /_chest_/, type: 'Chest' },
    { pattern: /_cow$/, type: 'Cow' },
    { pattern: /_cow_/, type: 'Cow' },
    { pattern: /_scrub/, type: 'Scrub' },
    { pattern: /_great_fairy/, type: 'Fairy' },
    { pattern: /_stray_fairy/, type: 'StrayFairy' },
    { pattern: /_freestanding/, type: 'Freestanding' },
    { pattern: /_pot_/, type: 'Pot' },
    { pattern: /_beehive/, type: 'Beehive' },
    { pattern: /_rupee/, type: 'Rupee' },
    { pattern: /_heart/, type: 'Heart' },
    { pattern: /_crate/, type: 'Crate' },
    { pattern: /_wonderitem/, type: 'Wonderitem' },
    { pattern: /_bean/, type: 'BeanPlant' },
    { pattern: /_shop_/, type: 'Shop' },
    { pattern: /_song_/, type: 'Song' },
    { pattern: /_mask_/, type: 'Mask' },
];

// ============================================================================
// Filter State
// ============================================================================

let filterState = {
    query: '',
    status: 'all',
    game: 'both',
    type: 'all',
    region: 'all'
};

// Data state
let dataState = {
    locations: [],
    filteredLocations: [],
    isLoading: false,
    error: null
};

// Custom data fetcher (null = use real API)
let customFetchFn = null;

// Debounce timer
let debounceTimer = null;
const DEBOUNCE_MS = 150;

// DOM element references
let containerElement = null;

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Get the current room name from the URL
 * Supports both /room/<name> and /room/<name>/<layout> formats
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
 * Extract game from location ID (OoT or MM)
 */
function extractGame(locationId) {
    if (locationId.startsWith('oot_')) {
        return 'OoT';
    } else if (locationId.startsWith('mm_')) {
        return 'MM';
    }
    return 'Unknown';
}

/**
 * Infer location type from location ID
 */
function inferType(locationId) {
    const lowerCaseId = locationId.toLowerCase();

    for (const { pattern, type } of TYPE_PATTERNS) {
        if (pattern.test(lowerCaseId)) {
            return type;
        }
    }

    return 'Other';
}

/**
 * Transform API response to add derived fields
 */
function transformLocation(apiLocation) {
    const locationId = apiLocation.location_id;
    const regionKey = extractRegion(locationId);

    return {
        location_id: locationId,
        status: apiLocation.status,
        game: extractGame(locationId),
        region: getRegionDisplayName(regionKey),
        type: inferType(locationId),
        is_mapped: apiLocation.is_mapped
    };
}

/**
 * Get unique values for a field from locations
 */
function getUniqueValues(locations, field) {
    const values = new Set(locations.map(loc => loc[field]));
    return Array.from(values).sort();
}

/**
 * Escape HTML special characters
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ============================================================================
// Filter Logic
// ============================================================================

/**
 * Apply filters to locations
 */
function applyFilters(locations) {
    return locations.filter(loc => {
        // Text search (case insensitive)
        if (filterState.query) {
            const query = filterState.query.toLowerCase();
            const searchable = [
                loc.location_id,
                loc.region,
                loc.type,
                loc.game
            ].join(' ').toLowerCase();
            if (!searchable.includes(query)) {
                return false;
            }
        }

        // Status filter
        if (filterState.status !== 'all') {
            if (loc.status.toLowerCase() !== filterState.status.toLowerCase()) {
                return false;
            }
        }

        // Game filter
        if (filterState.game !== 'both') {
            if (loc.game.toLowerCase() !== filterState.game.toLowerCase()) {
                return false;
            }
        }

        // Type filter
        if (filterState.type !== 'all') {
            if (loc.type.toLowerCase() !== filterState.type.toLowerCase()) {
                return false;
            }
        }

        // Region filter
        if (filterState.region !== 'all') {
            if (loc.region !== filterState.region) {
                return false;
            }
        }

        return true;
    });
}

/**
 * Update the filtered results and render
 */
function updateFilteredResults() {
    dataState.filteredLocations = applyFilters(dataState.locations);
    renderResults();
}

/**
 * Debounced search handler
 */
function handleSearchInput(event) {
    const query = event.target.value;

    if (debounceTimer) {
        clearTimeout(debounceTimer);
    }

    debounceTimer = setTimeout(() => {
        filterState.query = query;
        updateFilteredResults();
    }, DEBOUNCE_MS);
}

/**
 * Handle filter dropdown change
 */
function handleFilterChange(filterName, value) {
    filterState[filterName] = value;
    updateFilteredResults();
}

/**
 * Clear all filters
 */
function clearFilters() {
    filterState = {
        query: '',
        status: 'all',
        game: 'both',
        type: 'all',
        region: 'all'
    };

    // Reset UI elements
    const searchInput = document.getElementById('location-search-input');
    if (searchInput) searchInput.value = '';

    const statusSelect = document.getElementById('filter-status');
    if (statusSelect) statusSelect.value = 'all';

    const gameSelect = document.getElementById('filter-game');
    if (gameSelect) gameSelect.value = 'both';

    const typeSelect = document.getElementById('filter-type');
    if (typeSelect) typeSelect.value = 'all';

    const regionSelect = document.getElementById('filter-region');
    if (regionSelect) regionSelect.value = 'all';

    updateFilteredResults();
}

// ============================================================================
// Rendering
// ============================================================================

/**
 * Render the result count
 */
function renderResults() {
    const countElement = document.getElementById('location-filter-count');
    if (countElement) {
        const filtered = dataState.filteredLocations.length;
        const total = dataState.locations.length;
        countElement.textContent = `Showing ${filtered} of ${total} locations`;
    }

    // Dispatch custom event for external listeners
    if (containerElement) {
        containerElement.dispatchEvent(new CustomEvent('filterchange', {
            detail: {
                filters: { ...filterState },
                filteredLocations: dataState.filteredLocations,
                totalLocations: dataState.locations.length
            },
            bubbles: true
        }));
    }
}

/**
 * Create a select dropdown
 */
function createSelect(id, label, options, defaultValue) {
    const wrapper = document.createElement('div');
    wrapper.className = 'filter-select-wrapper';

    const labelEl = document.createElement('label');
    labelEl.htmlFor = id;
    labelEl.textContent = label + ':';

    const select = document.createElement('select');
    select.id = id;
    select.name = id;
    select.setAttribute('aria-label', label);

    for (const opt of options) {
        const option = document.createElement('option');
        option.value = opt.value;
        option.textContent = opt.label;
        if (opt.value === defaultValue) {
            option.selected = true;
        }
        select.appendChild(option);
    }

    wrapper.appendChild(labelEl);
    wrapper.appendChild(select);

    return wrapper;
}

/**
 * Create the filter UI
 */
function createFilterUI() {
    const container = document.createElement('div');
    container.id = 'location-filter-container';
    container.className = 'location-filter-container';

    // Search row
    const searchRow = document.createElement('div');
    searchRow.className = 'filter-search-row';

    const searchWrapper = document.createElement('div');
    searchWrapper.className = 'search-input-wrapper';

    const searchIcon = document.createElement('span');
    searchIcon.className = 'search-icon';
    searchIcon.setAttribute('aria-hidden', 'true');
    searchIcon.textContent = '\uD83D\uDD0D'; // Magnifying glass

    const searchInput = document.createElement('input');
    searchInput.type = 'text';
    searchInput.id = 'location-search-input';
    searchInput.placeholder = 'Search locations...';
    searchInput.setAttribute('aria-label', 'Search locations');
    searchInput.addEventListener('input', handleSearchInput);
    searchInput.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            searchInput.value = '';
            filterState.query = '';
            updateFilteredResults();
        }
    });

    const clearSearchBtn = document.createElement('button');
    clearSearchBtn.type = 'button';
    clearSearchBtn.className = 'clear-search-btn';
    clearSearchBtn.setAttribute('aria-label', 'Clear search');
    clearSearchBtn.textContent = '\u2715'; // X mark
    clearSearchBtn.addEventListener('click', () => {
        searchInput.value = '';
        filterState.query = '';
        updateFilteredResults();
        searchInput.focus();
    });

    searchWrapper.appendChild(searchIcon);
    searchWrapper.appendChild(searchInput);
    searchWrapper.appendChild(clearSearchBtn);
    searchRow.appendChild(searchWrapper);

    // Filter row 1
    const filterRow1 = document.createElement('div');
    filterRow1.className = 'filter-row';

    const statusSelect = createSelect('filter-status', 'Status', [
        { value: 'all', label: 'All' },
        { value: 'checked', label: 'Checked' },
        { value: 'unchecked', label: 'Unchecked' }
    ], filterState.status);
    statusSelect.querySelector('select').addEventListener('change', (e) => {
        handleFilterChange('status', e.target.value);
    });

    const gameSelect = createSelect('filter-game', 'Game', [
        { value: 'both', label: 'Both' },
        { value: 'oot', label: 'OoT' },
        { value: 'mm', label: 'MM' }
    ], filterState.game);
    gameSelect.querySelector('select').addEventListener('change', (e) => {
        handleFilterChange('game', e.target.value);
    });

    filterRow1.appendChild(statusSelect);
    filterRow1.appendChild(gameSelect);

    // Filter row 2
    const filterRow2 = document.createElement('div');
    filterRow2.className = 'filter-row';

    // Build type options from data
    const types = getUniqueValues(dataState.locations, 'type');
    const typeOptions = [{ value: 'all', label: 'All' }];
    for (const type of types) {
        typeOptions.push({ value: type.toLowerCase(), label: type });
    }

    const typeSelect = createSelect('filter-type', 'Type', typeOptions, filterState.type);
    typeSelect.querySelector('select').addEventListener('change', (e) => {
        handleFilterChange('type', e.target.value);
    });

    // Build region options from data
    const regions = getUniqueValues(dataState.locations, 'region');
    const regionOptions = [{ value: 'all', label: 'All' }];
    for (const region of regions) {
        regionOptions.push({ value: region, label: region });
    }

    const regionSelect = createSelect('filter-region', 'Region', regionOptions, filterState.region);
    regionSelect.querySelector('select').addEventListener('change', (e) => {
        handleFilterChange('region', e.target.value);
    });

    filterRow2.appendChild(typeSelect);
    filterRow2.appendChild(regionSelect);

    // Results row
    const resultsRow = document.createElement('div');
    resultsRow.className = 'filter-results-row';

    const countDisplay = document.createElement('span');
    countDisplay.id = 'location-filter-count';
    countDisplay.className = 'filter-count';
    countDisplay.textContent = `Showing ${dataState.locations.length} of ${dataState.locations.length} locations`;

    const clearBtn = document.createElement('button');
    clearBtn.type = 'button';
    clearBtn.className = 'clear-filters-btn';
    clearBtn.textContent = 'Clear Filters';
    clearBtn.addEventListener('click', clearFilters);

    resultsRow.appendChild(countDisplay);
    resultsRow.appendChild(clearBtn);

    // Assemble container
    container.appendChild(searchRow);
    container.appendChild(filterRow1);
    container.appendChild(filterRow2);
    container.appendChild(resultsRow);

    return container;
}

// ============================================================================
// Data Loading
// ============================================================================

/**
 * Fetch locations from the API
 */
async function fetchFromApi() {
    const roomName = getRoomName();
    if (!roomName) {
        throw new Error('Not on a room page');
    }

    const response = await fetch(`/api/room/${encodeURIComponent(roomName)}/checked-locations`);

    if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data = await response.json();

    // Transform API response to add derived fields
    return data.locations.map(transformLocation);
}

/**
 * Load location data
 */
async function loadLocations() {
    dataState.isLoading = true;
    dataState.error = null;

    try {
        if (customFetchFn) {
            // Use custom data source
            dataState.locations = await customFetchFn();
        } else {
            // Use real API
            dataState.locations = await fetchFromApi();
        }

        dataState.filteredLocations = applyFilters(dataState.locations);
        dataState.isLoading = false;
    } catch (error) {
        dataState.error = error.message;
        dataState.isLoading = false;
        console.error('Failed to load locations:', error);
    }
}

// ============================================================================
// Initialization
// ============================================================================

/**
 * Initialize the location filter
 */
async function initLocationFilter(targetSelector) {
    // Load data first
    await loadLocations();

    // Find or create container
    const target = targetSelector
        ? document.querySelector(targetSelector)
        : document.body;

    if (!target) {
        console.error('Location filter target not found:', targetSelector);
        return null;
    }

    // Create and insert UI
    containerElement = createFilterUI();

    // Insert at the beginning of target
    if (target.firstChild) {
        target.insertBefore(containerElement, target.firstChild);
    } else {
        target.appendChild(containerElement);
    }

    return containerElement;
}

/**
 * Set a custom data source function
 * @param {Function} fetchFn - Async function that returns array of location objects
 */
function setDataSource(fetchFn) {
    customFetchFn = fetchFn;
}

/**
 * Programmatic search
 * @param {string} query - Search query
 */
function search(query) {
    filterState.query = query;
    const searchInput = document.getElementById('location-search-input');
    if (searchInput) searchInput.value = query;
    updateFilteredResults();
}

/**
 * Get current filter state
 */
function getFilters() {
    return { ...filterState };
}

/**
 * Get filtered locations
 */
function getFilteredLocations() {
    return [...dataState.filteredLocations];
}

/**
 * Refresh data from source
 */
async function refreshData() {
    await loadLocations();
    updateFilteredResults();
    return dataState.locations;
}

// Auto-initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        // Only auto-init if a placeholder exists
        const placeholder = document.getElementById('location-filter-placeholder');
        if (placeholder) {
            initLocationFilter('#location-filter-placeholder');
        }
    });
} else {
    const placeholder = document.getElementById('location-filter-placeholder');
    if (placeholder) {
        initLocationFilter('#location-filter-placeholder');
    }
}

// Export API for external use
window.locationFilter = {
    init: initLocationFilter,
    setDataSource: setDataSource,
    search: search,
    getFilters: getFilters,
    clearFilters: clearFilters,
    getFilteredLocations: getFilteredLocations,
    refreshData: refreshData,
    // For testing/debugging
    _getState: () => ({ filter: { ...filterState }, data: { ...dataState } }),
    // Expose helper functions for potential reuse
    _extractRegion: extractRegion,
    _extractGame: extractGame,
    _inferType: inferType,
    _getRegionDisplayName: getRegionDisplayName
};
