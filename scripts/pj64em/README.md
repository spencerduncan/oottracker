# OoTTracker PJ64-EM Integration

Lua script for tracking OoT, MM, and OoTMM combo randomizers in Project64-EM.

## Requirements

- [Project64-EM](https://github.com/Project64-EM/Project64-EM) (Windows, works via Wine on Linux)
- OoTTracker GUI running and listening on port 24801

## Installation

1. Copy `oottracker.lua` to your Project64-EM `Scripts` folder
2. Start OoTTracker GUI
3. In Project64-EM: load your ROM
4. In Project64-EM: go to `Debugger` → `Scripts` → select `oottracker.lua` → `Run`

## Supported ROMs

- **Ocarina of Time** (vanilla randomizer)
- **Majora's Mask** (vanilla randomizer)
- **OoTMM Combo** (combined randomizer) - auto-detected via RAM signature

## Features

- Automatic game type detection (OoT, MM, or Combo)
- Real-time item tracking as you play
- Cross-game item tracking in combo mode (items obtained in OoT that affect MM and vice versa)
- World switch detection in combo mode

## Wine Notes (Linux)

Project64-EM runs under Wine. A 32-bit Wine prefix is recommended:

```bash
WINEPREFIX=~/.wine-pj64-32 WINEARCH=win32 wine Project64-EM.exe
```

If you get "Plugins not initialized" error on subsequent runs:
1. Go to Options → Settings → Plugins
2. Re-select each plugin (Video, Audio, Input, RSP)
3. Click OK, then load your ROM

## Technical Details

- TCP port: 24801 (connects to OoTTracker)
- Protocol version: 6
- OoT save context: 0x11a5d0 (size 0x1450)
- MM save context: 0x1ef670 (size 0x48d0)

### PJ64-EM Socket Quirk

PJ64-EM's Lua socket implementation returns `nil` from `send()` even on success. The script uses `pcall()` and continues regardless of return value.

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "Failed to connect to OoT Tracker" | Make sure OoTTracker GUI is running |
| No items tracking | Ensure ROM is loaded before running script |
| MM items not showing | Combo layout display is WIP (see issue #461) |
