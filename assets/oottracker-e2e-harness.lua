-- OoT Tracker E2E Test Harness for Project64-EM
-- This script provides emulator control and memory access for automated E2E testing.
-- It runs alongside the main tracker script (oottracker-pj64em-base.lua).

local VERSION = 1

-- Default port for E2E test harness (different from tracker port)
local E2E_PORT = 43435

-- RDRAM base address for N64
local RDRAM_BASE = 0x80000000

-- Command types received from test runner
local CMD_PING = 0x01
local CMD_SAVE_STATE = 0x02
local CMD_LOAD_STATE = 0x03
local CMD_ADVANCE_FRAMES = 0x04
local CMD_READ_MEMORY = 0x05
local CMD_WRITE_MEMORY = 0x06
local CMD_GET_FRAME_COUNT = 0x07
local CMD_RESET = 0x08
local CMD_PAUSE = 0x09
local CMD_RESUME = 0x0A
local CMD_SET_INPUT = 0x0B

-- Response types sent to test runner
local RESP_OK = 0x00
local RESP_ERROR = 0x01
local RESP_DATA = 0x02
local RESP_PONG = 0x03

-- State management
local serverSocket = nil
local clientSocket = nil
local isConnected = false
local framesToAdvance = 0
local isPaused = false
local pendingInput = nil
local frameCount = 0

-- Save state slots (in-memory state storage for quick save/load)
local saveStateSlots = {}

-- ============================================================================
-- Helper Functions
-- ============================================================================

-- Pack a 32-bit unsigned integer in big-endian format
local function packU32BE(value)
    return binary.pack_u8(bit.band(bit.rshift(value, 24), 0xFF)) ..
           binary.pack_u8(bit.band(bit.rshift(value, 16), 0xFF)) ..
           binary.pack_u8(bit.band(bit.rshift(value, 8), 0xFF)) ..
           binary.pack_u8(bit.band(value, 0xFF))
end

-- Unpack a 32-bit unsigned integer from big-endian bytes
local function unpackU32BE(b1, b2, b3, b4)
    return bit.bor(
        bit.lshift(b1, 24),
        bit.lshift(b2, 16),
        bit.lshift(b3, 8),
        b4
    )
end

-- Pack a 16-bit unsigned integer in big-endian format
local function packU16BE(value)
    return binary.pack_u8(bit.band(bit.rshift(value, 8), 0xFF)) ..
           binary.pack_u8(bit.band(value, 0xFF))
end

-- Unpack a 16-bit unsigned integer from big-endian bytes
local function unpackU16BE(b1, b2)
    return bit.bor(bit.lshift(b1, 8), b2)
end

-- Read a block of memory
local function readMemoryBlock(addr, size)
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

-- Write a block of memory
local function writeMemoryBlock(addr, data)
    for i = 1, #data do
        local success, err = pcall(function()
            memory.write_u8(addr + i - 1, data[i])
        end)
        if not success then
            return false, "Failed to write at offset " .. (i - 1) .. ": " .. tostring(err)
        end
    end
    return true
end

-- ============================================================================
-- Response Builders
-- ============================================================================

-- Send OK response
local function sendOk(sock)
    if not sock then return end
    local response = binary.pack_u8(RESP_OK)
    sock:send(response)
end

