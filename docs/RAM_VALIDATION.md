# RAM Validation Test Harness

This document describes the RAM validation test harness for automated emulator testing in oottracker.

## Overview

The RAM validation system provides tools for reading and validating RAM contents from emulators (Project64-EM, BizHawk) running OoT/MM. It enables automated verification that emulator state matches expected fixture values.

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Emulator      │────▶│  Lua Harness     │────▶│  Test Harness   │
│  (PJ64-EM)      │     │ (oottracker-e2e) │     │  (Rust)         │
└─────────────────┘     └──────────────────┘     └─────────────────┘
        │                       │                        │
        │ Game RAM              │ TCP Protocol           │ Validation
        ▼                       ▼                        ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  N64 RDRAM      │     │  CMD_READ_RAM    │     │  Fixtures &     │
│  (8 MB)         │     │  (0x21)          │     │  Reports        │
└─────────────────┘     └──────────────────┘     └─────────────────┘
```

## Key Components

### 1. RAM Read Protocol (`CMD_READ_RAM`)

The Lua harness (`assets/oottracker-e2e-harness.lua`) implements a command to read arbitrary RAM regions.

**Protocol:**
- Command: `0x21` (CMD_READ_RAM)
- Request: `[cmd] [len_hi] [len_lo] [addr_0..3] [size_hi] [size_lo]`
- Response: `[0x00] [data...]` on success, `[0x01] [error_code]` on failure

**Example:**
```
Read 256 bytes from 0x11A5D0:
Request:  21 00 06 00 11 A5 D0 01 00
Response: 00 [256 bytes of save context data]
```

### 2. RamReadRequest

Represents a request to read RAM from the emulator.

```rust
use oottracker_e2e::ram_validation::RamReadRequest;

// Read 256 bytes from an address
let request = RamReadRequest::new(0x11A5D0, 256);

// Read with a name for logging
let request = RamReadRequest::named(0x11A5D0, 256, "Save Context");

// Predefined requests
let oot_save = RamReadRequest::oot_save_context();
let mm_save = RamReadRequest::mm_save_context();

// OoT field helper (offset from save context base)
let quest_status = RamReadRequest::oot_field(0xA4, 4, "Quest Status");
```

### 3. ExpectedValue

Specifies expected bytes at a memory location.

```rust
use oottracker_e2e::ram_validation::ExpectedValue;

// Single byte
let emerald_bit = ExpectedValue::byte(0xA4, 0x04, "Kokiri Emerald");

// Big-endian u16
let health = ExpectedValue::u16_be(0x30, 16, "Current Health");

// Big-endian u32
let game_mode = ExpectedValue::u32_be(0x135C, 0, "Game Mode");

// Arbitrary bytes
let zeldaz = ExpectedValue::new(
    0x1C,
    vec![0x5A, 0x45, 0x4C, 0x44, 0x41, 0x5A], // "ZELDAZ"
    "Magic Number",
);

// Non-critical (won't fail overall validation)
let optional = ExpectedValue::optional(0x50, vec![0x00], "Optional Field");
```

### 4. RamValidator

Validator for comparing RAM contents against expected values.

```rust
use oottracker_e2e::{
    ram_validation::{RamValidator, CompareMode},
    fixtures::deku_tree_complete,
};

// Create from fixture
let fixture = deku_tree_complete();
let validator = RamValidator::from_fixture(&fixture);

// Build manually
let validator = RamValidator::oot_save()
    .expect_byte(0xA4, 0x04, "Kokiri Emerald")
    .expect_u16(0x30, 16, "Current Health")
    .with_timeout(Duration::from_secs(10));

// Use bit masking
let bit_validator = RamValidator::oot_save()
    .with_mode(CompareMode::BitsSet)
    .expect_byte(0xA4, 0x04, "Has Emerald Bit");
```

### 5. ValidationReport

Structured results from validation.

```rust
// Validate against emulator
let report = validator.validate(&mut harness).await?;

// Check results
println!("Passed: {}", report.passed());
println!("Pass count: {}/{}", report.pass_count(), report.results.len());
println!("Failures: {}", report.fail_count());
println!("Critical failures: {}", report.critical_fail_count());

// Print summary
println!("{}", report.summary());

// Full report
println!("{}", report);

