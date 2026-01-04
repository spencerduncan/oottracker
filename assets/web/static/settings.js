/**
 * OoTMM Tracker Settings UI
 *
 * This module handles the settings configuration UI for the OoTMM randomizer tracker.
 * It provides functionality to load, save, and manage RandomizerSettings.
 */

// Default settings matching RandomizerSettings::default() in Rust
const DEFAULT_SETTINGS = {
    // Boolean settings
    agelessBoots: false,
    agelessHookshot: false,
    agelessStrength: false,
    alterLostWoodsExits: false,
    erIndoorsExtra: false,
    erIndoorsGameLinks: false,
    erIndoorsMajor: false,
    erMoon: false,
    openMaskShop: false,
    openMoon: false,
    openZdShortcut: false,
    pondFishShuffle: false,
    restoreBrokenActors: false,
    skipZelda: false,
    timeTravelSword: false,

    // Set settings (arrays)
    openDungeonsOot: [],
    openDungeonsMm: [],
    clearStateDungeonsMm: [],
    jpLayouts: [],
    logicTricks: [],

    // Enum settings
    dekuTree: 'closed',
    doorOfTime: 'closed',
    kakarikoGate: 'closed',
    ganonBossKey: 'vanilla',
    lacs: 'vanilla',
    majoraChild: 'vanilla',
    moonCrash: 'vanilla',
    ageChange: 'templeOfTime',
    climbMostSurfacesOot: 'on',
    hookshotAnywhereOot: 'on',
    beneathWell: 'vanilla',
    erOverworld: 'none',
    erGrottos: 'none',
    bossWarpPads: 'vanilla',
    smallKeyShuffleOot: 'vanilla',
    shufflePotsMm: 'none',
    logicMode: 'glitchless'
};

/**
 * Gets the current settings from the form as a JSON object.
 * @returns {Object} The current settings
 */
function getSettingsFromForm() {
    const form = document.getElementById('settings-form');
    const settings = { ...DEFAULT_SETTINGS };

    // Boolean checkboxes
    const booleanFields = [
        'agelessBoots', 'agelessHookshot', 'agelessStrength',
        'alterLostWoodsExits', 'erIndoorsExtra', 'erIndoorsGameLinks',
        'erIndoorsMajor', 'erMoon', 'openMaskShop', 'openMoon',
        'openZdShortcut', 'pondFishShuffle', 'restoreBrokenActors',
        'skipZelda', 'timeTravelSword'
    ];

    booleanFields.forEach(field => {
        const checkbox = form.querySelector(`input[name="${field}"]`);
        if (checkbox) {
            settings[field] = checkbox.checked;
        }
    });

    // Multi-value checkboxes (sets)
    settings.openDungeonsOot = Array.from(form.querySelectorAll('input[name="openDungeonsOot"]:checked'))
        .map(cb => cb.value);
    settings.openDungeonsMm = Array.from(form.querySelectorAll('input[name="openDungeonsMm"]:checked'))
        .map(cb => cb.value);

    // Select dropdowns
    const selectFields = [
        'logicMode', 'dekuTree', 'doorOfTime', 'kakarikoGate',
        'ganonBossKey', 'lacs', 'majoraChild', 'moonCrash',
        'ageChange', 'climbMostSurfacesOot', 'hookshotAnywhereOot',
        'beneathWell', 'erOverworld', 'erGrottos', 'bossWarpPads',
        'smallKeyShuffleOot', 'shufflePotsMm'
    ];

    selectFields.forEach(field => {
        const select = form.querySelector(`select[name="${field}"]`);
        if (select) {
            settings[field] = select.value;
        }
    });

    // Logic tricks (comma-separated textarea)
    const tricksTextarea = form.querySelector('textarea[name="logicTricks"]');
    if (tricksTextarea && tricksTextarea.value.trim()) {
        settings.logicTricks = tricksTextarea.value
            .split(',')
            .map(trick => trick.trim())
            .filter(trick => trick.length > 0);
    } else {
        settings.logicTricks = [];
    }

    return settings;
}

/**
 * Populates the form with settings values.
 * @param {Object} settings The settings object to populate from
 */