-- Send error response with message
local function sendError(sock, message)
    if not sock then return end
    local msgBytes = {string.byte(message, 1, #message)}
    local response = binary.pack_u8(RESP_ERROR) .. binary.pack_u8(#message)
    for i = 1, #msgBytes do
        response = response .. binary.pack_u8(msgBytes[i])
    end
    sock:send(response)
end

-- Send data response
local function sendData(sock, data)
    if not sock then return end
    local response = binary.pack_u8(RESP_DATA) .. packU32BE(#data)
    for i = 1, #data do
        response = response .. binary.pack_u8(data[i])
    end
    sock:send(response)
end

-- Send pong response with version
local function sendPong(sock)
    if not sock then return end
    local response = binary.pack_u8(RESP_PONG) .. binary.pack_u8(VERSION)
    sock:send(response)
end

-- ============================================================================
-- Command Handlers
-- ============================================================================

-- Handle PING command
local function handlePing(sock)
    print("[E2E] Received PING")
    sendPong(sock)
end

-- Handle SAVE_STATE command
-- Format: CMD_SAVE_STATE + slot (1 byte)
local function handleSaveState(sock, slot)
    print("[E2E] Save state to slot " .. slot)

    -- For PJ64-EM, we use savestate functions if available
    if emu and emu.savestate then
        local success, err = pcall(function()
            emu.savestate(slot)
        end)
        if success then
            sendOk(sock)
        else
            sendError(sock, "savestate failed: " .. tostring(err))
        end
    else
        -- Fallback: save RDRAM to internal slot
        local saveData = {}
        -- Save critical memory regions (save context + some RAM)
        local regions = {
            {0x11A5D0, 0x1450}, -- OoT save context
            {0x1C8544, 0x0038}, -- OoT scene flags area
        }

        for _, region in ipairs(regions) do
            local addr = region[1]
            local size = region[2]
            local data = readMemoryBlock(RDRAM_BASE + addr, size)
            saveData[addr] = data
        end

        saveStateSlots[slot] = saveData
        sendOk(sock)
    end
end

-- Handle LOAD_STATE command
-- Format: CMD_LOAD_STATE + slot (1 byte)
local function handleLoadState(sock, slot)
    print("[E2E] Load state from slot " .. slot)

    if emu and emu.loadstate then
        local success, err = pcall(function()
            emu.loadstate(slot)
        end)
        if success then
            sendOk(sock)
        else
            sendError(sock, "loadstate failed: " .. tostring(err))
        end
    else
        -- Fallback: restore from internal slot
        local saveData = saveStateSlots[slot]
        if not saveData then
            sendError(sock, "No save state in slot " .. slot)
            return
        end

        for addr, data in pairs(saveData) do
            writeMemoryBlock(RDRAM_BASE + addr, data)
        end
        sendOk(sock)
    end
end

-- Handle ADVANCE_FRAMES command
-- Format: CMD_ADVANCE_FRAMES + frame_count (4 bytes, big-endian)
local function handleAdvanceFrames(sock, count)
    print("[E2E] Advance " .. count .. " frames")
    framesToAdvance = count
    isPaused = false
    sendOk(sock)
end

-- Handle READ_MEMORY command
-- Format: CMD_READ_MEMORY + address (4 bytes) + size (4 bytes)
local function handleReadMemory(sock, address, size)
    print("[E2E] Read " .. size .. " bytes from 0x" .. string.format("%08X", address))

    if size > 65536 then
        sendError(sock, "Read size too large (max 65536)")
        return
    end

    local data = readMemoryBlock(address, size)
    sendData(sock, data)
end

-- Handle WRITE_MEMORY command
-- Format: CMD_WRITE_MEMORY + address (4 bytes) + size (4 bytes) + data
local function handleWriteMemory(sock, address, data)
    print("[E2E] Write " .. #data .. " bytes to 0x" .. string.format("%08X", address))

    local success, err = writeMemoryBlock(address, data)
    if success then
        sendOk(sock)
    else
        sendError(sock, err)
    end
end

-- Handle GET_FRAME_COUNT command
local function handleGetFrameCount(sock)
    local response = binary.pack_u8(RESP_DATA) .. packU32BE(4) .. packU32BE(frameCount)
    sock:send(response)
end

-- Handle RESET command
local function handleReset(sock)
    print("[E2E] Reset emulator")

    if emu and emu.reset then
        local success, err = pcall(function()
            emu.reset()
        end)
        if success then
            frameCount = 0
            sendOk(sock)
        else
            sendError(sock, "reset failed: " .. tostring(err))
        end
    else
        sendError(sock, "reset not supported")
    end
end

-- Handle PAUSE command
local function handlePause(sock)
    print("[E2E] Pause emulator")
    isPaused = true

    if emu and emu.pause then
        pcall(function() emu.pause() end)
    end

    sendOk(sock)
end

-- Handle RESUME command
local function handleResume(sock)
    print("[E2E] Resume emulator")
    isPaused = false

    if emu and emu.unpause then
        pcall(function() emu.unpause() end)
    end

    sendOk(sock)
end

-- Handle SET_INPUT command
-- Format: CMD_SET_INPUT + controller (1 byte) + buttons (2 bytes) + stick_x (1 byte) + stick_y (1 byte)
local function handleSetInput(sock, controller, buttons, stickX, stickY)
    print("[E2E] Set input for controller " .. controller)

    pendingInput = {
        controller = controller,
        buttons = buttons,
        stickX = stickX,
        stickY = stickY
    }

    sendOk(sock)
end

-- ============================================================================
-- Network Communication
-- ============================================================================

-- Process incoming data from client
local function processClientData(sock)
    -- Try to receive data (non-blocking)
    local data, err = sock:receive(1)
    if not data then
        if err == "timeout" then
            return true -- No data available, continue
        end
        return false -- Connection error
    end

    local cmd = string.byte(data, 1)

    if cmd == CMD_PING then
        handlePing(sock)

    elseif cmd == CMD_SAVE_STATE then
        local slotData = sock:receive(1)
        if slotData then
            handleSaveState(sock, string.byte(slotData, 1))
        end

    elseif cmd == CMD_LOAD_STATE then
        local slotData = sock:receive(1)
        if slotData then
            handleLoadState(sock, string.byte(slotData, 1))
        end

    elseif cmd == CMD_ADVANCE_FRAMES then
        local countData = sock:receive(4)
        if countData then
            local count = unpackU32BE(
                string.byte(countData, 1),
                string.byte(countData, 2),
                string.byte(countData, 3),
                string.byte(countData, 4)
            )
            handleAdvanceFrames(sock, count)
        end

    elseif cmd == CMD_READ_MEMORY then
        local addrSizeData = sock:receive(8)
        if addrSizeData then
            local address = unpackU32BE(
                string.byte(addrSizeData, 1),
                string.byte(addrSizeData, 2),
                string.byte(addrSizeData, 3),
                string.byte(addrSizeData, 4)
            )
            local size = unpackU32BE(
                string.byte(addrSizeData, 5),
                string.byte(addrSizeData, 6),
                string.byte(addrSizeData, 7),
                string.byte(addrSizeData, 8)
            )
            handleReadMemory(sock, address, size)
        end

    elseif cmd == CMD_WRITE_MEMORY then
        local headerData = sock:receive(8)
        if headerData then
            local address = unpackU32BE(
                string.byte(headerData, 1),
                string.byte(headerData, 2),
                string.byte(headerData, 3),
                string.byte(headerData, 4)
            )
            local size = unpackU32BE(
                string.byte(headerData, 5),
                string.byte(headerData, 6),
                string.byte(headerData, 7),
                string.byte(headerData, 8)
            )

            if size > 0 and size <= 65536 then
                local memData = sock:receive(size)
                if memData then
                    local dataTable = {}
                    for i = 1, size do
                        dataTable[i] = string.byte(memData, i)
                    end
                    handleWriteMemory(sock, address, dataTable)
                end
            else
                sendError(sock, "Invalid write size")
            end
        end

    elseif cmd == CMD_GET_FRAME_COUNT then
        handleGetFrameCount(sock)

    elseif cmd == CMD_RESET then
        handleReset(sock)

    elseif cmd == CMD_PAUSE then
        handlePause(sock)

    elseif cmd == CMD_RESUME then
        handleResume(sock)

    elseif cmd == CMD_SET_INPUT then
        local inputData = sock:receive(5)
        if inputData then
            local controller = string.byte(inputData, 1)
            local buttons = unpackU16BE(
                string.byte(inputData, 2),
                string.byte(inputData, 3)
            )
            local stickX = string.byte(inputData, 4)
            local stickY = string.byte(inputData, 5)
            handleSetInput(sock, controller, buttons, stickX, stickY)
        end

    else
        sendError(sock, "Unknown command: " .. cmd)
    end

    return true
end

-- Accept new client connection
local function acceptClient()
    if not serverSocket then return end

    local client, err = serverSocket:accept()
    if client then
        client:settimeout(0) -- Non-blocking
        clientSocket = client
        isConnected = true
        print("[E2E] Test runner connected")
    end
end

-- ============================================================================
-- Frame Callback
-- ============================================================================

local function onFrame()
    frameCount = frameCount + 1

    -- Accept new connections if no client
    if not isConnected then
        acceptClient()
    end

    -- Process client commands
    if clientSocket and isConnected then
        local success = processClientData(clientSocket)
        if not success then
            print("[E2E] Client disconnected")
            clientSocket:close()
            clientSocket = nil
            isConnected = false
        end
    end

    -- Handle frame advancement
    if framesToAdvance > 0 then
        framesToAdvance = framesToAdvance - 1
        if framesToAdvance == 0 and emu and emu.pause then
            -- Pause after advancing requested frames
            pcall(function() emu.pause() end)
        end
    end

    -- Apply pending input
    if pendingInput and joypad then
        local success = pcall(function()
            joypad.set(pendingInput.controller, {
                A = bit.band(pendingInput.buttons, 0x8000) ~= 0,
                B = bit.band(pendingInput.buttons, 0x4000) ~= 0,
                Z = bit.band(pendingInput.buttons, 0x2000) ~= 0,
                Start = bit.band(pendingInput.buttons, 0x1000) ~= 0,
                DUp = bit.band(pendingInput.buttons, 0x0800) ~= 0,
                DDown = bit.band(pendingInput.buttons, 0x0400) ~= 0,
                DLeft = bit.band(pendingInput.buttons, 0x0200) ~= 0,
                DRight = bit.band(pendingInput.buttons, 0x0100) ~= 0,
                L = bit.band(pendingInput.buttons, 0x0020) ~= 0,
                R = bit.band(pendingInput.buttons, 0x0010) ~= 0,
                CUp = bit.band(pendingInput.buttons, 0x0008) ~= 0,
                CDown = bit.band(pendingInput.buttons, 0x0004) ~= 0,
                CLeft = bit.band(pendingInput.buttons, 0x0002) ~= 0,
                CRight = bit.band(pendingInput.buttons, 0x0001) ~= 0,
                X = pendingInput.stickX - 128,
                Y = pendingInput.stickY - 128
            })
        end)
        -- Clear input after applying (one-shot)
        pendingInput = nil
    end
end

-- ============================================================================
-- Main Entry Point
-- ============================================================================

local function main()
    print("[E2E] OoT Tracker E2E Test Harness v" .. VERSION)
    print("[E2E] Starting TCP server on port " .. E2E_PORT)

    -- Create TCP server socket
    local server, err = socket.tcp()
    if not server then
        print("[E2E] Failed to create socket: " .. (err or "unknown error"))
        return
    end

    -- Allow address reuse
    server:setoption("reuseaddr", true)

    -- Bind to port
    local success, bindErr = server:bind("127.0.0.1", E2E_PORT)
    if not success then
        print("[E2E] Failed to bind to port " .. E2E_PORT .. ": " .. (bindErr or "unknown error"))
        server:close()
        return
    end

    -- Start listening
    success, err = server:listen(1)
    if not success then
        print("[E2E] Failed to listen: " .. (err or "unknown error"))
        server:close()
        return
    end

    -- Set non-blocking for accept
    server:settimeout(0)
    serverSocket = server

    print("[E2E] Server ready, waiting for test runner connection...")

    -- Register frame callback
    if emu and emu.atvi then
        emu.atvi(onFrame)
    elseif events and events.onframeend then
        events.onframeend(onFrame)
    else
        print("[E2E] Warning: No frame callback available, using polling loop")
        while true do
            onFrame()
            -- Small delay to avoid consuming too much CPU
            if emu and emu.frameadvance then
                emu.frameadvance()
            end
        end
    end
end

-- Export functions for use by other scripts
E2EHarness = {
    VERSION = VERSION,
    PORT = E2E_PORT,
    readMemoryBlock = readMemoryBlock,
    writeMemoryBlock = writeMemoryBlock,
    isConnected = function() return isConnected end,
    getFrameCount = function() return frameCount end
}

-- Start the harness
main()
