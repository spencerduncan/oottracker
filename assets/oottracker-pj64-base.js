// The constants above are generated from Rust code in crate/oottracker-utils/src/release.rs. If they're missing, you have the wrong file.
// Generated constants include: TCP_PORT, SAVE_ADDR, SAVE_SIZE, RAM_RANGES, MM_SAVE_ADDR, MM_SAVE_SIZE, MM_RAM_RANGES, OOT_COMBO_CONTEXT_ADDR, MM_COMBO_CONTEXT_ADDR

const VERSION = 6; // do not rename this variable, the build script checks against it

// Game type detection
var GAME_TYPE_UNKNOWN = 0;
var GAME_TYPE_OOT = 1;
var GAME_TYPE_MM = 2;
var GAME_TYPE_COMBO = 3;

var detectedGame = GAME_TYPE_UNKNOWN;

// ============================================================================
// OoTMM Combo ROM Detection
// ============================================================================

// Combo ROM detection flags
var isComboRomDetected = false;      // True if OoTMM ROM detected via header/signature
var comboDetectionAttempted = false; // True after initial ROM detection attempt

// Active world in combo mode (0 = unknown, 1 = OoT, 2 = MM)
var COMBO_WORLD_UNKNOWN = 0;
var COMBO_WORLD_OOT = 1;
var COMBO_WORLD_MM = 2;
var comboActiveWorld = COMBO_WORLD_UNKNOWN;

// OoTMM ROM Header constants
// The OoTMM combo ROM has "THE LEGEND OF ZELDA" at ROM offset 0x20 (same as OoT)
// but contains a special OoTMM signature in the combo context area
var ROM_HEADER_OFFSET = 0x20;
var ROM_HEADER_OOT_MAGIC = [0x54, 0x48, 0x45, 0x20, 0x4C, 0x45, 0x47, 0x45, 0x4E, 0x44]; // "THE LEGEND"
var ROM_HEADER_MM_MAGIC = [0x5A, 0x45, 0x4C, 0x44, 0x41, 0x20, 0x4D, 0x41, 0x4A, 0x4F]; // "ZELDA MAJO"

// OoTMM uses a special signature at the combo context address
// Non-zero data here indicates combo mode
var OOTMM_SIGNATURE_SIZE = 8;

var RAM_INIT_PACKET_LENGTH = 1;
for (var i = 0; i < RAM_RANGES.length; i++) {
    RAM_INIT_PACKET_LENGTH += RAM_RANGES[i][1];
}

// MM packet length (if MM_RAM_RANGES is defined)
var MM_RAM_INIT_PACKET_LENGTH = 1;
if (typeof MM_RAM_RANGES !== 'undefined') {
    for (var i = 0; i < MM_RAM_RANGES.length; i++) {
        MM_RAM_INIT_PACKET_LENGTH += MM_RAM_RANGES[i][1];
    }
}

function arraysEqual(lhs, rhs) {
    if (lhs.length != rhs.length) { return false; }
    for (var i = 0; i < lhs.length; i++) {
        if (lhs[i] != rhs[i]) { return false; }
    }
    return true;
}

// Send OoT RAM data as RamInit packet (variant 4)
function sendOotRamData(socket, rawRam) {
    const ramData = new ArrayBuffer(RAM_INIT_PACKET_LENGTH);
    new DataView(ramData).setUint8(0, 4); // Packet variant: RamInit
    const ramDataByteArray = new Uint8Array(ramData);
    var offset = 1;
    for (var i = 0; i < RAM_RANGES.length; i++) {
        ramDataByteArray.set(new Uint8Array(rawRam[i]), offset);
        offset += RAM_RANGES[i][1];
    }
    socket.write(new Buffer(ramDataByteArray));
}

// Send MM RAM data as MmRamInit packet (variant 8)
function sendMmRamData(socket, rawMmRam) {
    if (typeof MM_RAM_RANGES === 'undefined' || rawMmRam === null) { return; }
    const mmData = new ArrayBuffer(MM_RAM_INIT_PACKET_LENGTH);
    new DataView(mmData).setUint8(0, 8); // Packet variant: MmRamInit
    const mmDataByteArray = new Uint8Array(mmData);
    var offset = 1;
    for (var i = 0; i < MM_RAM_RANGES.length; i++) {
        mmDataByteArray.set(new Uint8Array(rawMmRam[i]), offset);
        offset += MM_RAM_RANGES[i][1];
    }
    socket.write(new Buffer(mmDataByteArray));
}