// Iterate failures
for failure in report.failures() {
    println!("{}", failure);
}
```

## Comparison Modes

| Mode | Description |
|------|-------------|
| `Exact` | Byte-for-byte equality (default) |
| `BitsSet` | Expected bits are set in actual (`(actual & expected) == expected`) |
| `BitsClear` | Expected bits are clear in actual (`(actual & expected) == 0`) |

## Memory Layout Reference

### OoT Save Context (Base: `0x11A5D0`)

| Offset | Size | Description |
|--------|------|-------------|
| 0x1C | 6 | ZELDAZ magic ("ZELDAZ") |
| 0x2E | 2 | Max health (quarter hearts) |
| 0x30 | 2 | Current health |
| 0x34 | 2 | Rupees |
| 0x74 | 24 | Inventory items |
| 0x9C | 4 | Equipment owned |
| 0xA4 | 4 | Quest status (stones/medallions) |
| 0xED4 | 28 | Event check flags |
| 0x135C | 4 | Game mode |

### MM Save Context (Base: `0x1EF670`)

| Offset | Size | Description |
|--------|------|-------------|
| 0x20 | 1 | Player form |
| 0x2C | 2 | Health capacity |
| 0x34 | 2 | Rupees |
| 0x44 | 1 | Sword/Shield |
| 0x48 | 4 | Day |
| 0x70 | 24 | Inventory |
| 0x88 | 24 | Masks |
| 0xA4 | 4 | Quest items |
| 0xA8 | 10 | Dungeon items |
| 0xBC | 10 | Small keys |
| 0xD0 | 5 | Stray fairies |

## Usage Examples

### Basic Validation

```rust
use oottracker_e2e::{
    HarnessBuilder,
    fixtures::deku_tree_complete,
    ram_validation::{RamValidator, RamValidationExt},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create harness and connect
    let mut harness = HarnessBuilder::new()
        .wine_prefix("/home/user/.wine")
        .pj64_exe("/path/to/Project64.exe")
        .rom("/path/to/oot.z64")
        .build();

    harness.start_listener().await?;
    harness.launch().await?;
    harness.wait_for_connection().await?;
    harness.handshake().await?;

    // Validate against fixture
    let fixture = deku_tree_complete();
    let report = harness.validate_fixture(&fixture).await?;

    if report.passed() {
        println!("Validation PASSED!");
    } else {
        println!("Validation FAILED:");
        for failure in report.failures() {
            println!("  {}", failure);
        }
    }

    Ok(())
}
```

### Batch Validation

```rust
use oottracker_e2e::{
    ram_validation::{BatchValidator, RamValidator},
    fixtures::all_fixtures,
};

// Create batch from fixtures
let batch = BatchValidator::from_fixtures(&all_fixtures());

// Run all validators
let reports = batch.validate(&mut harness).await?;

// Summarize results
let summary = BatchValidator::summarize(&reports);
println!("{}", summary);
// Output: "Batch Validation: 10/14 validators passed, 45/60 checks passed"
```

### Custom Validation

```rust
use oottracker_e2e::ram_validation::{RamValidator, ExpectedValue, CompareMode};

// Check specific game state
let validator = RamValidator::oot_save()
    // Must have ZELDAZ magic
    .expect(ExpectedValue::new(
        0x1C,
        vec![0x5A, 0x45, 0x4C, 0x44, 0x41, 0x5A],
        "ZELDAZ Magic",
    ))
    // Must be in gameplay mode
    .expect(ExpectedValue::u32_be(0x135C, 0, "Game Mode"))
    // Check for specific items using bit mask
    .with_mode(CompareMode::BitsSet)
    .expect_byte(0xA4, 0x07, "All Stones");  // Has all 3 spiritual stones

let report = validator.validate(&mut harness).await?;
```

### Offline Validation (Testing)

```rust
use oottracker_e2e::ram_validation::{RamValidator, zeldaz_validator};

// Create test data
let mut fake_ram = vec![0u8; 0x1400];
fake_ram[0x1C..0x22].copy_from_slice(b"ZELDAZ");

// Validate without harness
let validator = zeldaz_validator();
let report = validator.validate_data(&fake_ram);

assert!(report.passed());
```

## Predefined Validators

| Function | Description |
|----------|-------------|
| `zeldaz_validator()` | Checks for valid ZELDAZ magic |
| `game_mode_validator(mode)` | Checks game mode field |
| `inventory_validator(items)` | Checks inventory slots |

## Error Handling

```rust
use oottracker_e2e::harness::HarnessError;

match validator.validate(&mut harness).await {
    Ok(report) => {
        if !report.passed() {
            // Validation ran but some checks failed
            for error in &report.errors {
                eprintln!("Error: {}", error);
            }
        }
    }
    Err(HarnessError::ConnectionTimeout) => {
        eprintln!("Connection timed out");
    }
    Err(HarnessError::Io(e)) => {
        eprintln!("I/O error: {}", e);
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Integration with Fixtures

The `RamValidator::from_fixture()` method automatically extracts expected values from `GameStateFixture`:

| Fixture Field | Validation |
|---------------|------------|
| `equipment` | Equipment bits at offset 0x9C |
| `quest_status` | Quest status bits at offset 0xA4 |
| `health` | Current health at offset 0x30 |
| `max_health` | Max health at offset 0x2E |
| `rupees` | Rupee count at offset 0x34 |

Additional fields (inventory, boss defeats) can be added by extending the fixture.

## Port Configuration

| Port | Purpose |
|------|---------|
| 24801 | Tracker data (RAM updates) |
| 24802 | E2E test control (CMD_READ_RAM, etc.) |

## Development

### Running Tests

```bash
# Run RAM validation unit tests
cargo test -p oottracker-e2e ram_validation

# Run all E2E tests
cargo test -p oottracker-e2e
```

### Adding New Validators

1. Define expected values:
```rust
pub fn my_custom_validator() -> RamValidator {
    RamValidator::oot_save()
        .expect_byte(OFFSET, VALUE, "Field Name")
}
```

2. Add to predefined validators in `ram_validation.rs`

3. Add tests to verify behavior

## References

- [Lua Harness](../assets/oottracker-e2e-harness.lua) - E2E test Lua script
- [Test Harness](../crate/oottracker-e2e/src/harness.rs) - Rust test harness
- [Fixtures](../crate/oottracker-e2e/src/fixtures.rs) - Game state fixtures
- [OoT RAM Layout](https://wiki.cloudmodding.com/oot/Save_Format) - CloudModding Wiki
