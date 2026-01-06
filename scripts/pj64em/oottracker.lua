-- Generated constants for OoTTracker PJ64-EM integration
TCP_PORT = 24801
SAVE_ADDR = 0x11a5d0
SAVE_SIZE = 0x1450
MM_SAVE_ADDR = 0x1ef670
MM_SAVE_SIZE = 0x48d0
OOT_COMBO_CONTEXT_ADDR = 0x6584
MM_COMBO_CONTEXT_ADDR = 0x98280

-- RAM_RANGES: {addr1, len1, addr2, len2, ...}
RAM_RANGES = {
    0x11a5d0, 0x1450,  -- save context
    0x1c84b4, 2,       -- buttons pressed
    0x1c8545, 1,       -- current scene ID
    0x1ca1c8, 4,       -- current scene switch flags
    0x1ca1d8, 8,       -- chest and room clear flags
    0x1d8870, 2,       -- text box ID
    0x1d887e, 0xc0,    -- text box contents
    0x1d8dd4, 0x16     -- pause context
}

-- MM_RAM_RANGES: {addr1, len1}
MM_RAM_RANGES = {
    0x1ef670, 0x48d0   -- MM save context
}

-- Include the base script content
local VERSION = 6 -- do not rename this variable, the build script checks against it

-- Game type detection
local GAME_TYPE_UNKNOWN = 0
local GAME_TYPE_OOT = 1
local GAME_TYPE_MM = 2
local GAME_TYPE_COMBO = 3

local detectedGame = GAME_TYPE_UNKNOWN

-- ============================================================================
-- OoTMM Combo ROM Detection
-- ============================================================================

-- Combo ROM detection flags
local isComboRomDetected = false      -- True if OoTMM ROM detected via header/signature
local comboDetectionAttempted = false -- True after initial ROM detection attempt

-- Active world in combo mode (0 = unknown, 1 = OoT, 2 = MM)
local COMBO_WORLD_UNKNOWN = 0
local COMBO_WORLD_OOT = 1
local COMBO_WORLD_MM = 2
local comboActiveWorld = COMBO_WORLD_UNKNOWN

-- OoTMM ROM Header constants
-- The OoTMM combo ROM has "THE LEGEND OF ZELDA" at ROM offset 0x20 (same as OoT)
-- but contains a special OoTMM signature in the combo context area
local ROM_HEADER_OFFSET = 0x20
local ROM_HEADER_OOT_MAGIC = {0x54, 0x48, 0x45, 0x20, 0x4C, 0x45, 0x47, 0x45, 0x4E, 0x44} -- "THE LEGEND"
local ROM_HEADER_MM_MAGIC = {0x5A, 0x45, 0x4C, 0x44, 0x41, 0x20, 0x4D, 0x41, 0x4A, 0x4F} -- "ZELDA MAJO"

-- OoTMM uses a special signature at the combo context address
-- Non-zero data here indicates combo mode
local OOTMM_SIGNATURE_SIZE = 8

-- RDRAM base address for N64
local RDRAM_BASE = 0x80000000

-- Calculate RAM init packet length
local RAM_INIT_PACKET_LENGTH = 1
for i = 1, #RAM_RANGES, 2 do
    RAM_INIT_PACKET_LENGTH = RAM_INIT_PACKET_LENGTH + RAM_RANGES[i + 1]
end

-- MM packet length (if MM_RAM_RANGES is defined)
local MM_RAM_INIT_PACKET_LENGTH = 1
if MM_RAM_RANGES ~= nil then
    for i = 1, #MM_RAM_RANGES, 2 do
        MM_RAM_INIT_PACKET_LENGTH = MM_RAM_INIT_PACKET_LENGTH + MM_RAM_RANGES[i + 1]
    end
end

-- ============================================================================
-- Helper Functions
-- ============================================================================

-- Compare two byte arrays for equality
local function arraysEqual(lhs, rhs)
    if #lhs ~= #rhs then return false end
    for i = 1, #lhs do
        if lhs[i] ~= rhs[i] then return false end
    end
    return true
end

