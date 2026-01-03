# BizHawk Setup Guide for OoTMM Tracker

This guide explains how to set up BizHawk with an OoT/MM randomizer ROM to use with the oottracker.

## Prerequisites

1. **BizHawk Emulator** (v2.9+ recommended)
   - Download from: https://github.com/TASEmulators/BizHawk/releases
   - Extract to a folder (e.g., `C:\BizHawk` or `~/BizHawk`)

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

## Step 2: Install the OotAutoTracker BizHawk Plugin

1. Build the tracker (or download a release):
   ```bash
   cd oottracker
   cargo build --release -p oottracker-bizhawk
   ```

2. Copy the plugin files to BizHawk:
   - Copy `crate/oottracker-bizhawk/OotAutoTracker/` folder to:
     - Windows: `<BizHawk>/ExternalTools/OotAutoTracker/`
     - Linux: `~/.config/BizHawk/ExternalTools/OotAutoTracker/`

3. The folder should contain:
   - `OotAutoTracker.dll`
   - `oottracker.dll` (native library)

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

## Step 4: Connect BizHawk to the Tracker

1. **Start BizHawk** and load your randomized ROM

2. **Open the External Tool**:
   - Tools → External Tools → OotAutoTracker

3. **The tracker should auto-connect** to the GUI/web interface on port 24801

4. **Verify connection**:
   - The tracker GUI should show items updating as you collect them in-game
   - Check the status bar for "Connected" indicator

## Troubleshooting

### Plugin not appearing in BizHawk
- Ensure the DLL files are in the correct ExternalTools folder
- Check BizHawk's Tools → External Tools menu
- Restart BizHawk after adding the plugin

### Tracker not connecting
- Ensure the tracker GUI/web is running BEFORE opening the BizHawk plugin
- Check that port 24801 is not blocked by firewall
- Try restarting both the tracker and BizHawk

### Wrong game detected
- The tracker auto-detects OoT, MM, or Combo ROMs
- If detection fails, ensure you're using a supported ROM version
- Check BizHawk console for error messages

### Items not tracking
- Verify the ROM is a randomizer ROM (not vanilla)
- Check that you're using BizHawk's N64 core (not another emulator core)
- Try saving and reloading the game state

## Layout Options

The tracker supports multiple layouts:
- **OoT Default**: Standard OoT items only
- **MM Default**: Majora's Mask items only
- **Combo**: Both OoT and MM items together

Select your layout in the tracker GUI settings.

## Memory Addresses

For developers/debugging, key memory ranges:
- OoT Save: `0x11a5d0` (size: `0x1450`)
- MM Save: `0x1ef670` (size: `0x48d0`)
- Combo Context: `0x801c6fa0` (determines active game)

## Links

- OoTMM Randomizer: https://ootmm.com/
- OoT Randomizer: https://ootrandomizer.com/
- BizHawk: https://github.com/TASEmulators/BizHawk
- Tracker Source: https://github.com/spencerduncan/oottracker
