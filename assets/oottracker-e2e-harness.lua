-- OoTTracker E2E Test Harness Lua Script
--
-- This script extends the base tracker script with E2E testing capabilities.
-- It provides additional hooks for automated testing, including:
-- - Test synchronization via TCP commands
-- - State injection for fixture loading
-- - Event logging for test verification
--
-- Usage: Load this script in Project64-EM for E2E testing

-- Configuration (can be overridden by test harness)
local E2E_TEST_PORT = 24802  -- Separate port for test control
local E2E_LOG_LEVEL = 2      -- 0 = none, 1 = errors, 2 = info, 3 = debug

-- Protocol version for E2E test harness
local E2E_VERSION = 1

-- Test control packet types
local CMD_PING = 0x01
local CMD_PONG = 0x02
local CMD_LOAD_STATE = 0x10
local CMD_SAVE_STATE = 0x11
local CMD_INJECT_RAM = 0x20
local CMD_READ_RAM = 0x21
local CMD_GET_STATUS = 0x30
local CMD_STATUS_RESPONSE = 0x31
local CMD_RESET = 0x40
local CMD_QUIT = 0xFF

-- Response codes
local RESP_OK = 0x00
local RESP_ERROR = 0x01
local RESP_BUSY = 0x02

-- Test state
local testSocket = nil
local testConnected = false
local eventLog = {}
local maxEventLogSize = 1000

-- ============================================================================
-- Logging Functions
-- ============================================================================

local function log(level, msg)
    if level <= E2E_LOG_LEVEL then
        local prefix = level == 1 and "[E2E ERROR] " or
                       level == 2 and "[E2E INFO] " or
                       "[E2E DEBUG] "
        print(prefix .. msg)
    end
end

local function logError(msg) log(1, msg) end
local function logInfo(msg) log(2, msg) end
local function logDebug(msg) log(3, msg) end

-- ============================================================================
-- Event Logging
-- ============================================================================

-- Log an event that occurred during testing
local function logEvent(eventType, data)
    local event = {
        timestamp = os.time(),
        frame = emu and emu.framecount and emu.framecount() or 0,
        type = eventType,
        data = data
    }

    table.insert(eventLog, event)

    -- Trim log if it gets too large
    while #eventLog > maxEventLogSize do
        table.remove(eventLog, 1)
    end

    logDebug("Event: " .. eventType .. " - " .. tostring(data))
end

-- Clear the event log
local function clearEventLog()
    eventLog = {}
    logInfo("Event log cleared")
end

-- Get events since a given timestamp
local function getEventsSince(timestamp)
    local events = {}
    for _, event in ipairs(eventLog) do
        if event.timestamp >= timestamp then
            table.insert(events, event)
        end
    end
    return events
end

-- ============================================================================
-- RAM Manipulation
-- ============================================================================

-- Read a block of RAM
local function readRamBlock(addr, size)
    local data = {}
    for i = 0, size - 1 do
        local success, byte = pcall(function()
            return memory.read_u8(addr + i)
        end)
        if success then
            data[i + 1] = byte
        else
            data[i + 1] = 0
        end
    end
    return data
end

-- Write a block of RAM
local function writeRamBlock(addr, data)
    local success = true
    for i, byte in ipairs(data) do
        local ok = pcall(function()
            memory.write_u8(addr + i - 1, byte)
        end)
        if not ok then
            success = false
            logError("Failed to write byte at " .. string.format("0x%08X", addr + i - 1))
        end
    end
    return success
end