-- Check if array starts with prefix
local function arrayStartsWith(arr, prefix)
    if #arr < #prefix then return false end
    for i = 1, #prefix do
        if arr[i] ~= prefix[i] then return false end
    end
    return true
end

-- Check if array contains all zeros
local function isAllZeros(arr)
    for i = 1, #arr do
        if arr[i] ~= 0 then return false end
    end
    return true
end

-- Read a block of memory into a byte array
local function readMemoryBlock(addr, size)
    local data = {}
    for i = 0, size - 1 do
        data[i + 1] = memory.read_u8(addr + i)
    end
    return data
end

-- ============================================================================
-- Network Functions
-- ============================================================================

-- Send OoT RAM data as RamInit packet (variant 4)
local function sendOotRamData(sock, rawRam)
    local packet = binary.pack_u8(4) -- Packet variant: RamInit

    local rangeIdx = 1
    for i = 1, #RAM_RANGES, 2 do
        local rangeData = rawRam[rangeIdx]
        for j = 1, #rangeData do
            packet = packet .. binary.pack_u8(rangeData[j])
        end
        rangeIdx = rangeIdx + 1
    end

    sock:send(packet)
end

-- Send MM RAM data as MmRamInit packet (variant 8)
local function sendMmRamData(sock, rawMmRam)
    if MM_RAM_RANGES == nil or rawMmRam == nil then return end

    local packet = binary.pack_u8(8) -- Packet variant: MmRamInit

    local rangeIdx = 1
    for i = 1, #MM_RAM_RANGES, 2 do
        local rangeData = rawMmRam[rangeIdx]
        for j = 1, #rangeData do
            packet = packet .. binary.pack_u8(rangeData[j])
        end
        rangeIdx = rangeIdx + 1
    end

    sock:send(packet)
end

-- Read MM RAM ranges and check for changes
local function readMmRamRanges(rawMmRam)
    if MM_RAM_RANGES == nil then
        return nil, false
    end

    local mmChanged = true
    if rawMmRam == nil then
        rawMmRam = {}
        local rangeIdx = 1
        for i = 1, #MM_RAM_RANGES, 2 do
            local addr = MM_RAM_RANGES[i]
            local size = MM_RAM_RANGES[i + 1]
            rawMmRam[rangeIdx] = readMemoryBlock(RDRAM_BASE + addr, size)
            rangeIdx = rangeIdx + 1
        end
    else
        mmChanged = false
        local rangeIdx = 1
        for i = 1, #MM_RAM_RANGES, 2 do
            local addr = MM_RAM_RANGES[i]
            local size = MM_RAM_RANGES[i + 1]
            local newRange = readMemoryBlock(RDRAM_BASE + addr, size)
            if not arraysEqual(newRange, rawMmRam[rangeIdx]) then
                rawMmRam[rangeIdx] = newRange
                mmChanged = true
            end
            rangeIdx = rangeIdx + 1
        end
    end

    return rawMmRam, mmChanged
end

-- ============================================================================
-- ROM Header Detection
-- ============================================================================

-- Check ROM header to identify game type
-- Returns: { isOot = bool, isMm = bool, isCombo = bool }
local function checkRomHeader()
    local result = { isOot = false, isMm = false, isCombo = false }

    -- ROM header reading may not be available on all emulators
    -- PJ64-EM might not support ROM memory access, so we skip this check
    -- and rely on RAM-based detection

    return result
end

-- Check for OoT "ZELDAZ" magic number at save context offset 0x1c
local function checkOotMagic(saveData)
    -- Lua arrays are 1-indexed, so offset 0x1c becomes index 0x1d (29)
    return saveData[0x1d] == 0x5a and saveData[0x1e] == 0x45 and
           saveData[0x1f] == 0x4c and saveData[0x20] == 0x44 and
           saveData[0x21] == 0x41 and saveData[0x22] == 0x5a
end

-- Check OoT game mode (0 = gameplay)
local function checkOotGameMode(saveData)
    -- Lua arrays are 1-indexed, so offset 0x135c becomes index 0x135d (4958)
    return saveData[0x135d] == 0x00 and saveData[0x135e] == 0x00 and
           saveData[0x135f] == 0x00 and saveData[0x1360] == 0x00
