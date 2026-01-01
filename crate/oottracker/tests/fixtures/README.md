# Test Fixtures

This directory contains test fixture files for the oottracker crate.

## Directory Structure

```
fixtures/
├── README.md           # This file
└── knowledge/          # Knowledge-related fixtures
    ├── default.json    # Default (empty) knowledge state
    └── vanilla.json    # Vanilla game knowledge state
```

## Usage

Use the helper functions from `tests/common/mod.rs` to load fixtures in your tests:

```rust
mod common;

use common::{fixture_path, load_fixture, load_fixture_bytes};

#[test]
fn test_with_fixture() {
    // Load fixture as string (for JSON, TOML, etc.)
    let content = load_fixture("knowledge/default.json");
    let knowledge: Knowledge = serde_json::from_str(&content).unwrap();

    // Get fixture path for custom handling
    let path = fixture_path("knowledge/vanilla.json");

    // Load binary fixtures
    let bytes = load_fixture_bytes("save/example.bin");
}
```

## Adding New Fixtures

1. Create an appropriate subdirectory if needed (e.g., `save/`, `config/`)
2. Add your fixture file with a descriptive name
3. Update this README if adding a new category of fixtures

## Fixture Categories

### knowledge/

JSON files representing `Knowledge` states for testing knowledge serialization and game state tracking.

- `default.json` - Empty/default knowledge state
- `vanilla.json` - Knowledge state for vanilla (non-randomized) game

## Notes

- JSON fixtures should be valid according to the `KnowledgeJson` schema
- Binary fixtures (e.g., save data) should be in the appropriate format for the data type
- Keep fixture files small and focused on specific test scenarios
