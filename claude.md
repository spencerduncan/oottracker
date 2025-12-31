# OOTTracker - Guide for AI Assistants

This document provides context for AI assistants (like Claude) working on the OOTTracker codebase.

## Project Overview

**OOTTracker** is an item tracker for the Ocarina of Time Randomizer (OoTR). It helps speedrunners and players track randomized item locations, dungeon information, and progression during OoT randomizer playthroughs.

**Note:** This repository is a **fork** of [Fenhl's original OOTTracker](https://github.com/fenhl/oottracker). The original project is live at https://oottracker.fenhl.net/.

### Key Features
- **Auto-tracking:** Real-time memory reading from BizHawk, Project64, and RetroArch emulators
- **Cross-platform:** Desktop GUI (Windows/macOS), web interface, and emulator plugins
- **Networked tracking:** WebSocket-based synchronization for multiworld and tournament play
- **Knowledge inference:** Extracts hints from text boxes, dungeon screens, and game state

### Deployment Notes for This Fork
- **Web deployment:** Requires your own server/hosting (does not use oottracker.fenhl.net)
- **Local development:** Can run web server locally on `localhost`
- **Production setup:** Requires nginx configuration, PostgreSQL database, systemd service setup
- Refer to `assets/oottracker-web.service` and `assets/*.nginx` for deployment configuration

## Architecture

### Monorepo Structure (14 Crates)

```
oottracker/
├── crate/
│   ├── ootr/                    # Core randomizer trait definitions
│   ├── ootr-dynamic/            # Runtime Python integration (PyO3)
│   ├── ootr-static/             # Compile-time randomizer data
│   ├── ootr-static-derive/      # Proc-macros for static data
│   ├── oottracker/              # Main library (largest: 8,636 LOC)
│   ├── oottracker-bizhawk/      # BizHawk plugin (C# + Rust FFI)
│   ├── oottracker-csharp/       # C# FFI bindings
│   ├── oottracker-derive/       # Proc-macros for oottracker
│   ├── oottracker-gui/          # Desktop GUI (Iced framework)
│   ├── oottracker-updater/      # Auto-updater
│   ├── oottracker-updater-bizhawk/  # BizHawk updater
│   ├── oottracker-utils/        # CLI utilities & build scripts
│   └── oottracker-web/          # Web server (Rocket + Warp)
├── assets/
│   ├── docs/                    # Project documentation
│   ├── web/                     # Static web assets
│   ├── img/                     # Game item/icon images
│   └── *.ps1                    # PowerShell build scripts
└── README.md
```

### Core Data Structures

#### ModelState (lib.rs:55-59)
The central state object passed throughout the codebase:
```rust
pub struct ModelState {
    pub knowledge: Knowledge,    // Learned information about the seed
    pub tracker_ctx: TrackerCtx, // UI configuration
    pub ram: Ram,                // Current emulator memory state
}
```

#### Key Modules

| Module | LOC | Purpose |
|--------|-----|---------|
| **ui.rs** | 2,441 | UI rendering, cell management, config I/O |
| **save.rs** | 1,543 | OoT save file parsing, binary protocol |
| **firebase.rs** | 775 | External tracker synchronization |
| **checks.rs** | 757 | Location validation (checks if locations are collected) |
| **knowledge.rs** | 537 | Game state inference, settings tracking |
| **scene.rs** | 510 | Scene flags, gold skulltula tracking |
| **ram.rs** | 409 | RAM dump parsing for emulator memory |
| **net.rs** | 354 | Network connections (TCP, WebSocket, UDP) |

### Technology Stack

- **Language:** Rust (Edition 2021)
- **Async Runtime:** Tokio 1.x
- **GUI Framework:** Iced 0.4 (desktop)
- **Web Framework:** Rocket 0.5.0-rc.2 (⚠️ release candidate) + Warp 0.3
- **Networking:** tokio-tungstenite 0.20, async-proto 0.16
- **Database:** PostgreSQL via sqlx 0.6
- **Python Integration:** PyO3 0.19 (for runtime randomizer access)

## Code Quality Standards

### Enforced via Lints (lib.rs:1-2)
```rust
#![deny(rust_2018_idioms, unused, unused_crate_dependencies,
        unused_import_braces, unused_qualifications, warnings)]
#![forbid(unsafe_code)]
```

- ✅ **No unsafe code** allowed anywhere in the project
- ✅ All warnings treated as errors
- ✅ Unused code/imports/dependencies are denied
- ✅ Rust 2018 idioms enforced

### Testing Status
⚠️ **Current test coverage: 0%**
- No unit tests exist in main crates
- No integration test suite
- No CI/CD automation (manual PowerShell scripts)
- See `assets/docs/TESTING_ANALYSIS.md` for detailed analysis

## Known Issues & Technical Debt

### Critical Issues

1. **Panic Macros in Production Code (checks.rs)**
   - Lines: 136, 400, 407, 414, 416
   - Panic on unknown events/locations instead of returning errors
   - Impact: Tracker crashes on new randomizer updates

2. **Rocket Version (0.5.0-rc.2)**
   - Using release candidate in production
   - Should upgrade to stable version
   - Location: `crate/oottracker/Cargo.toml:63-64`

3. **600+ TODO Comments**
   - Significant incomplete functionality
   - Examples: setting checks, trial checks, trick checks not implemented
   - Many knowledge extraction features partially implemented

4. **Hardcoded Dependencies**
   - URLs: `wss://oottracker.fenhl.net/websocket` (net.rs:130)
   - GitHub API URLs (github.rs:51-72)
   - Filesystem paths (ui.rs:106-116)
   - Makes testing difficult without mocking framework

5. **Git Dependencies**
   - 7 dependencies from personal GitHub repos on `main` branch
   - Risk: version drift, supply chain integrity
   - Currently pinned via Cargo.lock

### Architectural Concerns

1. **God Objects**
   - `ModelState` tightly couples all subsystems
   - `checks.rs::checked()` is 732-line function with 400+ match arms
   - Hard to test or maintain independently

2. **Missing Abstractions**
   - No `FileSystem` trait for mocking I/O
   - No `HttpClient` trait for mocking network calls
   - Direct dependencies on `tokio::fs`, `reqwest`

3. **Error Handling**
   - Many `Option<bool>` return types should be `Result<bool, Error>`
   - `unimplemented!()` macros in firebase.rs (6+ locations)
   - Silent failures with `//TODO report/log error?` comments

## Development Guidelines

### When Making Changes

1. **Read Before Editing**
   - Always read files before modifying them
   - Understand existing patterns and conventions
   - Check for related TODOs and comments

2. **Maintain Code Quality**
   - No unsafe code (enforced by compiler)
   - Add appropriate error handling (prefer `Result` over `panic!`)
   - Follow existing naming conventions
   - Keep functions under 200 lines when possible

3. **Platform Considerations**
   - Windows is the primary platform (most users)
   - macOS support is secondary but important
   - Web server runs on Linux (production)
   - BizHawk plugin is Windows-only (intentional)

4. **Binary Compatibility**
   - RAM addresses are hardcoded for N64 memory layout
   - Save file format must match retail OoT structure
   - Protocol changes require version bumps

### Testing Approach

Since no test infrastructure exists:
1. Manual testing is currently the only option
2. Test on target platforms (Windows for GUI, Linux for web)
3. Verify with real emulators when changing auto-tracking
4. Check web interface at localhost before deploying

For adding tests, see `assets/docs/TESTING_ANALYSIS.md`.

### Building the Project

```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build --package oottracker-gui

# Build for release (with LTO and stripping)
cargo build --release

# Run web server locally
cargo run --package oottracker-web

# Run GUI application
cargo run --package oottracker-gui
```

### Release Process

Currently manual via PowerShell scripts:
- `assets/release.ps1` - Main release pipeline
- `assets/test-all.ps1` - Test all crates
- `assets/test-gui.ps1` - GUI-specific tests
- `assets/test-bizhawk.ps1` - BizHawk plugin tests

Version bumping utility: `oottracker-utils::version-bump`

## Common Tasks

### Adding a New Item Type

1. Add item ID to `crate/oottracker/src/item_ids.rs`
2. Update save file structure in `crate/oottracker/src/save.rs`
3. Add UI cell rendering in `crate/oottracker/src/ui.rs`
4. Update click handlers in `ui.rs::left_click()` and `ui.rs::right_click()`
5. Add RAM parsing if needed in `crate/oottracker/src/ram.rs`
6. Update web UI templates in `assets/web/`

### Adding a New Check Location

1. Add location string to `crate/oottracker/src/checks.rs::checked()`
2. Map to appropriate scene flags or save file flags
3. Test with emulator auto-tracking
4. Update knowledge inference if location provides hints

### Modifying Network Protocol

1. Update protocol definitions in `crate/oottracker/src/proto.rs`
2. Increment version number if breaking change
3. Update both client and server handlers
4. Test with web interface and networked tracking

### Working with Firebase Integration

⚠️ **Note:** Firebase module has multiple `unimplemented!()` blocks
- Feature-gated: requires `firebase` feature flag
- Location: `crate/oottracker/src/firebase.rs`
- Currently hardcoded to `ootr_static::Rando`
- Many unimplemented checks (lines 195, 240, 281, etc.)

## Useful File References

### Configuration Files
- Workspace manifest: `Cargo.toml` (root)
- Platform-specific build: `.cargo/config.toml`
- Systemd service: `assets/oottracker-web.service`
- Nginx config: `assets/oottracker.nginx`

### Documentation
- Crate descriptions: `assets/docs/crates.md`
- Restream guide: `assets/docs/restream.md`
- Testing analysis: `assets/docs/TESTING_ANALYSIS.md`
- MM integration: `assets/docs/MM_INTEGRATION.md`

### Important Source Files
- Main library entry: `crate/oottracker/src/lib.rs`
- Check validation: `crate/oottracker/src/checks.rs`
- UI rendering: `crate/oottracker/src/ui.rs`
- Web server: `crate/oottracker-web/src/main.rs`
- GUI app: `crate/oottracker-gui/src/main.rs`

## Getting Help

- **Community:** #setup-support on OoT Randomizer Discord
- **Issues:** https://github.com/fenhl/oottracker/issues
- **Wiki:** https://github.com/fenhl/oottracker/wiki/instructions
- **Maintainer:** Fenhl (@Fenhl#4813 on Discord)

## Racing Rules Warning

⚠️ **Important:** As of 2021-11-15, racing rules prohibit all usage of trackers capable of auto-tracking even in manual mode. How this applies to OOTTracker is unclear pending RaceMod decision. See README.md:1-2.

## Project Status

- **Version:** 0.7.4
- **Status:** Active maintenance
- **Activity:** Regular commits (dependency updates, new features)
- **Maturity:** Production-ready, actively used by community
- **Recent Focus:** Item capacity tracking, ocarina differentiation, multiworld improvements

## Future Considerations

See `assets/docs/MM_INTEGRATION.md` for discussion of supporting Majora's Mask in a combined OoT/MM randomizer.

Key challenges:
- Different game memory layout
- Different item set and progression
- Different dungeon/region structure
- UI complexity with dual-game tracking

---

**Last Updated:** 2025-12-31
**Rust Toolchain:** 1.91.1
**For AI Assistant Context Only**