end

-- Check for combo randomizer context at OoT address
local function checkOotComboContext()
    if OOT_COMBO_CONTEXT_ADDR == nil then return false end

    local success, comboData = pcall(function()
        return readMemoryBlock(RDRAM_BASE + OOT_COMBO_CONTEXT_ADDR, OOTMM_SIGNATURE_SIZE)
    end)

    if not success then return false end
    return not isAllZeros(comboData)
end

-- Check for combo randomizer context at MM address
local function checkMmComboContext()
    if MM_COMBO_CONTEXT_ADDR == nil then return false end

    local success, comboData = pcall(function()
        return readMemoryBlock(RDRAM_BASE + MM_COMBO_CONTEXT_ADDR, OOTMM_SIGNATURE_SIZE)
    end)

    if not success then return false end
    return not isAllZeros(comboData)
end

-- Combined combo context check - checks both OoT and MM context addresses
local function checkComboContext()
    return checkOotComboContext() or checkMmComboContext()
end

-- ============================================================================
-- OoTMM Combo ROM Detection (Enhanced)
-- ============================================================================

-- Perform full OoTMM combo ROM detection
-- This should be called once at startup or when ROM changes
-- Returns: GAME_TYPE_UNKNOWN, GAME_TYPE_OOT, GAME_TYPE_MM, or GAME_TYPE_COMBO
local function detectComboRom()
    -- First try ROM header detection
    local romInfo = checkRomHeader()

    -- If we can read ROM header and it's OoT-based, check combo context
    if romInfo.isOot then
        -- For OoT ROMs, check if it's actually an OoTMM combo
        if checkComboContext() then
            isComboRomDetected = true
            print("OoTMM combo ROM detected via ROM header + combo context")
            return GAME_TYPE_COMBO
        end
        return GAME_TYPE_OOT
    end

    -- If ROM header shows MM, it could still be a combo waiting to initialize
    if romInfo.isMm then
        if checkComboContext() then
            isComboRomDetected = true
            print("OoTMM combo ROM detected (MM header + combo context)")
            return GAME_TYPE_COMBO
        end
        return GAME_TYPE_MM
    end

    -- ROM header check didn't work or not available
    -- Fall back to RAM-based detection
    return GAME_TYPE_UNKNOWN
end

-- Detect which world is currently active in combo mode
-- Returns: COMBO_WORLD_OOT, COMBO_WORLD_MM, or COMBO_WORLD_UNKNOWN
local function detectComboActiveWorld(ootSaveData)
    if not isComboRomDetected and detectedGame ~= GAME_TYPE_COMBO then
        return COMBO_WORLD_UNKNOWN
    end

    -- Check if OoT save context has valid data (ZELDAZ magic)
    local ootActive = checkOotMagic(ootSaveData)

    -- Check if MM save context has valid data
    local mmActive = false
    if MM_SAVE_ADDR ~= nil then
        local success, mmData = pcall(function()
            return readMemoryBlock(RDRAM_BASE + MM_SAVE_ADDR, 32)
        end)

        if success then
            -- MM save context is considered active if it has non-zero structured data
            -- Check player form and some basic fields
            local slice = {}
            for i = 1, 16 do
                slice[i] = mmData[i]
            end
            mmActive = not isAllZeros(slice)
        end
    end

    -- Determine active world based on which context is more recently valid
    if ootActive and not mmActive then
        return COMBO_WORLD_OOT
    elseif mmActive and not ootActive then
        return COMBO_WORLD_MM
    elseif ootActive and mmActive then
        -- Both contexts have data - check game mode to determine active world
        -- OoT game mode is at offset 0x135c-0x135f
        if checkOotGameMode(ootSaveData) then
            return COMBO_WORLD_OOT
        else
            return COMBO_WORLD_MM
        end
    end

    return COMBO_WORLD_UNKNOWN
end