function populateForm(settings) {
    const form = document.getElementById('settings-form');
    const merged = { ...DEFAULT_SETTINGS, ...settings };

    // Boolean checkboxes
    const booleanFields = [
        'agelessBoots', 'agelessHookshot', 'agelessStrength',
        'alterLostWoodsExits', 'erIndoorsExtra', 'erIndoorsGameLinks',
        'erIndoorsMajor', 'erMoon', 'openMaskShop', 'openMoon',
        'openZdShortcut', 'pondFishShuffle', 'restoreBrokenActors',
        'skipZelda', 'timeTravelSword'
    ];

    booleanFields.forEach(field => {
        const checkbox = form.querySelector(`input[name="${field}"]`);
        if (checkbox) {
            checkbox.checked = merged[field] || false;
        }
    });

    // Multi-value checkboxes (sets)
    const dungeonOot = merged.openDungeonsOot || [];
    form.querySelectorAll('input[name="openDungeonsOot"]').forEach(cb => {
        cb.checked = dungeonOot.includes(cb.value);
    });

    const dungeonMm = merged.openDungeonsMm || [];
    form.querySelectorAll('input[name="openDungeonsMm"]').forEach(cb => {
        cb.checked = dungeonMm.includes(cb.value);
    });

    // Select dropdowns
    const selectFields = [
        'logicMode', 'dekuTree', 'doorOfTime', 'kakarikoGate',
        'ganonBossKey', 'lacs', 'majoraChild', 'moonCrash',
        'ageChange', 'climbMostSurfacesOot', 'hookshotAnywhereOot',
        'beneathWell', 'erOverworld', 'erGrottos', 'bossWarpPads',
        'smallKeyShuffleOot', 'shufflePotsMm'
    ];

    selectFields.forEach(field => {
        const select = form.querySelector(`select[name="${field}"]`);
        if (select && merged[field]) {
            select.value = merged[field];
        }
    });

    // Logic tricks
    const tricksTextarea = form.querySelector('textarea[name="logicTricks"]');
    if (tricksTextarea) {
        const tricks = merged.logicTricks || [];
        tricksTextarea.value = tricks.join(', ');
    }
}

/**
 * Shows a status message to the user.
 * @param {string} message The message to display
 * @param {string} type The type of message ('success', 'error', 'info')
 */
function showStatus(message, type = 'info') {
    const statusEl = document.getElementById('status-message');
    if (statusEl) {
        statusEl.textContent = message;
        statusEl.className = `status-message ${type}`;
        statusEl.style.display = 'block';

        // Auto-hide after 3 seconds
        setTimeout(() => {
            statusEl.style.display = 'none';
        }, 3000);
    }
}

/**
 * Saves settings to a JSON file (download).
 */
function saveSettingsToFile() {
    try {
        const settings = getSettingsFromForm();
        const json = JSON.stringify(settings, null, 2);
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);

        const a = document.createElement('a');
        a.href = url;
        a.download = 'ootmm-settings.json';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);

        showStatus('Settings saved to file', 'success');
    } catch (error) {
        console.error('Error saving settings:', error);
        showStatus('Error saving settings: ' + error.message, 'error');
    }
}

/**
 * Loads settings from a JSON file.
 */
function loadSettingsFromFile() {
    const fileInput = document.getElementById('fileInput');
    fileInput.click();
}

/**
 * Handles file input change event.
 * @param {Event} event The change event
 */
function handleFileLoad(event) {
    const file = event.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = function(e) {
        try {
            const settings = JSON.parse(e.target.result);
            populateForm(settings);
            showStatus('Settings loaded from file', 'success');
        } catch (error) {
            console.error('Error loading settings:', error);
            showStatus('Error loading settings: Invalid JSON file', 'error');
        }
    };
    reader.readAsText(file);

    // Reset file input so the same file can be loaded again
    event.target.value = '';
}

/**
 * Resets all settings to defaults.
 */
function resetSettings() {
    if (confirm('Are you sure you want to reset all settings to defaults?')) {
        populateForm(DEFAULT_SETTINGS);
        showStatus('Settings reset to defaults', 'success');
    }
}

/**
 * Copies current settings to clipboard as JSON.
 */
async function copySettingsAsJson() {
    try {
        const settings = getSettingsFromForm();
        const json = JSON.stringify(settings, null, 2);
        await navigator.clipboard.writeText(json);
        showStatus('Settings copied to clipboard', 'success');
    } catch (error) {
        console.error('Error copying settings:', error);
        showStatus('Error copying settings: ' + error.message, 'error');
    }
}

/**
 * Saves settings to localStorage.
 */
function saveToLocalStorage() {
    try {
        const settings = getSettingsFromForm();
        localStorage.setItem('ootmm-settings', JSON.stringify(settings));
    } catch (error) {
        console.error('Error saving to localStorage:', error);
    }
}

/**
 * Loads settings from localStorage.
 */
function loadFromLocalStorage() {
    try {
        const saved = localStorage.getItem('ootmm-settings');
        if (saved) {
            const settings = JSON.parse(saved);
            populateForm(settings);
            return true;
        }
    } catch (error) {
        console.error('Error loading from localStorage:', error);
    }
    return false;
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    // Load settings from localStorage if available
    if (!loadFromLocalStorage()) {
        // Otherwise use defaults
        populateForm(DEFAULT_SETTINGS);
    }

    // Set up event listeners
    document.getElementById('saveSettings').addEventListener('click', saveSettingsToFile);
    document.getElementById('loadSettings').addEventListener('click', loadSettingsFromFile);
    document.getElementById('resetSettings').addEventListener('click', resetSettings);
    document.getElementById('copyJson').addEventListener('click', copySettingsAsJson);
    document.getElementById('fileInput').addEventListener('change', handleFileLoad);

    // Auto-save to localStorage on any form change
    const form = document.getElementById('settings-form');
    form.addEventListener('change', saveToLocalStorage);
    form.addEventListener('input', function(e) {
        if (e.target.tagName === 'TEXTAREA') {
            saveToLocalStorage();
        }
    });
});
