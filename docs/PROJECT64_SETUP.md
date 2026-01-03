# Project64 Setup Guide for OoTMM Tracker

This guide explains how to set up Project64 with an OoT/MM randomizer ROM to use with the oottracker.

## Prerequisites

1. **Project64** (v3.0+ recommended)
   - Download from: https://www.pj64-emu.com/
   - Windows only (use BizHawk for Linux/Mac)

2. **Legal ROM files** (you must own the original games)
   - Ocarina of Time (USA, v1.0)
   - Majora's Mask (USA)

3. **OoTMM Randomizer** (for combo randomizer)
   - Web generator: https://ootmm.com/
   - Or standalone: https://github.com/OoTMM/OoTMM

## Step 1: Generate a Randomized ROM

### For OoTMM Combo Randomizer:
1. Go to https://ootmm.com/
2. Upload your base OoT and MM ROMs
3. Configure your desired settings (or use a preset)
4. Click "Generate" to create the randomized ROM
5. Download the generated `.z64` file

### For OoT-only Randomizer:
1. Go to https://ootrandomizer.com/
2. Upload your base OoT ROM
3. Configure settings
4. Generate and download

## Step 2: Install the Project64 Script

1. Locate the tracker script:
   ```
   oottracker/assets/oottracker-pj64.js
   ```

2. Copy the script to Project64's scripts folder:
   - Default location: `C:\Program Files (x86)\Project64\Scripts\`
   - Or: `<Project64 Install Dir>\Scripts\`

3. If the Scripts folder doesn't exist, create it.

## Step 3: Run the Tracker GUI

1. Build and run the tracker GUI:
   ```bash
   cargo run --release -p oottracker-gui
   ```

2. Or use the web version:
   ```bash
   cargo run --release -p oottracker-web
   ```
   Then open http://localhost:24800 in your browser

## Step 4: Connect Project64 to the Tracker

1. **Start Project64** and load your randomized ROM

2. **Enable the Script**:
   - System → Scripts
   - Check the box next to `oottracker-pj64.js`
   - Click "Run"

3. **The tracker should auto-connect** to the GUI/web interface on port 24801

4. **Verify connection**:
   - The tracker GUI should show items updating as you collect them in-game
   - Check the Project64 console for connection status messages

## Step 5: Configure Script Settings (Optional)

The script supports some configuration at the top of the file:

```javascript
// Connection settings
const TRACKER_HOST = "127.0.0.1";
const TRACKER_PORT = 24801;

// Memory polling interval (ms)
const POLL_INTERVAL = 100;
```

Edit these if running the tracker on a different machine or port.

## Troubleshooting

### Script not appearing in Project64
- Ensure the `.js` file is in the Scripts folder
- Restart Project64 after adding the script
- Check that JavaScript scripting is enabled in Project64 settings

### Tracker not connecting
- Ensure the tracker GUI/web is running BEFORE enabling the script
- Check that port 24801 is not blocked by firewall
- Look for error messages in Project64's script console

### "Socket connection failed" error
- Verify the tracker is running and listening
- Try disabling and re-enabling the script
- Check Windows Firewall settings

### Items not tracking correctly
- Verify the ROM is a randomizer ROM (not vanilla)
- Check that you're using a supported ROM version (USA)
- Try saving and reloading your game state
- Restart both Project64 and the tracker

### Wrong game detected
- The script auto-detects OoT, MM, or Combo ROMs
- If detection fails, check the script console for debug info
- Ensure ROM header matches expected format

## Memory Addresses (Reference)

For developers/debugging, key memory ranges used by the script:

| Game | Save Address | Size |
|------|-------------|------|
| OoT | `0x11a5d0` | `0x1450` |
| MM | `0x1ef670` | `0x48d0` |
| Combo Context | `0x801c6fa0` | Determines active game |

## Layout Options

The tracker supports multiple layouts:
- **OoT Default**: Standard OoT items only
- **MM Default**: Majora's Mask items only
- **Combo**: Both OoT and MM items together

Select your layout in the tracker GUI settings.

## Limitations vs BizHawk

Project64 has some limitations compared to BizHawk:
- Windows only (BizHawk supports Linux/Mac)
- JavaScript scripting may have fewer features
- Memory access can be less reliable in some cases

For the most robust experience, BizHawk is recommended.

## Links

- OoTMM Randomizer: https://ootmm.com/
- OoT Randomizer: https://ootrandomizer.com/
- Project64: https://www.pj64-emu.com/
- Tracker Source: https://github.com/spencerduncan/oottracker