-- Inject a save context from fixture data
local function injectSaveContext(saveData)
    local RDRAM_BASE = 0x80000000
    local SAVE_ADDR = 0x11A5D0  -- OoT save context address

    logInfo("Injecting save context (" .. #saveData .. " bytes)")

    local success = writeRamBlock(RDRAM_BASE + SAVE_ADDR, saveData)

    if success then
        logEvent("save_context_injected", { size = #saveData })
    else
        logError("Failed to inject save context")
    end

    return success
end

-- ============================================================================
-- State Management
-- ============================================================================

-- Get current game status
local function getGameStatus()
    local RDRAM_BASE = 0x80000000
    local SAVE_ADDR = 0x11A5D0

    -- Read key fields from save context
    local status = {
        connected = testConnected,
        eventCount = #eventLog
    }

    -- Try to read ZELDAZ magic to verify game is running
    local magicOk, magic = pcall(function()
        local m = {}
        for i = 0, 5 do
            m[i + 1] = memory.read_u8(RDRAM_BASE + SAVE_ADDR + 0x1C + i)
        end
        return m
    end)

    if magicOk then
        local isZeldaz = magic[1] == 0x5A and magic[2] == 0x45 and
                         magic[3] == 0x4C and magic[4] == 0x44 and
                         magic[5] == 0x41 and magic[6] == 0x5A
        status.gameActive = isZeldaz
    else
        status.gameActive = false
    end

    -- Try to get frame count
    if emu and emu.framecount then
        status.frameCount = emu.framecount()
    end

    return status
end

-- ============================================================================
-- Network Protocol Handling
-- ============================================================================

-- Process a command packet
local function processCommand(cmd, payload)
    logDebug("Processing command: " .. string.format("0x%02X", cmd))

    if cmd == CMD_PING then
        -- Respond with pong
        return CMD_PONG, { E2E_VERSION }

    elseif cmd == CMD_GET_STATUS then
        -- Return game status
        local status = getGameStatus()
        local response = {
            status.connected and 1 or 0,
            status.gameActive and 1 or 0,
            (status.frameCount or 0) % 256,
            math.floor((status.frameCount or 0) / 256) % 256,
            status.eventCount % 256,
            math.floor(status.eventCount / 256) % 256
        }
        return CMD_STATUS_RESPONSE, response

    elseif cmd == CMD_INJECT_RAM then
        -- Inject RAM data
        -- Payload format: 4 bytes address (big-endian), then data
        if #payload < 5 then
            return RESP_ERROR, { 0x01 }  -- Invalid payload
        end

        local addr = payload[1] * 0x1000000 + payload[2] * 0x10000 +
                     payload[3] * 0x100 + payload[4]
        local data = {}
        for i = 5, #payload do
            data[i - 4] = payload[i]
        end

        if writeRamBlock(addr, data) then
            logEvent("ram_injected", { addr = addr, size = #data })
            return RESP_OK, {}
        else
            return RESP_ERROR, { 0x02 }  -- Write failed
        end

    elseif cmd == CMD_READ_RAM then
        -- Read RAM data
        -- Payload format: 4 bytes address (big-endian), 2 bytes size (big-endian)
        if #payload < 6 then
            return RESP_ERROR, { 0x01 }  -- Invalid payload
        end

        local addr = payload[1] * 0x1000000 + payload[2] * 0x10000 +
                     payload[3] * 0x100 + payload[4]
        local size = payload[5] * 0x100 + payload[6]

        local data = readRamBlock(addr, size)
        return RESP_OK, data

    elseif cmd == CMD_LOAD_STATE then
        -- Load save state (if emulator supports it)
        if savestate and savestate.loadslot then
            local slot = payload[1] or 1
            savestate.loadslot(slot)
            logEvent("state_loaded", { slot = slot })
            return RESP_OK, {}
        else
            return RESP_ERROR, { 0x03 }  -- Not supported
        end

    elseif cmd == CMD_SAVE_STATE then
        -- Save state (if emulator supports it)
        if savestate and savestate.saveslot then
            local slot = payload[1] or 1
            savestate.saveslot(slot)
            logEvent("state_saved", { slot = slot })
            return RESP_OK, {}
        else
            return RESP_ERROR, { 0x03 }  -- Not supported
        end

    elseif cmd == CMD_RESET then
        -- Clear event log
        clearEventLog()
        return RESP_OK, {}

    elseif cmd == CMD_QUIT then
        -- Disconnect test control
        logInfo("Received quit command")
        testConnected = false
        return RESP_OK, {}

    else
        logError("Unknown command: " .. string.format("0x%02X", cmd))
        return RESP_ERROR, { 0xFF }  -- Unknown command
    end
end

-- Send a response packet
local function sendResponse(sock, respType, data)
    local packet = string.char(respType)
    for _, byte in ipairs(data or {}) do
        packet = packet .. string.char(byte)
    end

    local success, err = sock:send(packet)
    if not success then
        logError("Failed to send response: " .. (err or "unknown error"))
    end
end

-- ============================================================================
-- Test Control Server
-- ============================================================================

-- Accept test control connections
local function acceptTestConnection()
    if testSocket then
        -- Check for incoming data
        testSocket:settimeout(0)  -- Non-blocking

        local data, err = testSocket:receive(1)
        if data then
            local cmd = string.byte(data)

            -- Read payload length (if any)
            local lenData = testSocket:receive(2)
            local payloadLen = 0
            if lenData and #lenData == 2 then
                payloadLen = string.byte(lenData, 1) * 256 + string.byte(lenData, 2)
            end

            -- Read payload
            local payload = {}
            if payloadLen > 0 then
                local payloadData = testSocket:receive(payloadLen)
                if payloadData then
                    for i = 1, #payloadData do
                        payload[i] = string.byte(payloadData, i)
                    end
                end
            end

            -- Process command
            local respType, respData = processCommand(cmd, payload)
            sendResponse(testSocket, respType, respData)
        elseif err ~= "timeout" then
            logError("Test connection error: " .. (err or "unknown"))
            testConnected = false
        end
    end
end

-- Initialize test control server
local function initTestServer()
    local success, sock = pcall(function()
        return socket.tcp("0.0.0.0", E2E_TEST_PORT)
    end)

    if success and sock then
        logInfo("E2E test server listening on port " .. E2E_TEST_PORT)
        testSocket = sock
        testConnected = true

        -- Send version handshake
        sendResponse(sock, E2E_VERSION, {})
    else
        logError("Failed to start E2E test server: " .. tostring(sock))
    end
end

-- ============================================================================
-- Frame Callback with Test Integration
-- ============================================================================

-- State tracking for change detection
local previousState = nil

-- Detect and log state changes
local function detectStateChanges()
    local RDRAM_BASE = 0x80000000
    local SAVE_ADDR = 0x11A5D0

    -- Read current state
    local currentState = {
        -- Quest status (stones and medallions)
        questStatus = readRamBlock(RDRAM_BASE + SAVE_ADDR + 0xA4, 4),
        -- Equipment
        equipment = readRamBlock(RDRAM_BASE + SAVE_ADDR + 0x9C, 4),
        -- Inventory
        inventory = readRamBlock(RDRAM_BASE + SAVE_ADDR + 0x74, 24),
        -- Health
        health = readRamBlock(RDRAM_BASE + SAVE_ADDR + 0x2E, 4)
    }

    if previousState then
        -- Check for quest status changes (medallions/stones)
        if not arraysEqual(currentState.questStatus, previousState.questStatus) then
            logEvent("quest_status_changed", {
                old = previousState.questStatus,
                new = currentState.questStatus
            })
        end

        -- Check for equipment changes
        if not arraysEqual(currentState.equipment, previousState.equipment) then
            logEvent("equipment_changed", {
                old = previousState.equipment,
                new = currentState.equipment
            })
        end

        -- Check for inventory changes
        if not arraysEqual(currentState.inventory, previousState.inventory) then
            logEvent("inventory_changed", {
                old = previousState.inventory,
                new = currentState.inventory
            })
        end

        -- Check for health changes
        if not arraysEqual(currentState.health, previousState.health) then
            logEvent("health_changed", {
                old = previousState.health,
                new = currentState.health
            })
        end
    end

    previousState = currentState
end

-- Helper to compare byte arrays
local function arraysEqual(a, b)
    if #a ~= #b then return false end
    for i = 1, #a do
        if a[i] ~= b[i] then return false end
    end
    return true
end

-- E2E frame callback
local function e2eFrameCallback()
    -- Process test commands
    if testConnected then
        acceptTestConnection()
    end

    -- Detect and log state changes
    detectStateChanges()
end

-- ============================================================================
-- Main Entry Point
-- ============================================================================

local function main()
    logInfo("OoTTracker E2E Test Harness v" .. E2E_VERSION)
    logInfo("Initializing test infrastructure...")

    -- Initialize test server
    initTestServer()

    -- Register frame callback
    if emu and emu.atvi then
        emu.atvi(e2eFrameCallback)
        logInfo("Registered VI callback")
    elseif events and events.onframeend then
        events.onframeend(e2eFrameCallback)
        logInfo("Registered frame end callback")
    else
        logError("No frame callback mechanism available!")
    end

    logInfo("E2E test harness initialized")
    logEvent("harness_initialized", { version = E2E_VERSION })
end

-- Export functions for external use
_G.e2e = {
    logEvent = logEvent,
    clearEventLog = clearEventLog,
    getEventsSince = getEventsSince,
    injectSaveContext = injectSaveContext,
    getGameStatus = getGameStatus,
    readRamBlock = readRamBlock,
    writeRamBlock = writeRamBlock
}

-- Start the harness
main()
