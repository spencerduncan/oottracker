# OOTTracker

[![CI](https://github.com/spencerduncan/oottracker/actions/workflows/ci.yml/badge.svg)](https://github.com/spencerduncan/oottracker/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/spencerduncan/oottracker/branch/main/graph/badge.svg)](https://codecov.io/gh/spencerduncan/oottracker)

> **Note:** This is a fork of [Fenhl's OOTTracker](https://github.com/fenhl/oottracker). The original project and web app are available at <https://oottracker.fenhl.net/>.

An item tracker for [the *Ocarina of Time* Randomizer](https://ootrandomizer.com/).

## Features

- **Cross-platform:** Runs on Windows and macOS, with a web interface available
- **Auto-tracking:** Real-time memory reading from [BizHawk](https://tasvideos.org/BizHawk), [Project64](https://www.pj64-emu.com/), or [RetroArch](https://retroarch.com/)
- **Networked tracking:** Connect to external trackers including <https://oot-tracker.web.app/>, <https://ootr-tracker.web.app/>, and <https://ootr-random-settings-tracker.web.app/>
- **Knowledge inference:** Extracts hints from text boxes, dungeon screens, and game state
- **Randomizer tracking:** OoTMM logic evaluation with settings support and location check detection

## Documentation

**Getting Started:**
- [**Quickstart Guide (PJ64-EM)**](docs/QUICKSTART_PJ64EM.md) - Get tracking in 5 minutes!

**Setup Guides:**
- [BizHawk Setup](docs/BIZHAWK_SETUP.md) - Windows BizHawk integration
- [Project64-EM Setup](docs/PROJECT64_EM_SETUP.md) - PJ64-EM on Windows/Linux (detailed)

**Technical Documentation:**
- [Randomizer Tracking System](docs/RANDOMIZER_TRACKING.md) - Architecture, flag mapping, and development
- [Error Handling Policy](docs/error-handling.md) - Error handling patterns and guidelines

## Racing Rules Notice

Please check [the current racing rules](https://wiki.ootrandomizer.com/index.php?title=Rules#Universal_Rules) before using auto-tracking features in races. Rules regarding trackers with auto-tracking capability may apply even when using manual mode.

## Download

For pre-built releases, see the [upstream releases](https://github.com/fenhl/oottracker/releases):

- [BizHawk (Windows, 64-bit)](https://github.com/fenhl/oottracker/releases/latest/download/oottracker-bizhawk-win64.zip)
- Project64-EM: Use the Lua script from `assets/oottracker-pj64em-base.lua` (see [setup guide](docs/PROJECT64_EM_SETUP.md))
- [Windows (64-bit)](https://github.com/fenhl/oottracker/releases/latest/download/oottracker-win64.exe) (includes auto-tracker for RetroArch)
- [macOS (Universal)](https://github.com/fenhl/oottracker/releases/latest/download/oottracker-mac.dmg) (includes auto-tracker for RetroArch)

For detailed install/usage instructions, see the [upstream wiki](https://github.com/fenhl/oottracker/wiki/instructions).

## Building from Source

```bash
# Build all crates
cargo build --workspace

# Build for release
cargo build --release

# Run the GUI application
cargo run --package oottracker-gui

# Run the web server locally
cargo run --package oottracker-web
```

## Support

If you run into problems:

- Ask in #setup-support ([invite link](https://discord.gg/BGRrKKn) • [direct channel link](https://discord.com/channels/274180765816848384/476723801032491008)) on the OoT Randomizer Discord
- [Open an issue](https://github.com/spencerduncan/oottracker/issues) on this fork
- For upstream issues, see [Fenhl's issue tracker](https://github.com/fenhl/oottracker/issues)

## Credits

- Original project by [Fenhl](https://github.com/fenhl)
- Big Poe image by [Maplestar](https://github.com/Maplesstar)
- Item images by [Xopar](https://github.com/matthewkirby), used with permission
- Game data from [the CloudModding wiki](https://wiki.cloudmodding.com/oot) and [RiptideSage's completed checks script](https://github.com/RiptideSage/OoT-CompletedChecks)
- Research tools: [BizHawk](https://tasvideos.org/BizHawk) and [the practice rom](https://www.practicerom.com/)
