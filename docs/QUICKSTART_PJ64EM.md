# OoTMM Tracker Quickstart Guide (Project64-EM)

Get your OoTMM randomizer tracker running in 5 minutes!

## What You'll Need

- **Project64-EM** - The OoTMM-optimized emulator
- **An OoTMM seed** - Generated at [ootmm.com](https://ootmm.com)
- **Your base ROMs** - Legally owned OoT and MM ROMs

## Quick Setup

### Step 1: Download Project64-EM

1. Go to [Project64-EM Releases](https://github.com/OoTMM/Project64-EM/releases)
2. Download the latest `Project64-EM-x.x.x.zip`
3. Extract to a folder (e.g., `C:\Project64-EM` on Windows)

### Step 2: Get the Tracker Script

1. Download `oottracker-pj64em-base.lua` from this repository's `assets/` folder
2. Copy it to Project64-EM's `Scripts` folder:
   - Windows: `C:\Project64-EM\Scripts\`
   - Linux: `/path/to/Project64-EM/Scripts/`

   > **Note:** Create the `Scripts` folder if it doesn't exist

### Step 3: Start the Web Tracker

**Option A: Use the hosted tracker** (easiest)
- Go to [oottracker.fenhl.net](https://oottracker.fenhl.net)
- Enter a room name and click "Go"

**Option B: Run locally**
```bash
# If you have Rust installed:
cargo run --release -p oottracker-web

# Then open: http://localhost:24800
```

### Step 4: Connect Everything

1. **Start Project64-EM** and load your OoTMM ROM
2. **Enable the tracker script:**
   - Go to `System` → `Scripts`
   - Check the box next to `oottracker-pj64em-base.lua`
   - Click **Run**
3. **Watch the magic!** Items appear on the tracker as you collect them

## Configuring Randomizer Settings

The tracker needs to know your seed settings to evaluate logic correctly.

### Access Settings Page

1. Go to `/settings` on the web tracker (e.g., `http://localhost:24800/settings`)
2. Or click "Configure randomizer settings" on the home page

### Load Your Seed Settings

**Easiest method:** Import from OoTMM
1. When generating your seed at ootmm.com, download the settings JSON
2. On the tracker settings page, click **"Load from File"**
3. Select your settings JSON file

**Manual method:** Configure each setting individually using the dropdowns and checkboxes.

### Key Settings to Configure

| Setting | What It Does |
|---------|--------------|
| **Logic Mode** | `Glitchless`, `Glitched`, or `No Logic` |
| **Open Dungeons (OoT)** | Which dungeons are pre-opened (DC, BotW, JJ, etc.) |
| **Open Dungeons (MM)** | Stone Tower, Woodfall open status |
| **MQ Dungeons** | Which OoT dungeons use Master Quest layout |
| **Door of Time** | Open or closed by default |
| **Ganon's Boss Key** | Vanilla, removed, or custom |

### Save Your Settings

- Click **"Save to File"** to download your configuration
- Click **"Copy as JSON"** to paste elsewhere
- Settings persist in your browser's local storage

## Understanding the Tracker Display

### Item Tracking

The tracker shows your collected items in real-time:
- **Grayed out** = Not yet obtained
- **Colored/Lit** = In your inventory
- **Numbers** = Quantity (rupees, keys, etc.)

### Checked Locations (Coming Soon)

The tracker will show which randomizer locations you've checked:
- ✅ **Checked** = You've collected this location's item
- ⬜ **Unchecked** = Location not yet visited
- ❓ **Unknown** = Status cannot be determined

### Location Logic

With proper settings configured, the tracker can evaluate:
- Which locations are **accessible** with your current items
- Which locations are **blocked** by missing items or events
- Trick-enabled locations (if you've enabled tricks)

## Troubleshooting

### Tracker Not Connecting

**Symptoms:** Items don't update when collected

**Solutions:**
1. Make sure the web tracker is running BEFORE enabling the script
2. Check that the script shows "Connected" in PJ64-EM's script console
3. Verify port 24801 isn't blocked by firewall

### Script Not Appearing

**Symptoms:** No `oottracker-pj64em-base.lua` in System → Scripts

**Solutions:**
1. Verify the `.lua` file is in the correct `Scripts` folder
2. Restart Project64-EM after adding the script
3. Check the file wasn't renamed (must end in `.lua`)

### Wrong Items Showing

**Symptoms:** Tracker shows items you don't have

**Solutions:**
1. Make sure you're using an OoTMM ROM (not vanilla OoT/MM)
2. Save and reload your game
3. Restart both PJ64-EM and the tracker

### Linux: "Failed to reserve RDRAM"

**Cause:** Using a 32-bit Wine prefix

**Fix:** Create a 64-bit Wine prefix:
```bash
WINEPREFIX=~/.wine-pj64-64 wine wineboot
```

Then launch PJ64-EM with:
```bash
WINEPREFIX=~/.wine-pj64-64 wine /path/to/Project64-EM.exe
```

### Linux: Input Lag

**Cause:** Wayland display server

**Fix:** Use X11 session or launch with gamescope:
```bash
gamescope -f -- wine /path/to/Project64-EM.exe
```

## Tips for Racing

⚠️ **Racing Rules:** Check [the current OoTR racing rules](https://wiki.ootrandomizer.com/index.php?title=Rules#Universal_Rules) before using auto-tracking in races. Some races prohibit auto-trackers.

### For Allowed Races

1. **Pre-configure settings** before the race starts
2. **Test the connection** with a practice seed
3. **Have a backup plan** (manual tracking) in case of issues

### Manual Mode

If auto-tracking isn't allowed, you can still use the tracker manually:
- Click items to toggle them on/off
- Use keyboard shortcuts for quick updates
- The tracker works without the PJ64-EM script connection

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `R` | Reset tracker |
| `1-9` | Quick item toggles |
| `Click` | Toggle item state |
| `Right-click` | Cycle item variants |

## Getting Help

- **Discord:** [OoT Randomizer Discord](https://discord.gg/BGRrKKn) → #setup-support
- **Issues:** [GitHub Issues](https://github.com/spencerduncan/oottracker/issues)
- **OoTMM Help:** [OoTMM Discord](https://discord.gg/ootmm)

## Next Steps

- **Configure logic tricks** for advanced routing
- **Set up entrance randomizer** tracking (if using ER)
- **Customize the layout** for your preferences
- **Read the full documentation:** [RANDOMIZER_TRACKING.md](RANDOMIZER_TRACKING.md)

---

Happy tracking! 🎮
