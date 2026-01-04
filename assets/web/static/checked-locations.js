/**
 * Checked Locations Display Module
 *
 * This module fetches and displays the checked locations status
 * from the API endpoint and updates the UI accordingly.
 */

// Checked locations state
let checkedLocationsState = {
    locations: {},
    lastUpdate: null,
    isLoading: false,
    error: null
};

// Status display element
let statusElement = null;

/**
 * Get the current room name from the URL
 */
function getRoomName() {
    const match = window.location.pathname.match(/^\/room\/([0-9A-Za-z-]+)\/?$/);
    return match ? match[1] : null;
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
        const percent = data.total_mapped > 0
            ? Math.round((data.checked_count / data.total_mapped) * 100)
            : 0;
        statusElement.innerHTML = `
            <span class="checked-count">${data.checked_count}</span>
            <span class="checked-separator">/</span>
            <span class="total-count">${data.total_mapped}</span>
            <span class="checked-percent">(${percent}%)</span>
        `;
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
 * Update the locations list display
 */
function updateLocationsList(data) {
    const list = document.getElementById('checked-locations-list');
    if (!list || !data) return;

    // Group locations by area (based on common prefix patterns)
    const grouped = {
        checked: [],
        unchecked: [],
        unknown: []
    };

    for (const loc of data.locations) {
        if (loc.status === 'Checked') {
            grouped.checked.push(loc.location_id);
        } else if (loc.status === 'Unchecked') {
            grouped.unchecked.push(loc.location_id);
        } else {
            grouped.unknown.push(loc.location_id);
        }
    }

    let html = '';

    if (grouped.checked.length > 0) {
        html += '<div class="location-group"><h4>Checked</h4><ul>';
        for (const loc of grouped.checked.sort()) {
            html += `<li class="location-checked">${escapeHtml(loc)}</li>`;
        }
        html += '</ul></div>';
    }

    if (grouped.unchecked.length > 0) {
        html += '<div class="location-group"><h4>Unchecked</h4><ul>';
        for (const loc of grouped.unchecked.sort()) {
            html += `<li class="location-unchecked">${escapeHtml(loc)}</li>`;
        }
        html += '</ul></div>';
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
    isLocationChecked: (locationId) => checkedLocationsState.locations[locationId] === 'Checked'
};
