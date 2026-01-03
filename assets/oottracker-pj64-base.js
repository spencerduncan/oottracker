// The constants above are generated from Rust code in crate/oottracker-utils/src/release.rs. If they're missing, you have the wrong file.
// Generated constants include: TCP_PORT, SAVE_ADDR, SAVE_SIZE, RAM_RANGES, MM_SAVE_ADDR, MM_SAVE_SIZE, MM_RAM_RANGES, OOT_COMBO_CONTEXT_ADDR, MM_COMBO_CONTEXT_ADDR

const VERSION = 5; // do not rename this variable, the build script checks against it

// Game type detection
var GAME_TYPE_UNKNOWN = 0;
var GAME_TYPE_OOT = 1;
var GAME_TYPE_MM = 2;
var GAME_TYPE_COMBO = 3;

var detectedGame = GAME_TYPE_UNKNOWN;

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

// Check for combo randomizer context
function checkComboContext() {
    if (typeof OOT_COMBO_CONTEXT_ADDR === 'undefined') { return false; }
    var comboData = mem.getblock(ADDR_ANY_RDRAM.start + OOT_COMBO_CONTEXT_ADDR, 4);
    return comboData[0] != 0 || comboData[1] != 0 || comboData[2] != 0 || comboData[3] != 0;
}

// Detect game type from memory
function detectGameType(ootSaveData) {
    // Check for OoT first
    if (checkOotMagic(ootSaveData)) {
        // Check if this is a combo randomizer
        if (checkComboContext()) {
            return GAME_TYPE_COMBO;
        }
        return GAME_TYPE_OOT;
    }

    // Check for MM by looking for non-zero data at MM save context
    if (typeof MM_SAVE_ADDR !== 'undefined') {
        var mmData = mem.getblock(ADDR_ANY_RDRAM.start + MM_SAVE_ADDR, 4);
        if (mmData[0] != 0 || mmData[1] != 0 || mmData[2] != 0 || mmData[3] != 0) {
            return GAME_TYPE_MM;
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
                detectedGame = detectGameType(rawRam[0]);
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
