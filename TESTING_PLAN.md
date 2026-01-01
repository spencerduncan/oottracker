# Unit Test Coverage Improvement Plan

This plan outlines a phased approach to incrementally increase unit test coverage from the current **0%** to meaningful coverage across the core crates.

## Current State

| Crate | Lines | Existing Tests | In CI |
|-------|-------|----------------|-------|
| `ootr` | 544 | 0 | Yes |
| `ootr-dynamic` | 349 | 1 (ignored) | Yes |
| `oottracker-derive` | 1,053 | 0 | Yes |
| `ootmm` | 1,543 | 19 | **No** |

**Total testable lines:** ~2,025 (in CI-covered crates)

---

## Phase 1: Quick Wins (ootr crate)

**Goal:** Add foundational tests to pure data structures with no external dependencies.

### 1.1 `ootr/src/model.rs` - Enum parsing and display (~15 tests)

This module contains pure enum types that are highly testable:

```rust
// Test targets:
- Dungeon::from_str() for all 13 dungeon variants
- MainDungeon::from_reward_location() / reward_location() round-trip
- Medallion::element() for all 6 medallions
- Stone::from_str() for all 3 stones
- DungeonReward conversions
- Display implementations
```

**Example tests:**
- `test_dungeon_from_str_deku_tree`
- `test_dungeon_from_str_invalid_returns_none`
- `test_main_dungeon_reward_location_round_trip`
- `test_medallion_element_mapping`

### 1.2 `ootr/src/check.rs` - Check enum Display (~8 tests)

```rust
// Test targets:
- Check::Location display formatting
- Check::Exit display formatting
- Check::Event display formatting
- All 7 Check variants
```

### 1.3 `ootr/src/region.rs` - Region structure (~5 tests)

```rust
// Test targets:
- Region field accessors
- Default values
- Debug/Clone derives
```

### 1.4 `ootr/src/item.rs` - Item wrapper (~4 tests)

```rust
// Test targets:
- Item::name() getter
- Item equality
```

**Phase 1 Total:** ~32 new tests

---

## Phase 2: ootr-dynamic crate

**Goal:** Add tests for parsing logic that doesn't require Python runtime.

### 2.1 `ootr-dynamic/src/region.rs` - Dungeon filename parsing (~10 tests)

```rust
// Test targets:
- parse_dungeon_info("Deku Tree") -> Some((Dungeon::DekuTree, Mq::Vanilla))
- parse_dungeon_info("Forest Temple MQ") -> Some((Dungeon::ForestTemple, Mq::Mq))
- parse_dungeon_info("Invalid Name") -> None
- All dungeon name formats
- MQ suffix handling
```

### 2.2 `ootr-dynamic/src/lib.rs` - JSON parsing helper (~5 tests)

```rust
// Test targets:
- read_json_lenient_sync() with valid JSON
- read_json_lenient_sync() with JSON containing comments
- Error handling for malformed JSON
```

**Phase 2 Total:** ~15 new tests

---

## Phase 3: ootmm crate (add to CI)

**Goal:** Expand existing tests and add ootmm to CI pipeline.

### 3.1 Update CI configuration

Add `ootmm` to the test and coverage jobs in `.github/workflows/ci.yml`:

```yaml
# In test job:
- name: Run tests
  run: cargo test -p ootr -p ootr-dynamic -p oottracker-derive -p ootmm

# In coverage job:
- name: Generate coverage
  run: cargo tarpaulin -p ootr -p ootr-dynamic -p oottracker-derive -p ootmm --out xml
```

### 3.2 `ootmm/src/item/*.rs` - Expand item tests (~40 tests)

Currently has 6 tests. Expand to cover:

```rust
// Test targets:
- OotItem::by_name() for all 180+ variants
- MmItem::by_name() for all 150+ variants
- Item::by_name() priority (OoT checked first)
- Case variations (PascalCase, snake_case)
- Invalid name handling
- Shared item lookups (items in both games)
```

**Strategy:** Use parameterized test macros or test each category:
- Swords, shields, tunics
- Songs (all 12 OoT songs)
- Dungeon items (keys, maps, compasses)
- Quest items
- MM masks

### 3.3 `ootmm/src/error.rs` - Expand error tests (~8 tests)

Currently has 7 tests. Add:

```rust
// Test targets:
- Error source chaining
- All Display format strings
- Error downcasting
```

### 3.4 `ootmm/src/expr/lexer.rs` - Complete lexer tests (~20 tests)

Currently has 2 incomplete stubs. Add comprehensive tests:

```rust
// Test targets:
- Lex keywords: true, false
- Lex identifiers: has, can_use, is_adult
- Lex numbers: 0, 42, 100
- Lex strings: "item_name", escaped strings
- Lex operators: &&, ||, !
- Lex delimiters: (, ), ,
- Whitespace handling
- Error cases: unterminated strings, unexpected chars
- Complex expressions: "has(HOOKSHOT) && is_adult"
```

### 3.5 `ootmm/src/expr/ast.rs` - Expand AST tests (~8 tests)

Currently has 4 tests. Add:

```rust
// Test targets:
- Deeply nested expressions
- Expr::and(), or(), not() chaining
- Expr::call() with multiple arguments
- Display for complex trees
- Equality comparisons
```

### 3.6 `ootmm/src/region.rs` - Region structure tests (~12 tests)