// Read MM RAM ranges and check for changes
function readMmRamRanges(rawMmRam) {
    if (typeof MM_RAM_RANGES === 'undefined') { return { rawMmRam: null, changed: false }; }

    var mmChanged = true;
    if (rawMmRam === null) {
        rawMmRam = [];
        for (var i = 0; i < MM_RAM_RANGES.length; i++) {
            rawMmRam.push(mem.getblock(ADDR_ANY_RDRAM.start + MM_RAM_RANGES[i][0], MM_RAM_RANGES[i][1]));
        }
    } else {
        mmChanged = false;
        for (var i = 0; i < MM_RAM_RANGES.length; i++) {
            const newRange = mem.getblock(ADDR_ANY_RDRAM.start + MM_RAM_RANGES[i][0], MM_RAM_RANGES[i][1]);
            if (!arraysEqual(newRange, rawMmRam[i])) {
                rawMmRam[i] = newRange;
                mmChanged = true;
            }
        }
    }
    return { rawMmRam: rawMmRam, changed: mmChanged };
}

// Check if array starts with prefix
function arrayStartsWith(arr, prefix) {
    if (arr.length < prefix.length) { return false; }
    for (var i = 0; i < prefix.length; i++) {
        if (arr[i] != prefix[i]) { return false; }
    }
    return true;
}

// Check if array contains all zeros
function isAllZeros(arr) {
    for (var i = 0; i < arr.length; i++) {
        if (arr[i] != 0) { return false; }
    }
    return true;
}

// ============================================================================
// ROM Header Detection
// ============================================================================

// Check ROM header to identify game type
// Returns: { isOot: bool, isMm: bool, isCombo: bool }
function checkRomHeader() {
    var result = { isOot: false, isMm: false, isCombo: false };

    try {
        // Read ROM header at offset 0x20 (game title location)
        // Note: ADDR_ROM may not be available on all Project64 versions
        if (typeof ADDR_ROM !== 'undefined') {
            var headerData = mem.getblock(ADDR_ROM.start + ROM_HEADER_OFFSET, 20);

            // Check for OoT ROM header
            if (arrayStartsWith(headerData, ROM_HEADER_OOT_MAGIC)) {
                result.isOot = true;
            }

            // Check for MM ROM header
            if (arrayStartsWith(headerData, ROM_HEADER_MM_MAGIC)) {
                result.isMm = true;
            }
        }
    } catch (e) {
        // ROM header reading not supported, fall back to RAM-based detection
        console.log('ROM header check not available: ' + e);
    }

    return result;
}

// Check for OoT "ZELDAZ" magic number at save context offset 0x1c
function checkOotMagic(saveData) {
    return saveData[0x1c] == 0x5a && saveData[0x1d] == 0x45 &&
           saveData[0x1e] == 0x4c && saveData[0x1f] == 0x44 &&
           saveData[0x20] == 0x41 && saveData[0x21] == 0x5a;
}

// Check OoT game mode (0 = gameplay)
function checkOotGameMode(saveData) {
    return saveData[0x135c] == 0x00 && saveData[0x135d] == 0x00 &&
           saveData[0x135e] == 0x00 && saveData[0x135f] == 0x00;
}

// Check for combo randomizer context at OoT address
function checkOotComboContext() {
    if (typeof OOT_COMBO_CONTEXT_ADDR === 'undefined') { return false; }
    try {
        var comboData = mem.getblock(ADDR_ANY_RDRAM.start + OOT_COMBO_CONTEXT_ADDR, OOTMM_SIGNATURE_SIZE);
        return !isAllZeros(comboData);
    } catch (e) {
        return false;
    }
}

// Check for combo randomizer context at MM address
function checkMmComboContext() {
    if (typeof MM_COMBO_CONTEXT_ADDR === 'undefined') { return false; }
    try {
        var comboData = mem.getblock(ADDR_ANY_RDRAM.start + MM_COMBO_CONTEXT_ADDR, OOTMM_SIGNATURE_SIZE);
        return !isAllZeros(comboData);
    } catch (e) {
        return false;
    }
}

// Combined combo context check - checks both OoT and MM context addresses
function checkComboContext() {
    return checkOotComboContext() || checkMmComboContext();
}

// ============================================================================
// OoTMM Combo ROM Detection (Enhanced)
// ============================================================================

// Perform full OoTMM combo ROM detection
// This should be called once at startup or when ROM changes
// Returns: GAME_TYPE_UNKNOWN, GAME_TYPE_OOT, GAME_TYPE_MM, or GAME_TYPE_COMBO
function detectComboRom() {
    // First try ROM header detection
    var romInfo = checkRomHeader();

    // If we can read ROM header and it's OoT-based, check combo context
    if (romInfo.isOot) {
        // For OoT ROMs, check if it's actually an OoTMM combo
        if (checkComboContext()) {
            isComboRomDetected = true;
            console.log('OoTMM combo ROM detected via ROM header + combo context');
            return GAME_TYPE_COMBO;
        }
        return GAME_TYPE_OOT;
    }

    // If ROM header shows MM, it could still be a combo waiting to initialize
    if (romInfo.isMm) {
        if (checkComboContext()) {
            isComboRomDetected = true;
            console.log('OoTMM combo ROM detected (MM header + combo context)');
            return GAME_TYPE_COMBO;
        }
        return GAME_TYPE_MM;
    }

    // ROM header check didn't work or not available
    // Fall back to RAM-based detection
    return GAME_TYPE_UNKNOWN;
}

