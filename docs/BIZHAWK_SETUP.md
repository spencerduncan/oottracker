# BizHawk Setup Guide for OoTMM Tracker

This guide explains how to set up BizHawk with an OoT/MM randomizer ROM to use with the oottracker.

## Prerequisites

1. **BizHawk Emulator** (v2.9+ required)
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

## Step 2: Build the OotAutoTracker BizHawk Plugin

### Requirements
- [Rust toolchain](https://rustup.rs/) (for building the native library)
- [.NET SDK 6.0+](https://dotnet.microsoft.com/download) (for building the C# plugin)
- BizHawk 2.9+ (for reference DLLs during compilation)

### Quick Setup (Recommended)

Use the setup script to configure your development environment:

```bash
# Run the setup script with your BizHawk installation path
./scripts/setup-bizhawk-dev.sh /path/to/BizHawk

# Example paths:
./scripts/setup-bizhawk-dev.sh ~/Games/BizHawk-2.9.1
./scripts/setup-bizhawk-dev.sh /opt/BizHawk
./scripts/setup-bizhawk-dev.sh "C:\BizHawk"  # Windows (use quotes)
```

The script will:
- Validate your BizHawk installation
- Create a symlink for the build process
- Check for required tools (.NET SDK, Rust)

### Build the Plugin

```bash
# 1. Build the Rust FFI library (produces oottracker.dll)
cargo build --release -p oottracker-csharp

# 2. Build the BizHawk plugin (compiles C# wrapper)
cargo build --release -p oottracker-bizhawk
```

After building, the plugin files are automatically placed in your BizHawk's ExternalTools folder (via the symlink).

### Manual Setup (Alternative)

If you prefer not to use the script:

1. Create a symlink from your BizHawk installation:
   ```bash
   ln -s /path/to/BizHawk crate/oottracker-bizhawk/OotAutoTracker/BizHawk
   ```

2. Build as shown above.

### Output Files

After a successful build:
- `OotAutoTracker.dll` - The C# BizHawk plugin
- `oottracker.dll` - The Rust native library

Both are placed in `<BizHawk>/ExternalTools/` (via the symlink).

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

3. **The plugin connects to the tracker** on port 24801
   - The tracker GUI/web runs on port 24800
   - The BizHawk plugin connects on port 24801

4. **Verify connection**:
   - The tracker GUI should show items updating as you collect them in-game
   - Check the status bar for "Connected" indicator

## Troubleshooting

### Plugin not appearing in BizHawk
- Ensure both DLL files are in the ExternalTools folder
- Check that you built with `--release` flag
- Restart BizHawk after adding the plugin
- Verify BizHawk version is 2.9+

### Build fails: "BizHawk DLLs not found"
- Run the setup script first: `./scripts/setup-bizhawk-dev.sh /path/to/BizHawk`
- Or manually create the symlink as shown above
- Ensure your BizHawk installation has the `dll/` subdirectory

### Build fails: "dotnet not found"
- Install .NET SDK 6.0 or later from https://dotnet.microsoft.com/download
- Verify with: `dotnet --version`

### Tracker not connecting
- Ensure the tracker GUI/web is running BEFORE opening the BizHawk plugin
- Check that ports 24800 and 24801 are not blocked by firewall
- Try restarting both the tracker and BizHawk

### Wrong game detected
- The tracker auto-detects OoT, MM, or Combo ROMs
- If detection fails, ensure you're using a supported ROM version
- Check BizHawk console for error messages

### Items not tracking
- Verify the ROM is a randomizer ROM (not vanilla)
- Check that you're using BizHawk's N64 core (Mupen64Plus)
- Try saving and reloading the game state

## Layout Options

The tracker supports multiple layouts:
- **OoT Default**: Standard OoT items only
- **MM Default**: Majora's Mask items only
- **Combo**: Both OoT and MM items together

Select your layout in the tracker GUI settings.

## Memory Addresses (Developer Reference)

For developers/debugging, key memory ranges:
- OoT Save: `0x11a5d0` (size: `0x1450`)
- MM Save: `0x1ef670` (size: `0x48d0`)
- Combo Context: `0x801c6fa0` (determines active game)

**Note**: These addresses are for specific ROM versions and may vary.

## Links

- OoTMM Randomizer: https://ootmm.com/
- OoT Randomizer: https://ootrandomizer.com/
- BizHawk: https://github.com/TASEmulators/BizHawk
- Tracker Source: https://github.com/spencerduncan/oottracker
