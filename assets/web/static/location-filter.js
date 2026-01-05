/**
 * Location Filter Module
 *
 * Search and filter component for tracker locations.
 * This is a STUB implementation using mock data for testing.
 * The setDataSource API allows integration with real API endpoints later.
 */

// Filter state
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

// Custom data fetcher (null = use mock data)
let customFetchFn = null;

// Debounce timer
let debounceTimer = null;
const DEBOUNCE_MS = 150;

// DOM element references
let containerElement = null;

/**
 * Mock location data for testing
 * Will be replaced by real API data via setDataSource
 * Format matches real API: {game}_{region}_{location_name}
 */
const mockLocations = [
    // OoT Kokiri Forest
    { location_id: 'oot_kf_mido_top_left_chest', status: 'Checked', game: 'OoT', type: 'Chest', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_mido_top_right_chest', status: 'Checked', game: 'OoT', type: 'Chest', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_mido_bottom_left_chest', status: 'Unchecked', game: 'OoT', type: 'Chest', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_mido_bottom_right_chest', status: 'Unchecked', game: 'OoT', type: 'Chest', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_kokiri_sword_chest', status: 'Checked', game: 'OoT', type: 'Chest', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_storms_grotto_chest', status: 'Unchecked', game: 'OoT', type: 'Chest', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_links_house_cow', status: 'Unchecked', game: 'OoT', type: 'Cow', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_gs_behind_mido_house', status: 'Checked', game: 'OoT', type: 'Skulltula', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_gs_know_it_all_house', status: 'Unchecked', game: 'OoT', type: 'Skulltula', region: 'Kokiri Forest' },
    { location_id: 'oot_kf_gs_bean_patch', status: 'Unchecked', game: 'OoT', type: 'Skulltula', region: 'Kokiri Forest' },
    // OoT Lost Woods
    { location_id: 'oot_lw_skull_kid', status: 'Checked', game: 'OoT', type: 'NPC', region: 'Lost Woods' },
    { location_id: 'oot_lw_ocarina_memory_game', status: 'Unchecked', game: 'OoT', type: 'Minigame', region: 'Lost Woods' },
    { location_id: 'oot_lw_target_in_woods', status: 'Unchecked', game: 'OoT', type: 'Minigame', region: 'Lost Woods' },
    { location_id: 'oot_lw_deku_scrub_near_bridge', status: 'Checked', game: 'OoT', type: 'Scrub', region: 'Lost Woods' },
    { location_id: 'oot_lw_gs_bean_patch_near_bridge', status: 'Unchecked', game: 'OoT', type: 'Skulltula', region: 'Lost Woods' },
    // OoT Hyrule Field
    { location_id: 'oot_hf_near_kak_grotto_chest', status: 'Unchecked', game: 'OoT', type: 'Chest', region: 'Hyrule Field' },
    { location_id: 'oot_hf_open_grotto_chest', status: 'Checked', game: 'OoT', type: 'Chest', region: 'Hyrule Field' },
    { location_id: 'oot_hf_deku_scrub_grotto', status: 'Unchecked', game: 'OoT', type: 'Scrub', region: 'Hyrule Field' },
    { location_id: 'oot_hf_ocarina_of_time', status: 'Checked', game: 'OoT', type: 'Event', region: 'Hyrule Field' },
    // OoT Kakariko Village
    { location_id: 'oot_kak_man_on_roof', status: 'Unchecked', game: 'OoT', type: 'NPC', region: 'Kakariko Village' },
    { location_id: 'oot_kak_anju_as_child', status: 'Checked', game: 'OoT', type: 'NPC', region: 'Kakariko Village' },
    { location_id: 'oot_kak_anju_as_adult', status: 'Unchecked', game: 'OoT', type: 'NPC', region: 'Kakariko Village' },
    { location_id: 'oot_kak_10_gold_skulltula_reward', status: 'Checked', game: 'OoT', type: 'NPC', region: 'Kakariko Village' },
    { location_id: 'oot_kak_20_gold_skulltula_reward', status: 'Unchecked', game: 'OoT', type: 'NPC', region: 'Kakariko Village' },
    { location_id: 'oot_kak_shooting_sun_chest', status: 'Unchecked', game: 'OoT', type: 'Chest', region: 'Kakariko Village' },
    // OoT Death Mountain
    { location_id: 'oot_dmt_chest_above_dodongo', status: 'Checked', game: 'OoT', type: 'Chest', region: 'Death Mountain' },
    { location_id: 'oot_dmt_biggoron', status: 'Unchecked', game: 'OoT', type: 'NPC', region: 'Death Mountain' },
    { location_id: 'oot_dmt_gs_bean_patch', status: 'Unchecked', game: 'OoT', type: 'Skulltula', region: 'Death Mountain' },
    { location_id: 'oot_dmc_great_fairy', status: 'Checked', game: 'OoT', type: 'Fairy', region: 'Death Mountain' },
    // MM Clock Town
    { location_id: 'mm_clock_town_platform_chest', status: 'Unchecked', game: 'MM', type: 'Chest', region: 'Clock Town' },
    { location_id: 'mm_clock_town_stock_pot_inn', status: 'Checked', game: 'MM', type: 'NPC', region: 'Clock Town' },
    { location_id: 'mm_clock_town_postman_game', status: 'Unchecked', game: 'MM', type: 'Minigame', region: 'Clock Town' },
    { location_id: 'mm_clock_town_honey_and_darling', status: 'Unchecked', game: 'MM', type: 'Minigame', region: 'Clock Town' },
    { location_id: 'mm_clock_town_treasure_chest_game', status: 'Checked', game: 'MM', type: 'Minigame', region: 'Clock Town' },
    { location_id: 'mm_clock_town_archery_prize_1', status: 'Unchecked', game: 'MM', type: 'Minigame', region: 'Clock Town' },
    { location_id: 'mm_clock_town_bomb_shop_lady', status: 'Checked', game: 'MM', type: 'NPC', region: 'Clock Town' },
    { location_id: 'mm_clock_town_mayors_wife_mask', status: 'Unchecked', game: 'MM', type: 'NPC', region: 'Clock Town' },
    { location_id: 'mm_clock_town_bank_reward_1', status: 'Checked', game: 'MM', type: 'NPC', region: 'Clock Town' },
    { location_id: 'mm_clock_town_bank_reward_2', status: 'Unchecked', game: 'MM', type: 'NPC', region: 'Clock Town' },
    // MM Woodfall
    { location_id: 'mm_woodfall_chest_behind_owl', status: 'Unchecked', game: 'MM', type: 'Chest', region: 'Woodfall' },
    { location_id: 'mm_woodfall_deku_princess', status: 'Checked', game: 'MM', type: 'Event', region: 'Woodfall' },
    { location_id: 'mm_woodfall_great_fairy', status: 'Checked', game: 'MM', type: 'Fairy', region: 'Woodfall' },
    { location_id: 'mm_woodfall_temple_stray_fairy_1', status: 'Checked', game: 'MM', type: 'StrayFairy', region: 'Woodfall' },
    { location_id: 'mm_woodfall_temple_stray_fairy_2', status: 'Unchecked', game: 'MM', type: 'StrayFairy', region: 'Woodfall' },
    // MM Snowhead
    { location_id: 'mm_snowhead_chest_in_ice_cave', status: 'Unchecked', game: 'MM', type: 'Chest', region: 'Snowhead' },
    { location_id: 'mm_snowhead_goron_race', status: 'Unchecked', game: 'MM', type: 'Minigame', region: 'Snowhead' },
    { location_id: 'mm_snowhead_great_fairy', status: 'Checked', game: 'MM', type: 'Fairy', region: 'Snowhead' },
    // MM Great Bay
    { location_id: 'mm_great_bay_coast_chest', status: 'Checked', game: 'MM', type: 'Chest', region: 'Great Bay' },
    { location_id: 'mm_great_bay_beaver_race', status: 'Unchecked', game: 'MM', type: 'Minigame', region: 'Great Bay' },
    { location_id: 'mm_great_bay_pirates_fortress_chest_1', status: 'Unchecked', game: 'MM', type: 'Chest', region: 'Great Bay' },
    { location_id: 'mm_great_bay_pirates_fortress_chest_2', status: 'Checked', game: 'MM', type: 'Chest', region: 'Great Bay' },
    { location_id: 'mm_great_bay_great_fairy', status: 'Unchecked', game: 'MM', type: 'Fairy', region: 'Great Bay' },
    // MM Ikana
    { location_id: 'mm_ikana_graveyard_chest', status: 'Unchecked', game: 'MM', type: 'Chest', region: 'Ikana' },
    { location_id: 'mm_ikana_dampe_dig', status: 'Checked', game: 'MM', type: 'Minigame', region: 'Ikana' },
    { location_id: 'mm_ikana_stone_tower_chest', status: 'Unchecked', game: 'MM', type: 'Chest', region: 'Ikana' },
    { location_id: 'mm_ikana_great_fairy', status: 'Checked', game: 'MM', type: 'Fairy', region: 'Ikana' }
];

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
            // Use mock data
            dataState.locations = [...mockLocations];
        }

        dataState.filteredLocations = applyFilters(dataState.locations);
        dataState.isLoading = false;
    } catch (error) {
        dataState.error = error.message;
        dataState.isLoading = false;
        console.error('Failed to load locations:', error);
    }
}

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
    _getMockData: () => [...mockLocations],
    _getState: () => ({ filter: { ...filterState }, data: { ...dataState } })
};
