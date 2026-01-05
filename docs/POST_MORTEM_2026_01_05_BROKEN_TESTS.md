# Post-Mortem: Broken Tests Merged to Main

**Date:** 2026-01-05
**Author:** Claude
**Severity:** Medium
**Status:** Root cause identified, fix pending

## Summary

5 failing tests were merged to the main branch without CI catching them. All tests passed in CI, but the failing tests were in a crate (`oottracker`) that CI was not configured to test.

## Timeline

| Time | Event |
|------|-------|
| 2026-01-04 23:31 | PR #416 opens with new tests in `oottracker` crate |
| 2026-01-04 23:31 | CI runs - all checks pass (9/9) |
| 2026-01-04 23:32 | PR #416 merged to main |
| 2026-01-05 00:00 | Manual `cargo test -p oottracker` reveals 5 failures |

## Failing Tests

```
flag_mapping::tests::test_mq_active_mappings_default
world_database_tests::test_world_database_has_locations
world_database_tests::test_world_database_has_mm_regions
world_database_tests::test_world_database_has_oot_regions
world_database_tests::test_world_database_region_count
```

## Root Cause Analysis

### Primary Cause: CI Gap in Test Coverage

The CI workflow (`ci.yml`) explicitly tests only 3 crates:

```yaml
# From .github/workflows/ci.yml line 67
- name: Run tests
  run: cargo test -p ootr -p oottracker-derive -p ootmm
```

The **`oottracker` crate is not included** in CI testing.

### Why `oottracker` Was Excluded

Looking at the CI configuration, `oottracker` was likely excluded because:

1. **Python dependency**: The full workspace build requires Python/randomizer setup
2. **Platform-specific code**: `oottracker` has Windows-specific code that fails on Linux CI
3. **Complexity**: Easier to test isolated crates than the full integration

This was a reasonable decision initially, but it created a blind spot.

### Contributing Factors

1. **No local test run before merge**: Workers created PRs and they were merged after CI passed, without manual verification
2. **False confidence from green CI**: All 9 CI checks showed ✅, suggesting everything was fine
3. **Test isolation assumptions**: Tests were added assuming CI would catch issues
4. **Parallel worker workflow**: Multiple PRs merged in quick succession without integration testing

## Impact

- **5 tests failing** on main branch
- **False sense of security** - CI shows passing but tests are broken
- **Technical debt** - broken tests reduce confidence in test suite
- **Discovery delay** - issues found only during manual analysis

## What Went Well

- Tests *were* written (even if they don't pass)
- Failure was caught relatively quickly (same day)
- Root cause was easy to identify once investigated

## What Went Wrong

1. **CI configuration incomplete** - major crate excluded from testing
2. **No integration test gate** - PRs merged without full workspace test
3. **Worker workflow bypassed verification** - automated PR creation/merge
4. **Assumed CI was comprehensive** - didn't question what CI actually tests

## Corrective Actions

### Immediate (Today)

- [ ] Create issue to fix failing tests (#424, #425) ✅ Done
- [ ] Document CI gap in this post-mortem ✅ Done

### Short-term (This Week)

- [ ] **Fix CI to test `oottracker` crate**
  - Add conditional compilation for Windows-specific code
  - Or add Linux-compatible test subset

```yaml
# Proposed fix
- name: Run tests
  run: |
    cargo test -p ootr -p oottracker-derive -p ootmm
    cargo test -p oottracker --lib  # Add this line
```

- [ ] Add pre-merge hook for worker workflow to run full tests

### Long-term

- [ ] Audit all crates to ensure CI coverage
- [ ] Add CI badge per-crate showing test status
- [ ] Consider requiring local `cargo test --workspace` before PR approval
- [ ] Add integration test job that runs full workspace tests (even if slow)

## Lessons Learned

1. **CI green ≠ All tests pass** - understand exactly what CI tests
2. **Automation requires verification** - worker workflow should include test gates
3. **Test what you ship** - if a crate is deployed, it should be tested in CI
4. **Document CI scope** - make clear what IS and ISN'T tested

## Related Issues

- #423 - [Epic] Stabilize Randomizer Tracking
- #424 - Fix MQ location filtering
- #425 - Update world_database tests

## Appendix: CI Configuration Analysis

```yaml
# Current CI test scope (ci.yml)
Crates Tested:
  ✅ ootr
  ✅ oottracker-derive
  ✅ ootmm

Crates NOT Tested:
  ❌ oottracker        <- FAILING TESTS HERE
  ❌ oottracker-web
  ❌ oottracker-gui
  ❌ oottracker-bizhawk
  ❌ oottracker-e2e
  ❌ ootr-dynamic
  ❌ ootr-static
```

The gap is significant - the main integration crate (`oottracker`) which ties everything together is not tested.