// Detect which world is currently active in combo mode
// Returns: COMBO_WORLD_OOT, COMBO_WORLD_MM, or COMBO_WORLD_UNKNOWN
function detectComboActiveWorld(ootSaveData) {
    if (!isComboRomDetected && detectedGame !== GAME_TYPE_COMBO) {
        return COMBO_WORLD_UNKNOWN;
    }

    // Check if OoT save context has valid data (ZELDAZ magic)
    var ootActive = checkOotMagic(ootSaveData);

    // Check if MM save context has valid data
    var mmActive = false;
    if (typeof MM_SAVE_ADDR !== 'undefined') {
        try {
            var mmData = mem.getblock(ADDR_ANY_RDRAM.start + MM_SAVE_ADDR, 32);
            // MM save context is considered active if it has non-zero structured data
            // Check player form and some basic fields
            mmActive = !isAllZeros(mmData.slice(0, 16));
        } catch (e) {
            mmActive = false;
        }
    }

    // Determine active world based on which context is more recently valid
    if (ootActive && !mmActive) {
        return COMBO_WORLD_OOT;
    } else if (mmActive && !ootActive) {
        return COMBO_WORLD_MM;
    } else if (ootActive && mmActive) {
        // Both contexts have data - check game mode to determine active world
        // OoT game mode is at offset 0x135c-0x135f
        if (checkOotGameMode(ootSaveData)) {
            return COMBO_WORLD_OOT;
        } else {
            return COMBO_WORLD_MM;
        }
    }

    return COMBO_WORLD_UNKNOWN;
}

// Detect game type from memory (enhanced with combo detection)
function detectGameType(ootSaveData) {
    // If we haven't attempted combo ROM detection yet, do it now
    if (!comboDetectionAttempted) {
        comboDetectionAttempted = true;
        var comboResult = detectComboRom();
        if (comboResult !== GAME_TYPE_UNKNOWN) {
            if (comboResult === GAME_TYPE_COMBO) {
                // Update active world detection for combo mode
                comboActiveWorld = detectComboActiveWorld(ootSaveData);
                console.log('Combo mode active world: ' + (comboActiveWorld === COMBO_WORLD_OOT ? 'OoT' : comboActiveWorld === COMBO_WORLD_MM ? 'MM' : 'Unknown'));
            }
            return comboResult;
        }
    }

    // If combo ROM was previously detected, maintain that state
    if (isComboRomDetected) {
        // Update active world detection
        comboActiveWorld = detectComboActiveWorld(ootSaveData);
        return GAME_TYPE_COMBO;
    }

    // Check for OoT via ZELDAZ magic
    if (checkOotMagic(ootSaveData)) {
        // Double-check for combo randomizer context (in case ROM header check missed it)
        if (checkComboContext()) {
            isComboRomDetected = true;
            comboActiveWorld = detectComboActiveWorld(ootSaveData);
            console.log('OoTMM combo detected via RAM check');
            return GAME_TYPE_COMBO;
        }
        return GAME_TYPE_OOT;
    }

    // Check for MM by looking for non-zero data at MM save context
    if (typeof MM_SAVE_ADDR !== 'undefined') {
        try {
            var mmData = mem.getblock(ADDR_ANY_RDRAM.start + MM_SAVE_ADDR, 4);
            if (!isAllZeros(mmData)) {
                // Also check if this is actually a combo ROM with MM active
                if (checkComboContext()) {
                    isComboRomDetected = true;
                    comboActiveWorld = COMBO_WORLD_MM;
                    console.log('OoTMM combo detected via MM context + combo check');
                    return GAME_TYPE_COMBO;
                }
                return GAME_TYPE_MM;
            }
        } catch (e) {
            // MM memory not accessible
        }
    }

    return GAME_TYPE_UNKNOWN;
}

