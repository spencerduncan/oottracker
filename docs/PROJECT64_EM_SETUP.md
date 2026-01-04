# Project64-EM Setup Guide for Linux (via Wine)

This guide explains how to set up Project64-EM on Linux using Wine to use with the oottracker.

## Prerequisites

1. **Wine** (latest stable recommended)
   - Install via your package manager (e.g., `sudo apt install wine` or `sudo pacman -S wine`)
   - **Important**: A 64-bit Wine prefix is required (see Wine Setup below)

2. **Project64-EM** (OoTMM's modified emulator)
   - Download from: https://github.com/OoTMM/Project64-EM/releases
   - This is a special build optimized for OoTMM randomizer

3. **OoTMM ROM**
   - Generate from: https://ootmm.com/
   - Upload your legally owned OoT and MM ROMs
   - Download the generated `.z64` file

## Step 1: Wine Setup

### Creating a 64-bit Wine Prefix (Required)

**This step is critical.** Using a 32-bit Wine prefix will fail with a "Failed to reserve RDRAM" error. You must create a dedicated 64-bit prefix:

```bash
WINEPREFIX=/home/user/.wine-pj64-64 wine wineboot
```

Wait for Wine to finish initializing the prefix. This may take a minute and might show some dialog boxes.

### Creating a Launch Script

Create a launch script for convenience and optimal performance:

```bash
#!/bin/bash
export WINEPREFIX="/home/user/.wine-pj64-64"
export vblank_mode=0
export __GL_SYNC_TO_VBLANK=0
export WINEDEBUG=-all
wine /path/to/Project64-EM.exe "$@"
```

Save this as `pj64em.sh` and make it executable:

```bash
chmod +x pj64em.sh
```

**Environment variables explained:**
- `WINEPREFIX`: Points to your 64-bit Wine prefix
- `vblank_mode=0`: Disables vsync for Mesa drivers (AMD/Intel)
- `__GL_SYNC_TO_VBLANK=0`: Disables vsync for NVIDIA drivers
- `WINEDEBUG=-all`: Suppresses Wine debug output for cleaner console

## Step 2: Display Server Considerations

### Wayland vs X11

**Wayland** may introduce noticeable input lag when running Wine applications. For the best gaming experience:

**Option 1: Use an X11 Session**
- Log out and select "GNOME on Xorg" or equivalent X11 session at login

**Option 2: Use Gamescope**
Gamescope provides an X11-compatible environment on Wayland:

```bash
gamescope -f -- ./pj64em.sh
```

Install gamescope via your package manager if needed (e.g., `sudo apt install gamescope`).

## Step 3: Install the Tracker Script

1. **Locate the tracker script** in the oottracker repository:
   ```
   assets/oottracker-pj64.js
   ```

2. **Copy to Project64-EM's Scripts folder**:
   ```bash
   # Find where you extracted Project64-EM
   cp assets/oottracker-pj64.js /path/to/Project64-EM/Scripts/
   ```

   If the `Scripts` folder doesn't exist, create it:
   ```bash
   mkdir -p /path/to/Project64-EM/Scripts
   ```

## Step 4: Run the Tracker GUI

1. Build and run the tracker GUI:
   ```bash
   cargo run --release -p oottracker-gui
   ```

2. Or use the web version:
   ```bash
   cargo run --release -p oottracker-web
   ```
   Then open http://localhost:24800 in your browser

## Step 5: Connect Project64-EM to the Tracker

1. **Start Project64-EM** using your launch script and load your OoTMM ROM

2. **Enable the Script**:
   - System → Scripts
   - Check the box next to `oottracker-pj64.js`
   - Click "Run"

3. **The tracker should auto-connect** to the GUI/web interface on port 24801

4. **Verify connection**:
   - The tracker GUI should show items updating as you collect them in-game
   - Check the Project64-EM console for connection status messages

## Troubleshooting

### "Failed to reserve RDRAM" or "Plugins not initialized"
- **Cause**: Using a 32-bit Wine prefix
- **Fix**: Create a 64-bit Wine prefix as shown in Step 1
- Verify with: `WINEPREFIX=/home/user/.wine-pj64-64 wine --version` (should show `wine-X.X (Debian X.X)` or similar, not `wine-X.X (WoW64)`)

### Noticeable input lag
- **Cause**: Wayland display server
- **Fix**: Switch to an X11 session or use gamescope:
  ```bash
  gamescope -f -- ./pj64em.sh
  ```

### Tracker not connecting
- Ensure the tracker GUI/web is running BEFORE enabling the script
- Check that port 24801 is not blocked by firewall
- Verify with: `ss -tlnp | grep 24801`
- Look for error messages in Project64-EM's script console

### Script not appearing in Project64-EM
- Ensure the `.js` file is in the Scripts folder within your Project64-EM directory
- Restart Project64-EM after adding the script

### Wine crashes or graphical glitches
- Try different Wine versions (wine-staging often works better for games)
- Install required Wine dependencies: `winetricks d3dx9 vcrun2019`
- Check Wine logs by removing `WINEDEBUG=-all` from your launch script

### Items not tracking correctly
- Verify you're using an OoTMM ROM (not vanilla OoT or MM)
- Try saving and reloading your game state
- Restart both Project64-EM and the tracker

## Script Configuration (Optional)

The tracker script supports configuration at the top of the file:

```javascript
// Connection settings
const TRACKER_HOST = "127.0.0.1";
const TRACKER_PORT = 24801;

// Memory polling interval (ms)
const POLL_INTERVAL = 100;
```

Edit these if running the tracker on a different machine or port.

## Layout Options

The tracker supports multiple layouts:
- **OoT Default**: Standard OoT items only
- **MM Default**: Majora's Mask items only
- **Combo**: Both OoT and MM items together

Select your layout in the tracker GUI settings.

## Links

- Project64-EM: https://github.com/OoTMM/Project64-EM
- OoTMM Randomizer: https://ootmm.com/
- Wine: https://www.winehq.org/
- Gamescope: https://github.com/ValveSoftware/gamescope
- Tracker Source: https://github.com/spencerduncan/oottracker