-- Detect game type from memory (enhanced with combo detection)
local function detectGameType(ootSaveData)
    -- If we haven't attempted combo ROM detection yet, do it now
    if not comboDetectionAttempted then
        comboDetectionAttempted = true
        local comboResult = detectComboRom()
        if comboResult ~= GAME_TYPE_UNKNOWN then
            if comboResult == GAME_TYPE_COMBO then
                -- Update active world detection for combo mode
                comboActiveWorld = detectComboActiveWorld(ootSaveData)
                local worldName = comboActiveWorld == COMBO_WORLD_OOT and "OoT" or
                                  (comboActiveWorld == COMBO_WORLD_MM and "MM" or "Unknown")
                print("Combo mode active world: " .. worldName)
            end
            return comboResult
        end
    end

    -- If combo ROM was previously detected, maintain that state
    if isComboRomDetected then
        -- Update active world detection
        comboActiveWorld = detectComboActiveWorld(ootSaveData)
        return GAME_TYPE_COMBO
    end

    -- Check for OoT via ZELDAZ magic
    if checkOotMagic(ootSaveData) then
        -- Double-check for combo randomizer context (in case ROM header check missed it)
        if checkComboContext() then
            isComboRomDetected = true
            comboActiveWorld = detectComboActiveWorld(ootSaveData)
            print("OoTMM combo detected via RAM check")
            return GAME_TYPE_COMBO
        end
        return GAME_TYPE_OOT
    end

    -- Check for MM by looking for non-zero data at MM save context
    if MM_SAVE_ADDR ~= nil then
        local success, mmData = pcall(function()
            return readMemoryBlock(RDRAM_BASE + MM_SAVE_ADDR, 4)
        end)

        if success and not isAllZeros(mmData) then
            -- Also check if this is actually a combo ROM with MM active
            if checkComboContext() then
                isComboRomDetected = true
                comboActiveWorld = COMBO_WORLD_MM
                print("OoTMM combo detected via MM context + combo check")
                return GAME_TYPE_COMBO
            end
            return GAME_TYPE_MM
        end
    end

    return GAME_TYPE_UNKNOWN
end

-- ============================================================================
-- Main Connection and Event Loop
-- ============================================================================