var sock = new Socket();
sock.on('close', function() {
    alert('connection to oottracker lost');
    throw 'connection to oottracker lost';
});
sock.connect({host: "127.0.0.1", port: TCP_PORT}, function() {
    const handshake = new ArrayBuffer(1);
    new DataView(handshake).setUint8(0, VERSION);
    sock.write(new Buffer(new Uint8Array(handshake)), function() {
        console.log('Connected to OoT Tracker');
        //TODO send auto-tracker context
        var rawRam = null;
        var rawMmRam = null;
        var lastActiveWorld = COMBO_WORLD_UNKNOWN; // Track world changes
        events.ondraw(function() {
            var changed = true;

            // Read OoT RAM ranges
            if (rawRam === null) {
                rawRam = [];
                for (var i = 0; i < RAM_RANGES.length; i++) {
                    rawRam.push(mem.getblock(ADDR_ANY_RDRAM.start + RAM_RANGES[i][0], RAM_RANGES[i][1]));
                }
            } else {
                changed = false;
                for (var i = 0; i < RAM_RANGES.length; i++) {
                    const newRange = mem.getblock(ADDR_ANY_RDRAM.start + RAM_RANGES[i][0], RAM_RANGES[i][1]);
                    if (!arraysEqual(newRange, rawRam[i])) {
                        rawRam[i] = newRange;
                        changed = true;
                    }
                }
            }

            // Detect game type on first run or if changed
            if (detectedGame === GAME_TYPE_UNKNOWN || changed) {
                var previousGame = detectedGame;
                detectedGame = detectGameType(rawRam[0]);

                // Log game type detection changes
                if (detectedGame !== previousGame) {
                    var gameTypeNames = ['Unknown', 'OoT', 'MM', 'OoTMM Combo'];
                    console.log('Game type detected: ' + gameTypeNames[detectedGame]);

                    // Log combo-specific info
                    if (detectedGame === GAME_TYPE_COMBO) {
                        console.log('OoTMM combo ROM detected - isComboRomDetected=' + isComboRomDetected);
                    }
                }
            }

            // For combo mode, track world changes
            if (detectedGame === GAME_TYPE_COMBO) {
                var newWorld = detectComboActiveWorld(rawRam[0]);
                if (newWorld !== lastActiveWorld && newWorld !== COMBO_WORLD_UNKNOWN) {
                    var worldNames = ['Unknown', 'OoT', 'MM'];
                    console.log('Combo world switched to: ' + worldNames[newWorld]);
                    lastActiveWorld = newWorld;
                    comboActiveWorld = newWorld;
                }
            }

            // Handle MM-only game (skip OoT processing)
            if (detectedGame === GAME_TYPE_MM) {
                var mmResult = readMmRamRanges(rawMmRam);
                rawMmRam = mmResult.rawMmRam;
                if (mmResult.changed && rawMmRam !== null) {
                    sendMmRamData(sock, rawMmRam);
                }
                return;
            }

            // Handle OoT/Combo game
            if (detectedGame === GAME_TYPE_OOT || detectedGame === GAME_TYPE_COMBO) {
                // For combo mode, check if we're in OoT world before sending OoT data
                if (detectedGame === GAME_TYPE_COMBO && comboActiveWorld === COMBO_WORLD_MM) {
                    // In MM world of combo - handle MM data instead
                    if (typeof MM_RAM_RANGES !== 'undefined') {
                        var mmChanged = true;
                        if (rawMmRam === null) {
                            rawMmRam = [];
                            for (var i = 0; i < MM_RAM_RANGES.length; i++) {
                                rawMmRam.push(mem.getblock(ADDR_ANY_RDRAM.start + MM_RAM_RANGES[i][0], MM_RAM_RANGES[i][1]));
                            }
                        } else {
                            mmChanged = false;
                            for (var i = 0; i < MM_RAM_RANGES.length; i++) {
                                const newRange = mem.getblock(ADDR_ANY_RDRAM.start + MM_RAM_RANGES[i][0], MM_RAM_RANGES[i][1]);
                                if (!arraysEqual(newRange, rawMmRam[i])) {
                                    rawMmRam[i] = newRange;
                                    mmChanged = true;
                                }
                            }
                        }
                        // TODO: Send MM data when protocol supports combo mode
                    }
                    // Also still read OoT ranges to keep save data updated for combo tracking
                    // (save data persists across world switches in OoTMM)
                }

                if (!changed) { return; }
                if (!checkOotMagic(rawRam[0])) { return; } // ZELDAZ magic number not present
                if (!checkOotGameMode(rawRam[0])) { return; } // game mode != gameplay

                // Send OoT RAM data
                sendOotRamData(sock, rawRam);

                // For combo mode, also read and send MM data
                if (detectedGame === GAME_TYPE_COMBO) {
                    var mmResult = readMmRamRanges(rawMmRam);
                    rawMmRam = mmResult.rawMmRam;
                    // In combo mode, always send MM data when OoT data is sent
                    // (since they share the same save file)
                    if (rawMmRam !== null) {
                        sendMmRamData(sock, rawMmRam);
                    }
                }
            }
        });
    });
});