```rust
// Test targets:
- Region::new() and builder methods
- Location::new() with various LocationTypes
- Exit construction
- Event construction
- Serde deserialization from YAML
```

**Phase 3 Total:** ~88 new tests

---

## Phase 4: Expression System Implementation

**Goal:** Implement and test the currently stubbed expression parser/evaluator.

### 4.1 Implement `ootmm/src/expr/parser.rs` (~30 tests)

Currently returns `todo!()`. Implement recursive descent parser:

```rust
// Grammar to implement:
expr     -> or_expr
or_expr  -> and_expr ("||" and_expr)*
and_expr -> unary ("&&" unary)*
unary    -> "!" unary | call
call     -> IDENT "(" args? ")" | primary
primary  -> "true" | "false" | NUMBER | STRING | IDENT | "(" expr ")"
```

**Tests:**
- Parse simple expressions: `true`, `false`, `42`
- Parse identifiers: `is_adult`, `has_item`
- Parse function calls: `has(HOOKSHOT)`, `can_use(LENS_OF_TRUTH)`
- Parse binary ops: `a && b`, `a || b`
- Parse unary: `!is_child`
- Parse complex: `has(HOOKSHOT) && (is_adult || has(IRON_BOOTS))`
- Error recovery and messages

### 4.2 Implement `ootmm/src/expr/eval.rs` (~25 tests)

Currently returns `todo!()`. Implement against `EvalContext` trait:

```rust
// Test with mock EvalContext:
- Evaluate literals: true -> true, false -> false
- Evaluate &&: true && false -> false
- Evaluate ||: true || false -> true
- Evaluate !: !true -> false
- Evaluate function calls with mock context
- Short-circuit evaluation
```

### 4.3 Implement builtins (~15 tests)

```rust
// ootmm/src/expr/builtins/items.rs:
- has(item) - Check if player has item
- can_use(item) - Check if item is usable in current context

// ootmm/src/expr/builtins/logic.rs:
- event(name) - Check if event occurred
- setting(name) - Check setting value
- trick(name) - Check if trick is enabled

// ootmm/src/expr/builtins/time.rs:
- is_day(), is_night() - MM time functions
```

**Phase 4 Total:** ~70 new tests

---

## Phase 5: Proc Macro Testing (Advanced)

**Goal:** Add compile-time tests for procedural macros.

### 5.1 Add `trybuild` dependency

```toml
[dev-dependencies]
trybuild = "1.0"
```

### 5.2 Create test cases for `oottracker-derive`

```
crate/oottracker-derive/tests/
├── ui/
│   ├── flags_list_valid.rs      # Should compile
│   ├── flags_list_invalid.rs    # Should fail with specific error
│   ├── scene_flags_valid.rs
│   └── scene_flags_invalid.rs
└── macros.rs                     # trybuild test runner
```

### 5.3 Integration tests

Test that generated code works correctly:

```rust
// Test that flags_list! generates correct TryFrom implementation
// Test that scene_flags! generates correct region lookups
```

**Phase 5 Total:** ~20 tests (compile-time)

---

## Implementation Order & Milestones

| Milestone | Phases | New Tests | Estimated Coverage |
|-----------|--------|-----------|-------------------|
| M1 | Phase 1 | +32 | ~5% |
| M2 | Phase 1-2 | +47 | ~10% |
| M3 | Phase 1-3 | +135 | ~25-30% |
| M4 | Phase 1-4 | +205 | ~40-50% |
| M5 | Phase 1-5 | +225 | ~50-60% |

---

## Test Organization Guidelines

### File structure
```rust
// At bottom of each source file:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        // Act
        // Assert
    }
}
```

### Naming conventions
- `test_<function>_<scenario>` - e.g., `test_dungeon_from_str_valid_input`
- `test_<type>_<behavior>` - e.g., `test_medallion_display_format`

### Test categories (use attributes)
```rust
#[test]
fn test_unit_behavior() { }

#[test]
#[ignore] // Requires external setup
fn test_integration_with_randomizer() { }
```

---

## Success Criteria

1. **Phase 1 Complete:** `cargo test -p ootr` passes with 30+ tests
2. **Phase 2 Complete:** `cargo test -p ootr-dynamic` passes with 15+ tests (excluding ignored)
3. **Phase 3 Complete:** `cargo test -p ootmm` runs in CI with 100+ tests
4. **Phase 4 Complete:** Expression system fully implemented and tested
5. **Overall Goal:** 50%+ line coverage on core crates

---

## Quick Start

To begin Phase 1, create tests in `crate/ootr/src/model.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dungeon_from_str_deku_tree() {
        assert_eq!(Dungeon::from_str("Deku Tree"), Some(Dungeon::DekuTree));
    }

    #[test]
    fn test_dungeon_from_str_case_insensitive() {
        // Add based on actual implementation
    }

    #[test]
    fn test_main_dungeon_reward_location_round_trip() {
        for dungeon in MainDungeon::iter() {
            let location = dungeon.reward_location();
            assert_eq!(MainDungeon::from_reward_location(location), Some(dungeon));
        }
    }
}
```

Run tests with:
```bash
cargo test -p ootr -p ootr-dynamic -p oottracker-derive -p ootmm
```

Generate coverage:
```bash
cargo tarpaulin -p ootr -p ootr-dynamic -p oottracker-derive -p ootmm --out Html
```