local function main()
    -- Connect to tracker
    local sock, err = socket.tcp("127.0.0.1", TCP_PORT)
    if not sock then
        print("Failed to connect to OoT Tracker: " .. (err or "unknown error"))
        return
    end

    -- Send version handshake
    -- Note: PJ64-EM's socket:send() returns nil even on success, so we use pcall
    -- and continue regardless of return value
    local handshake = binary.pack_u8(VERSION)
    local ok, result = pcall(function() return sock:send(handshake) end)
    if not ok then
        print("Failed to send handshake: " .. tostring(result))
        sock:close()
        return
    end

    print("Connected to OoT Tracker")

    -- State for tracking RAM changes
    local rawRam = nil
    local rawMmRam = nil
    local lastActiveWorld = COMBO_WORLD_UNKNOWN -- Track world changes

    -- Main loop function to be called each frame
    local function onFrame()
        local changed = true

        -- Read OoT RAM ranges
        if rawRam == nil then
            rawRam = {}
            local rangeIdx = 1
            for i = 1, #RAM_RANGES, 2 do
                local addr = RAM_RANGES[i]
                local size = RAM_RANGES[i + 1]
                rawRam[rangeIdx] = readMemoryBlock(RDRAM_BASE + addr, size)
                rangeIdx = rangeIdx + 1
            end
        else
            changed = false
            local rangeIdx = 1
            for i = 1, #RAM_RANGES, 2 do
                local addr = RAM_RANGES[i]
                local size = RAM_RANGES[i + 1]
                local newRange = readMemoryBlock(RDRAM_BASE + addr, size)
                if not arraysEqual(newRange, rawRam[rangeIdx]) then
                    rawRam[rangeIdx] = newRange
                    changed = true
                end
                rangeIdx = rangeIdx + 1
            end
        end

        -- Detect game type on first run or if changed
        if detectedGame == GAME_TYPE_UNKNOWN or changed then
            local previousGame = detectedGame
            detectedGame = detectGameType(rawRam[1])

            -- Log game type detection changes
            if detectedGame ~= previousGame then
                local gameTypeNames = {"Unknown", "OoT", "MM", "OoTMM Combo"}
                print("Game type detected: " .. gameTypeNames[detectedGame + 1])

                -- Log combo-specific info
                if detectedGame == GAME_TYPE_COMBO then
                    print("OoTMM combo ROM detected - isComboRomDetected=" .. tostring(isComboRomDetected))
                end
            end
        end

        -- For combo mode, track world changes
        if detectedGame == GAME_TYPE_COMBO then
            local newWorld = detectComboActiveWorld(rawRam[1])
            if newWorld ~= lastActiveWorld and newWorld ~= COMBO_WORLD_UNKNOWN then
                local worldNames = {"Unknown", "OoT", "MM"}
                print("Combo world switched to: " .. worldNames[newWorld + 1])
                lastActiveWorld = newWorld
                comboActiveWorld = newWorld
            end
        end

        -- Handle MM-only game (skip OoT processing)
        if detectedGame == GAME_TYPE_MM then
            rawMmRam, mmChanged = readMmRamRanges(rawMmRam)
            if mmChanged and rawMmRam ~= nil then
                sendMmRamData(sock, rawMmRam)
            end
            return
        end

        -- Handle OoT/Combo game
        if detectedGame == GAME_TYPE_OOT or detectedGame == GAME_TYPE_COMBO then
            -- For combo mode, check if we're in MM world
            if detectedGame == GAME_TYPE_COMBO and comboActiveWorld == COMBO_WORLD_MM then
                -- In MM world of combo - handle MM data instead
                if MM_RAM_RANGES ~= nil then
                    local mmChanged = true
                    if rawMmRam == nil then
                        rawMmRam = {}
                        local rangeIdx = 1
                        for i = 1, #MM_RAM_RANGES, 2 do
                            local addr = MM_RAM_RANGES[i]
                            local size = MM_RAM_RANGES[i + 1]
                            rawMmRam[rangeIdx] = readMemoryBlock(RDRAM_BASE + addr, size)
                            rangeIdx = rangeIdx + 1
                        end
                    else
                        mmChanged = false
                        local rangeIdx = 1
                        for i = 1, #MM_RAM_RANGES, 2 do
                            local addr = MM_RAM_RANGES[i]
                            local size = MM_RAM_RANGES[i + 1]
                            local newRange = readMemoryBlock(RDRAM_BASE + addr, size)
                            if not arraysEqual(newRange, rawMmRam[rangeIdx]) then
                                rawMmRam[rangeIdx] = newRange
                                mmChanged = true
                            end
                            rangeIdx = rangeIdx + 1
                        end
                    end
                    -- TODO: Send MM data when protocol supports combo mode
                end
                -- Also still read OoT ranges to keep save data updated for combo tracking
                -- (save data persists across world switches in OoTMM)
            end

            -- For combo mode, check MM changes independently of OoT changes
            -- (cross-game rewards can modify MM RAM while in OoT world)
            local mmChanged = false
            if detectedGame == GAME_TYPE_COMBO then
                rawMmRam, mmChanged = readMmRamRanges(rawMmRam)
                if mmChanged and rawMmRam ~= nil then
                    sendMmRamData(sock, rawMmRam)
                end
            end

            -- Now handle OoT data
            if not changed then return end
            if not checkOotMagic(rawRam[1]) then return end -- ZELDAZ magic number not present
            if not checkOotGameMode(rawRam[1]) then return end -- game mode ~= gameplay

            -- Send OoT RAM data
            sendOotRamData(sock, rawRam)
        end
    end

    -- Register the frame callback
    -- PJ64-EM uses emu.atvi() for VI (vertical interrupt) callbacks
    if emu and emu.atvi then
        emu.atvi(onFrame)
    elseif events and events.onframeend then
        -- Alternative callback registration
        events.onframeend(onFrame)
    else
        -- Fallback: run in a loop (less ideal but functional)
        print("Warning: No frame callback available, using polling loop")
        while true do
            onFrame()
            -- Small delay to avoid consuming too much CPU
            if emu and emu.pause then
                emu.pause()
            end
        end
    end
end

-- Start the tracker
main()
