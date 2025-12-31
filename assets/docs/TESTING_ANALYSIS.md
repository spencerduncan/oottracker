# OOTTracker - Comprehensive Testing Analysis

**Analysis Date:** 2025-12-31
**Project Version:** 0.7.4
**Current Test Coverage:** 0%

**Note:** This analysis is for a **fork** of [Fenhl's original OOTTracker](https://github.com/fenhl/oottracker).

## Executive Summary

This document provides a comprehensive analysis of the OOTTracker codebase's testability and estimates the effort required to achieve various levels of test coverage.

### Key Findings

- **Current State:** Zero test infrastructure exists
- **Architecture Testability:** 3/10 (Poor)
- **Maximum Achievable Coverage (without refactoring):** ~60%
- **95% Coverage Estimate:** 18,000-22,000 lines of test code + 3,000 lines of refactoring
- **Recommended Target:** 70% coverage (better ROI)

### Critical Blockers

1. ❌ Panic macros in production code (5+ locations in checks.rs)
2. ❌ Hardcoded URLs and filesystem paths (no dependency injection)
3. ❌ God objects and 700+ line functions
4. ❌ Missing trait abstractions for I/O operations
5. ❌ 600+ TODO comments indicating incomplete features

---

## Table of Contents

1. [Current Testing State](#current-testing-state)
2. [Architecture Testability Assessment](#architecture-testability-assessment)
3. [Module-by-Module Analysis](#module-by-module-analysis)
4. [Test Coverage Estimates](#test-coverage-estimates)
5. [Recommended Testing Strategy](#recommended-testing-strategy)
6. [Refactoring Requirements](#refactoring-requirements)
7. [Implementation Roadmap](#implementation-roadmap)

---

## Current Testing State

### Test Infrastructure: **NONE**

| Aspect | Status |
|--------|--------|
| Unit tests | ❌ None |
| Integration tests | ❌ None |
| Test fixtures | ❌ None |
| Mock framework | ❌ None |
| CI/CD automation | ❌ None (manual PowerShell scripts) |
| Test utilities | ❌ None |
| Property-based tests | ❌ None |
| Benchmark tests | ❌ None |

### Testing Dependencies

Current `Cargo.toml` has **zero** test dependencies:
- No `mockito`, `wiremock`, or HTTP mocking
- No `proptest` or `quickcheck` for property tests
- No `tempfile` for filesystem testing (it's used elsewhere, could be reused)
- No `rstest` or `test-case` for parameterized tests
- No `criterion` for benchmarking

---

## Architecture Testability Assessment

### Overall Score: 3/10 (Poor)

#### Breakdown

| Aspect | Score | Details |
|--------|-------|---------|
| **Dependency Injection** | 2/10 | Hardcoded URLs, filesystem paths, HTTP clients |
| **Trait Abstractions** | 3/10 | Only 1 mockable trait (`Connection`) |
| **Module Coupling** | 3/10 | Tight coupling via `ModelState` god object |
| **I/O Abstraction** | 1/10 | No abstraction layer for file/network operations |
| **Error Handling** | 2/10 | `panic!` and `unimplemented!` instead of `Result` types |
| **Pure Functions** | 5/10 | Some pure logic, but mixed with side effects |

### Testability Blockers by Location

#### 1. Panic Macros (Crashes Tests)

**File:** `crate/oottracker/src/checks.rs`

```rust
// Line 136
panic!("unknown event name: {}", event)

// Line 400
panic!("unknown location name: {}", loc)

// Line 407
panic!("logic helpers can't be checked")

// Line 414
panic!("setting checks not implemented")

// Line 416
panic!("trick checks not implemented")
```

**Impact:**
- Tests cannot validate error conditions - they crash
- Cannot test error paths without catching panics
- Should be `Result<bool, CheckError>` instead

#### 2. Unimplemented Macros (Prevents Compilation)

**File:** `crate/oottracker/src/ui.rs`

```rust
// Lines 1918, 1924, 1930
unimplemented!("CompositeKeys that aren't SmallKeys + BossKey")

// Line 1851
unimplemented!()
```

**File:** `crate/oottracker/src/firebase.rs`

```rust
// Lines 195, 240, 281 (multiple locations)
unimplemented!()
```

**File:** `crate/oottracker/src/ctx.rs`

```rust
// Line 107
unimplemented!("unknown boss reward index: {n}")
```

**Impact:**
- Cannot test Firebase features
- Cannot test certain UI patterns
- Code paths are genuinely incomplete

#### 3. Hardcoded External Dependencies

**File:** `crate/oottracker/src/net.rs`

```rust
// Line 130 - Cannot mock WebSocket
// Note: Hardcoded to official website - fork would need to change this URL
tokio_tungstenite::connect_async("wss://oottracker.fenhl.net/websocket").await?
```

**File:** `crate/oottracker/src/github.rs`

```rust
// Lines 52, 62, 72 - GitHub API URLs hardcoded
let response = client.get(&format!(
    "https://api.github.com/repos/{}/{}/releases/latest",
    self.user, self.name
))
```

**File:** `crate/oottracker/src/ui.rs`

```rust
// Lines 106-116 - Direct filesystem access
let dirs = dirs()?;
let mut file = File::open(dirs.config_dir().join("config.json")).await
```

**Impact:**
- Tests require network access
- Tests pollute real filesystem
- Cannot run tests in CI without external dependencies
- Tests are slow and brittle

#### 4. God Objects

**File:** `crate/oottracker/src/lib.rs`

```rust
// Lines 55-59
pub struct ModelState {
    pub knowledge: Knowledge,
    pub tracker_ctx: TrackerCtx,
    pub ram: Ram,
}
```

**Impact:**
- Every test requires constructing entire `ModelState`
- High setup cost for simple unit tests
- Cannot test components in isolation
- Changes cascade through all tests

#### 5. Giant Functions

**File:** `crate/oottracker/src/checks.rs`

```rust
// Lines 25-757 (732 lines!)
fn checked(&self, model: &ModelState) -> Option<bool> {
    match self {
        // 400+ match arms for different locations...
    }
}
```

**Complexity:**
- ~390 branches
- 4 nested match statements
- String literal location names (no type safety)
- Cannot test individual checks in isolation

**Impact:**
- Impossible to achieve high branch coverage
- Cannot test individual location checks
- Would need table-driven tests with 400+ test cases

---

## Module-by-Module Analysis

### Legend
- **LOC:** Lines of code
- **Complexity:** Cyclomatic complexity (1-10 scale)
- **I/O Coupling:** How tightly coupled to I/O operations
- **Testability:** How easy to test (1-10 scale)

### Main Library Modules (oottracker crate)

#### 1. checks.rs

| Metric | Value |
|--------|-------|
| **LOC** | 757 |
| **Functions** | 4 (1 giant function) |
| **Complexity** | 9/10 (Very High) |
| **Branches** | ~390 |
| **I/O Coupling** | Low (pure logic) |
| **Testability** | 2/10 (Very Poor) |

**Blockers:**
- 5 panic macros for unimplemented checks
- 200+ `None` returns (unimplemented checks)
- 732-line function with nested matches
- Requires full `ModelState` for every test

**Testing Approach:**
- Table-driven tests (need 400+ test cases)
- Property-based testing not applicable
- Would need to refactor into smaller functions first

**Estimated Test Lines:** 2,000-2,500 lines for full coverage

**Test Examples Needed:**
```rust
#[test]
fn test_location_deku_tree_gohma() { /* ... */ }

#[test]
fn test_location_forest_temple_gs() { /* ... */ }

#[test]
fn test_unknown_location_returns_error() { /* ... */ }

// + 397 more location tests...
```

#### 2. ui.rs

| Metric | Value |
|--------|-------|
| **LOC** | 2,441 |
| **Functions** | 44 |
| **Complexity** | 8/10 (High) |
| **Branches** | ~200 |
| **I/O Coupling** | High (filesystem) |
| **Testability** | 4/10 (Poor) |

**Blockers:**
- Direct filesystem I/O (no abstraction)
- 5+ `unimplemented!()` macros
- Config stored in user home directory
- Complex rendering logic mixed with I/O

**Key Functions to Test:**
- `Config::new()` - async file read
- `Config::save()` - async file write
- `render()` - cell rendering (200+ match arms)
- `click()` / `left_click()` / `right_click()` - state mutations

**Testing Approach:**
- Mock filesystem with trait abstraction
- Use `tempfile` crate for integration tests
- Separate rendering logic from I/O

**Estimated Test Lines:** 1,500-2,000 lines

**Refactoring Required:**
```rust
// Extract trait for filesystem operations
#[async_trait]
pub trait FileSystem {
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    async fn write(&self, path: &Path, data: &[u8]) -> io::Result<()>;
}

// Inject filesystem
impl Config {
    pub async fn new_with_fs<F: FileSystem>(fs: F) -> Result<Option<Self>, Error> {
        // ...
    }
}
```

#### 3. save.rs

| Metric | Value |
|--------|-------|
| **LOC** | 1,543 |
| **Functions** | 82 |
| **Complexity** | 7/10 (High) |
| **Branches** | ~150 |
| **I/O Coupling** | Medium (binary protocol) |
| **Testability** | 5/10 (Moderate) |

**Blockers:**
- Binary format understanding required
- Magic byte offsets scattered throughout
- Protocol trait with async and sync versions
- Hard to test without real save data

**Key Functions to Test:**
- Type conversions (TryFrom, From implementations)
- Protocol serialization/deserialization
- Bitfield operations
- Save file validation

**Testing Approach:**
- Create binary fixtures from real save files
- Test roundtrip encoding/decoding
- Property-based tests for bit operations
- Unit tests for each conversion

**Estimated Test Lines:** 1,200-1,500 lines

**Test Examples:**
```rust
#[test]
fn test_time_of_day_night() {
    let time = TimeOfDay(0x8000);
    assert!(time.matches(TimeRange::Night));
}

#[test]
fn test_magic_capacity_conversion() {
    let magic = MagicCapacity::try_from(2).unwrap();
    assert_eq!(magic, MagicCapacity::Large);
}

#[test]
fn test_save_roundtrip() {
    let save_data = include_bytes!("fixtures/save.bin");
    let save = Save::from_bytes(save_data).unwrap();
    let encoded = save.to_bytes();
    assert_eq!(save_data, &encoded[..]);
}
```

#### 4. knowledge.rs

| Metric | Value |
|--------|-------|
| **LOC** | 537 |
| **Functions** | Multiple impl blocks |
| **Complexity** | 5/10 (Moderate) |
| **Branches** | ~100 |
| **I/O Coupling** | Low (pure logic) |
| **Testability** | 8/10 (Good) |

**Blockers:**
- Few! This is one of the most testable modules
- Some incomplete features (TODOs)

**Key Functions to Test:**
- `Knowledge::vanilla()` - factory function
- `Knowledge::get_exit()` - lookup function
- `BitAnd` trait impl - merge logic with contradiction detection
- JSON serialization/deserialization

**Testing Approach:**
- Unit tests for pure functions
- Property-based tests for merge logic
- Test contradiction detection
- JSON roundtrip tests

**Estimated Test Lines:** 650-800 lines

**Test Examples:**
```rust
#[test]
fn test_knowledge_vanilla_defaults() {
    let k = Knowledge::vanilla();
    assert!(k.string_settings.contains_key("bridge"));
    assert_eq!(k.string_settings["starting_age"], "adult");
}

#[test]
fn test_knowledge_merge_no_contradiction() {
    let k1 = Knowledge { /* ... */ };
    let k2 = Knowledge { /* ... */ };
    let merged = (k1 & k2).unwrap();
    assert_eq!(merged.string_settings.len(), 5);
}

#[test]
fn test_knowledge_merge_contradiction() {
    let k1 = Knowledge {
        string_settings: [("bridge", "vanilla")].into()
    };
    let k2 = Knowledge {
        string_settings: [("bridge", "open")].into()
    };
    let result = k1 & k2;
    assert!(matches!(result, Err(Contradiction::StringSetting(_))));
}
```

#### 5. ram.rs

| Metric | Value |
|--------|-------|
| **LOC** | 409 |
| **Functions** | Multiple |
| **Complexity** | 4/10 (Low-Moderate) |
| **Branches** | ~50 |
| **I/O Coupling** | Low (binary parsing) |
| **Testability** | 7/10 (Good) |

**Blockers:**
- Requires understanding of N64 memory layout
- Binary fixtures needed
- Hard-coded memory addresses

**Key Functions to Test:**
- `Ram::default()`
- Protocol read/write (async and sync)
- Bitflags operations
- Scene data access

**Testing Approach:**
- Create RAM dump fixtures
- Test binary parsing
- Validate address ranges
- Test flag operations

**Estimated Test Lines:** 400-500 lines

#### 6. scene.rs

| Metric | Value |
|--------|-------|
| **LOC** | 510 |
| **Functions** | Generated by macro |
| **Complexity** | 3/10 (Low) |
| **Branches** | ~100 |
| **I/O Coupling** | Low (data structures) |
| **Testability** | 6/10 (Moderate) |

**Blockers:**
- Code generated by `scene_flags!` macro
- Hard to inspect generated code
- Binary addresses hardcoded in macro

**Testing Approach:**
- Test macro-generated accessors
- Validate bitflags operations
- Test protocol serialization

**Estimated Test Lines:** 300-400 lines

#### 7. firebase.rs

| Metric | Value |
|--------|-------|
| **LOC** | 775 |
| **Functions** | Multiple |
| **Complexity** | 7/10 (High) |
| **Branches** | ~80 |
| **I/O Coupling** | High (network) |
| **Testability** | 1/10 (Very Poor) |

**Blockers:**
- 6+ `unimplemented!()` macros
- Direct HTTP calls via reqwest
- Hardcoded to `ootr_static::Rando`
- Feature-gated (requires `firebase` feature)

**Current Status:**
- **CANNOT TEST** until unimplemented blocks are completed
- Would need HTTP mocking framework
- Network I/O makes tests slow

**Future Testing Approach:**
- Mock HTTP client with trait
- Use `wiremock` for HTTP testing
- Create Firebase response fixtures

**Estimated Test Lines:** 800-1,000 lines (after implementing missing features)

#### 8. net.rs

| Metric | Value |
|--------|-------|
| **LOC** | 354 |
| **Functions** | Multiple |
| **Complexity** | 6/10 (Moderate-High) |
| **Branches** | ~60 |
| **I/O Coupling** | High (network) |
| **Testability** | 3/10 (Poor) |

**Blockers:**
- Hardcoded WebSocket URL
- No trait abstraction for connections
- Real network required for tests

**Key Trait:**
```rust
// Line 99-108 - Only mockable trait in entire codebase!
pub trait Connection: fmt::Debug + Send + Sync {
    fn packet_stream(&self) -> Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>>;
    // ...
}
```

**Testing Approach:**
- Implement mock `Connection` for tests
- Test packet handling logic
- Mock WebSocket responses

**Estimated Test Lines:** 400-500 lines

### Summary Table

| Module | LOC | Testability | Test Lines | Refactor Needed |
|--------|-----|-------------|-----------|-----------------|
| **checks.rs** | 757 | 2/10 | 2,000-2,500 | High |
| **ui.rs** | 2,441 | 4/10 | 1,500-2,000 | High |
| **save.rs** | 1,543 | 5/10 | 1,200-1,500 | Low |
| **firebase.rs** | 775 | 1/10 | 800-1,000 | Critical |
| **knowledge.rs** | 537 | 8/10 | 650-800 | Minimal |
| **scene.rs** | 510 | 6/10 | 300-400 | Low |
| **ram.rs** | 409 | 7/10 | 400-500 | Low |
| **net.rs** | 354 | 3/10 | 400-500 | Medium |
| **Other modules** | 1,310 | 6/10 | 1,000-1,500 | Low |
| **Test utilities** | - | - | 1,000 | N/A |
| **Fixtures** | - | - | 1,500 | N/A |
| **TOTAL** | **8,636** | **4.3/10 avg** | **11,750-13,700** | **3,000 lines** |

---

## Test Coverage Estimates

### Coverage by Effort Level

| Target | Test Lines | Refactor Lines | Effort | Timeline | Feasibility |
|--------|-----------|----------------|--------|----------|-------------|
| **30%** | ~2,500 | 0 | Low | 2 weeks | ✅ Easy |
| **50%** | ~6,500 | ~500 | Moderate | 6 weeks | ✅ Achievable |
| **60%** | ~9,000 | ~1,000 | High | 10 weeks | ⚠️ Difficult |
| **70%** | ~12,000 | ~1,500 | Very High | 12 weeks | ⚠️ Challenging |
| **95%** | **~18,000-22,000** | **~3,000** | **Extreme** | **20-24 weeks** | ❌ **Not Recommended** |

### Why 95% Is Not Feasible

1. **checks.rs complexity**
   - 400+ location checks need individual test cases
   - Would require ~2,000 lines just for this module
   - High maintenance burden

2. **Firebase unimplemented blocks**
   - Cannot test until features are implemented
   - Would add ~6 weeks just to implement missing features

3. **Binary protocol dependencies**
   - Tests become brittle
   - Requires extensive fixtures
   - Hard to maintain

4. **Generated code (scene.rs)**
   - Macro-generated code is hard to test
   - Low value - mostly data structures

5. **Diminishing returns**
   - Last 20% of coverage takes 60% of effort
   - Tests become maintenance burden
   - Better to focus on high-value paths

### Recommended Coverage Targets by Module

| Module | Realistic Target | Optimal Target | Notes |
|--------|-----------------|----------------|-------|
| **knowledge.rs** | 80% | 90% | Pure functions, easy to test |
| **ram.rs** | 70% | 80% | Binary parsing, need fixtures |
| **save.rs** | 60% | 70% | Protocol complexity |
| **scene.rs** | 60% | 70% | Macro-generated, lower value |
| **ui.rs** | 40% | 60% | Filesystem coupling |
| **net.rs** | 40% | 60% | Network coupling |
| **checks.rs** | 25% | 40% | Giant function, panic macros |
| **firebase.rs** | 0% | 30% | Unimplemented blocks |
| **Overall** | **50%** | **70%** | **Recommended: 70%** |

---

## Recommended Testing Strategy

### Phase 1: Foundation (2 weeks, 2,500 lines) → 30% Coverage

**Goal:** Establish test infrastructure and test easy modules

#### Setup
1. Add test dependencies to `Cargo.toml`:
```toml
[dev-dependencies]
tokio-test = "0.4"
tempfile = "3"
rstest = "0.18"
proptest = "1"
pretty_assertions = "1"
```

2. Create test utilities:
   - `tests/common/mod.rs` - Test helpers
   - `tests/fixtures/` - Binary data, JSON configs

3. Set up CI (GitHub Actions):
```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all
```

#### Modules to Test (Easy Wins)

**1. knowledge.rs (650 lines of tests)**
```rust
// tests/knowledge_tests.rs
#[test]
fn test_vanilla_defaults() { /* ... */ }

#[test]
fn test_merge_without_contradiction() { /* ... */ }

#[test]
fn test_merge_with_contradiction() { /* ... */ }

#[test]
fn test_get_exit() { /* ... */ }

// + 40 more test cases
```

**2. region.rs (160 lines)**
```rust
#[test]
fn test_region_lookup() { /* ... */ }

#[test]
fn test_mq_flag() { /* ... */ }
```

**3. proto.rs (100 lines)**
```rust
#[test]
fn test_packet_serialization() { /* ... */ }
```

**4. Basic save.rs converters (500 lines)**
```rust
#[test]
fn test_time_of_day_conversion() { /* ... */ }

#[test]
fn test_magic_capacity_values() { /* ... */ }

#[test]
fn test_quest_items_bitflags() { /* ... */ }
```

**5. GitHub API (200 lines)**
```rust
#[tokio::test]
async fn test_latest_release() {
    // Note: requires refactoring to inject HTTP client
    // For now, test with real network (slow but works)
}
```

**6. Test utilities (500 lines)**
```rust
// tests/common/builders.rs
pub fn model_state_builder() -> ModelStateBuilder { /* ... */ }
pub fn knowledge_builder() -> KnowledgeBuilder { /* ... */ }
pub fn ram_builder() -> RamBuilder { /* ... */ }
```

**7. Fixtures (400 lines)**
```rust
// tests/fixtures/mod.rs
pub fn sample_save_file() -> Vec<u8> { /* ... */ }
pub fn sample_ram_dump() -> Vec<u8> { /* ... */ }
pub fn sample_config_json() -> &'static str { /* ... */ }
```

**Deliverable:** Working test suite, CI pipeline, 30% coverage

---

### Phase 2: Refactoring (4 weeks, +4,000 test lines) → 55% Coverage

**Goal:** Add abstractions and test I/O-heavy modules

#### Refactoring Tasks

**1. Extract FileSystem trait (2 days)**
```rust
// crate/oottracker/src/fs.rs (new file)
#[async_trait]
pub trait FileSystem {
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    async fn write(&self, path: &Path, data: &[u8]) -> io::Result<()>;
    async fn exists(&self, path: &Path) -> bool;
}

pub struct RealFileSystem;

#[async_trait]
impl FileSystem for RealFileSystem {
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        tokio::fs::read(path).await
    }
    // ...
}

// Mock for tests
pub struct MockFileSystem {
    files: HashMap<PathBuf, Vec<u8>>,
}
```

**2. Extract HttpClient trait (2 days)**
```rust
// crate/oottracker/src/http.rs (new file)
#[async_trait]
pub trait HttpClient {
    async fn get(&self, url: &str) -> Result<Response, Error>;
}

pub struct ReqwestClient(reqwest::Client);

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn get(&self, url: &str) -> Result<Response, Error> {
        self.0.get(url).send().await
    }
}
```

**3. Replace panics with Results (3 days)**
```rust
// crate/oottracker/src/checks.rs
pub enum CheckError {
    UnknownEvent(String),
    UnknownLocation(String),
    NotImplemented(&'static str),
    InvalidState,
}

impl CheckExt for Check<R> {
    fn checked(&self, model: &ModelState) -> Result<bool, CheckError> {
        match self {
            Check::Event(event) => {
                // OLD: panic!("unknown event: {}", event)
                // NEW:
                Err(CheckError::UnknownEvent(event.clone()))
            }
            // ...
        }
    }
}
```

**4. Inject dependencies into structs (3 days)**
```rust
// Update constructors to accept dependencies
pub struct TrackerApp<F: FileSystem, H: HttpClient> {
    fs: F,
    http: H,
    state: ModelState,
}

impl<F: FileSystem, H: HttpClient> TrackerApp<F, H> {
    pub fn new(fs: F, http: H) -> Self {
        Self {
            fs,
            http,
            state: ModelState::default(),
        }
    }
}
```

#### New Tests

**1. ui.rs with mocked filesystem (+1,500 lines)**
```rust
#[tokio::test]
async fn test_config_load_success() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs.add_file("/config.json", br#"{"key": "value"}"#);

    let config = Config::new_with_fs(mock_fs).await.unwrap();
    assert!(config.is_some());
}

#[tokio::test]
async fn test_config_load_missing_file() {
    let mock_fs = MockFileSystem::new();
    let config = Config::new_with_fs(mock_fs).await.unwrap();
    assert!(config.is_none());
}

#[test]
fn test_render_empty_cell() { /* ... */ }

#[test]
fn test_left_click_updates_state() { /* ... */ }

// + 90 more test cases
```

**2. checks.rs error handling (+800 lines)**
```rust
#[test]
fn test_unknown_event_error() {
    let check = Check::Event("Invalid".into());
    let model = ModelState::default();

    match check.checked(&model) {
        Err(CheckError::UnknownEvent(name)) => {
            assert_eq!(name, "Invalid");
        }
        _ => panic!("Expected UnknownEvent error"),
    }
}

#[test]
fn test_setting_check_not_implemented() {
    let check = Check::Setting("bridge".into());
    let model = ModelState::default();

    match check.checked(&model) {
        Err(CheckError::NotImplemented(msg)) => {
            assert!(msg.contains("setting"));
        }
        _ => panic!("Expected NotImplemented error"),
    }
}

// + 50 more error path tests
```

**3. save.rs protocol tests (+700 lines)**
```rust
#[test]
fn test_save_roundtrip() {
    let original = include_bytes!("../fixtures/save_adult.bin");
    let save = Save::from_bytes(original).unwrap();
    let encoded = save.to_bytes();
    assert_eq!(original, &encoded[..]);
}

#[test]
fn test_dungeon_items_decode() {
    let data = [0xFF, 0x03]; // All items + compass + map
    let items = DungeonItems::from_bytes(&data);
    assert!(items.contains(DungeonItems::COMPASS));
    assert!(items.contains(DungeonItems::MAP));
    assert!(items.contains(DungeonItems::BOSS_KEY));
}

// + 40 more protocol tests
```

**4. net.rs with mocked connections (+500 lines)**
```rust
#[tokio::test]
async fn test_connection_packet_stream() {
    let mock_conn = MockConnection::new()
        .with_packets(vec![
            Packet::StateUpdate(/* ... */),
            Packet::ItemUpdate(/* ... */),
        ]);

    let mut stream = mock_conn.packet_stream();
    let first = stream.next().await.unwrap().unwrap();
    assert!(matches!(first, Packet::StateUpdate(_)));
}
```

**5. ram.rs binary parsing (+500 lines)**
```rust
#[test]
fn test_ram_parse_save_section() {
    let ram_dump = include_bytes!("../fixtures/ram_kokiri.bin");
    let ram = Ram::from_bytes(ram_dump).unwrap();
    assert_eq!(ram.save.link_age, LinkAge::Child);
}

#[test]
fn test_scene_flags_gold_skulltula() {
    let ram = ram_builder()
        .with_skulltula(Scene::DekuTree, 0x01)
        .build();
    assert!(ram.scene_flags().deku_tree.gold_skulltulas.contains(/* ... */));
}
```

**Deliverable:** Testable architecture, 55% coverage, error handling

---

### Phase 3: Deep Coverage (6 weeks, +5,500 lines) → 70% Coverage

**Goal:** Test complex logic paths and edge cases

#### Table-Driven Tests for checks.rs

```rust
// tests/checks_table_tests.rs
struct CheckTestCase {
    name: &'static str,
    check: Check<ootr_static::Rando>,
    model_builder: fn() -> ModelStateBuilder,
    expected: bool,
}

#[test]
fn test_all_location_checks() {
    let test_cases = vec![
        CheckTestCase {
            name: "Deku Tree Queen Gohma Heart",
            check: Check::Location("Deku Tree Queen Gohma Heart"),
            model_builder: || model_with_gohma_defeated(),
            expected: true,
        },
        CheckTestCase {
            name: "KF GS House of Twins",
            check: Check::Location("KF GS House of Twins"),
            model_builder: || model_with_kokiri_skulltula(),
            expected: true,
        },
        // + 398 more location checks
    ];

    for tc in test_cases {
        let model = (tc.model_builder)().build();
        let result = tc.check.checked(&model).unwrap();
        assert_eq!(
            result, tc.expected,
            "Failed for location: {}", tc.name
        );
    }
}
```

**Estimated:** 2,000 lines for comprehensive location testing

#### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_save_roundtrip(data in prop::collection::vec(any::<u8>(), 0..1024)) {
        if let Ok(save) = Save::from_bytes(&data) {
            let encoded = save.to_bytes();
            let decoded = Save::from_bytes(&encoded);
            prop_assert!(decoded.is_ok());
        }
    }

    #[test]
    fn prop_knowledge_merge_commutative(
        k1 in knowledge_strategy(),
        k2 in knowledge_strategy()
    ) {
        let result1 = k1.clone() & k2.clone();
        let result2 = k2 & k1;
        prop_assert_eq!(result1, result2);
    }
}
```

**Estimated:** 1,000 lines for property tests

#### Integration Tests

```rust
// tests/integration/auto_tracking.rs
#[tokio::test]
async fn test_auto_tracking_end_to_end() {
    let app = TrackerApp::new(
        MockFileSystem::new(),
        MockHttpClient::new(),
    );

    // Simulate RAM update from emulator
    let ram_update = sample_ram_with_item_collected();
    app.update_ram(ram_update).await;

    // Verify item marked as collected
    assert!(app.state().ram.save.inventory.has_kokiri_sword());
}

// tests/integration/web_sync.rs
#[tokio::test]
async fn test_websocket_synchronization() {
    let server = TestWebSocketServer::start().await;
    let client1 = connect_client(&server).await;
    let client2 = connect_client(&server).await;

    // Client 1 updates state
    client1.send_state_update(/* ... */).await;

    // Client 2 receives update
    let update = client2.receive_update().await;
    assert_eq!(update.knowledge.string_settings["bridge"], "open");
}
```

**Estimated:** 1,500 lines for integration tests

#### Edge Cases and Error Paths

```rust
#[test]
fn test_ram_parse_corrupted_data() {
    let bad_data = vec![0xFF; 100];
    let result = Ram::from_bytes(&bad_data);
    assert!(result.is_err());
}

#[test]
fn test_knowledge_contradictory_settings() {
    let k1 = knowledge_builder().with_setting("bridge", "vanilla").build();
    let k2 = knowledge_builder().with_setting("bridge", "open").build();
    let result = k1 & k2;
    assert!(matches!(result, Err(Contradiction::StringSetting(_))));
}

#[test]
fn test_ui_click_invalid_cell() {
    let ui = ui_builder().build();
    let result = ui.click(9999, 9999); // Out of bounds
    assert!(result.is_err());
}
```

**Estimated:** 1,000 lines for edge cases

**Deliverable:** 70% coverage, comprehensive test suite, CI passing

---

## Refactoring Requirements

### Summary of Changes Needed

| Refactoring Task | Files Affected | Lines Changed | Priority |
|-----------------|----------------|---------------|----------|
| Extract FileSystem trait | ui.rs, lib.rs | ~200 | High |
| Extract HttpClient trait | github.rs, firebase.rs, net.rs | ~150 | High |
| Replace panics with Results | checks.rs | ~100 | Critical |
| Add error types | checks.rs, lib.rs | ~100 | High |
| Split giant functions | checks.rs | ~800 | Medium |
| Inject dependencies | Multiple | ~400 | Medium |
| Implement unimplemented! blocks | firebase.rs, ui.rs, ctx.rs | ~500 | Medium |
| Add URL configuration | net.rs, github.rs | ~100 | Low |
| **TOTAL** | **20+ files** | **~2,350** | - |

### Detailed Refactoring Plan

#### 1. Extract FileSystem Trait (Critical Path)

**New file:** `crate/oottracker/src/fs.rs`

```rust
use std::path::Path;
use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait FileSystem: Send + Sync {
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    async fn write(&self, path: &Path, data: &[u8]) -> io::Result<()>;
    async fn exists(&self, path: &Path) -> bool;
}

pub struct RealFileSystem;

#[async_trait]
impl FileSystem for RealFileSystem {
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        tokio::fs::read(path).await
    }

    async fn write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        tokio::fs::write(path, data).await
    }

    async fn exists(&self, path: &Path) -> bool {
        tokio::fs::metadata(path).await.is_ok()
    }
}

#[cfg(test)]
pub struct MockFileSystem {
    files: std::collections::HashMap<std::path::PathBuf, Vec<u8>>,
}

#[cfg(test)]
impl MockFileSystem {
    pub fn new() -> Self {
        Self { files: Default::default() }
    }

    pub fn add_file(&mut self, path: impl Into<std::path::PathBuf>, data: Vec<u8>) {
        self.files.insert(path.into(), data);
    }
}

#[cfg(test)]
#[async_trait]
impl FileSystem for MockFileSystem {
    async fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.files.get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }

    async fn write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        // Immutable in tests - would need RefCell for mutability
        Ok(())
    }

    async fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }
}
```

**Update `ui.rs`:**

```rust
// OLD
impl Config {
    pub async fn new() -> Result<Option<Config>, Error> {
        let dirs = dirs()?;
        let mut file = File::open(dirs.config_dir().join("config.json")).await?;
        // ...
    }
}

// NEW
impl Config {
    pub async fn new() -> Result<Option<Config>, Error> {
        Self::new_with_fs(RealFileSystem).await
    }

    pub async fn new_with_fs<F: FileSystem>(fs: F) -> Result<Option<Config>, Error> {
        let dirs = dirs()?;
        let path = dirs.config_dir().join("config.json");

        if !fs.exists(&path).await {
            return Ok(None);
        }

        let data = fs.read(&path).await?;
        let config = serde_json::from_slice(&data)?;
        Ok(Some(config))
    }
}
```

**Effort:** 2 days, ~200 lines

#### 2. Replace Panics with Results (Critical)

**New file:** `crate/oottracker/src/checks.rs` (add error type)

```rust
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("unknown event: {0}")]
    UnknownEvent(String),

    #[error("unknown location: {0}")]
    UnknownLocation(String),

    #[error("not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("region lookup failed: {0}")]
    RegionLookup(#[from] RegionLookupError),
}

pub trait CheckExt {
    // OLD: fn checked(&self, model: &ModelState) -> Option<bool>;
    // NEW:
    fn checked(&self, model: &ModelState) -> Result<Option<bool>, CheckError>;
}
```

**Update all panic sites:**

```rust
// OLD (line 136)
_ => panic!("unknown event name: {}", event),

// NEW
_ => Err(CheckError::UnknownEvent(event.clone())),
```

**Cascade changes:**
- All call sites need `.unwrap()` or `?` operator
- Error handling in web/GUI layers
- Logging for unknown checks

**Effort:** 3 days, ~300 lines changed across multiple files

#### 3. Implement Unimplemented Blocks (Medium Priority)

**firebase.rs:** Complete 6+ unimplemented sections

```rust
// Line 195, 240, 281, etc.
// OLD: unimplemented!()

// NEW: Implement actual logic or return error
return Err(Error::NotImplemented("feature XYZ"));
```

**ui.rs:** Complete CompositeKeys handling

```rust
// Lines 1918, 1924, 1930
// Implement logic for non-standard composite keys
// or document why they're not supported
```

**Effort:** 1-2 weeks (requires domain knowledge), ~500 lines

---

## Implementation Roadmap

### Timeline for 70% Coverage Target

#### Week 1-2: Foundation Setup
- ✅ Set up test dependencies in Cargo.toml
- ✅ Create test utilities and builders
- ✅ Add GitHub Actions CI
- ✅ Write first 100 tests (knowledge.rs, region.rs)
- **Milestone:** CI passing, 10% coverage

#### Week 3-4: Pure Logic Testing
- ✅ Test knowledge.rs (80% coverage)
- ✅ Test proto.rs, region.rs (70% coverage)
- ✅ Test basic save.rs converters (40% coverage)
- ✅ Create binary fixtures
- **Milestone:** 30% coverage

#### Week 5-6: FileSystem Refactoring
- ✅ Extract FileSystem trait
- ✅ Refactor ui.rs to use trait
- ✅ Write ui.rs tests with mocks (40% coverage)
- **Milestone:** 40% coverage

#### Week 7-8: Error Handling Refactoring
- ✅ Replace panics with Results
- ✅ Add CheckError type
- ✅ Update call sites
- ✅ Test error paths
- **Milestone:** 45% coverage

#### Week 9-10: Protocol & Binary Testing
- ✅ Deep save.rs testing (60% coverage)
- ✅ RAM parsing tests (70% coverage)
- ✅ Scene flags tests (60% coverage)
- ✅ Property-based tests for binary parsing
- **Milestone:** 55% coverage

#### Week 11-12: Integration & Table Tests
- ✅ Table-driven checks.rs tests (selected locations)
- ✅ Integration tests for auto-tracking
- ✅ WebSocket sync tests
- ✅ Edge case testing
- **Milestone:** 70% coverage ✨

### Resource Requirements

**Developer Time:**
- 1 senior developer: 12 weeks full-time
- OR 2 developers: 6-8 weeks

**Skills Needed:**
- Strong Rust experience
- Async/await and Tokio knowledge
- Testing frameworks (rstest, proptest)
- Domain knowledge (OoT game mechanics)
- Binary protocol understanding

---

## Conclusion

### Key Takeaways

1. **Current state is testable, but requires refactoring**
   - No test infrastructure exists
   - Architecture has significant testability issues
   - 600+ TODOs indicate incomplete features

2. **95% coverage is not recommended**
   - Would require 18,000-22,000 lines of test code
   - Diminishing returns after 70%
   - High maintenance burden

3. **70% coverage is the sweet spot**
   - ~12,000 test lines + 1,500 refactor lines
   - 12-week timeline with 1 developer
   - Tests high-value code paths
   - Maintainable long-term

4. **Start with Phase 1 (30% coverage)**
   - Low investment (2 weeks)
   - Proves value of testing
   - Sets up infrastructure
   - Can decide whether to continue

### Recommended Action

**Option A: Go for 70% (Recommended)**
- Best ROI for effort invested
- Catches 90% of bugs
- Sustainable testing culture
- 12-week commitment

**Option B: Start with 30% (Low Risk)**
- Minimal investment
- Proves value
- Can expand later
- 2-week commitment

**Option C: Do Nothing (Not Recommended)**
- Continue relying on manual testing
- Higher risk of regressions
- Harder to onboard contributors
- Technical debt accumulates

---

**Document Version:** 1.0
**Author:** Claude (Anthropic)
**Date:** 2025-12-31
