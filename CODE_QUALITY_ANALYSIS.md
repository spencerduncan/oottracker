# OoTTracker Codebase Quality Analysis

**Analysis Date:** 2026-01-03
**Codebase:** OoTTracker (Ocarina of Time / Majora's Mask Auto-Tracker)
**Files Analyzed:** 107 Rust source files + 1 C# file across 14 crates
**Analyzer:** Claude Code (claude-opus-4-5-20251101)

---

## Executive Summary

This comprehensive code quality analysis reveals **no malicious code or hidden functionality**, but identifies significant security vulnerabilities and technical debt that require attention. The codebase is legitimate game tracking software with good architectural patterns in many areas, but has critical issues in the update mechanism and web security.

### Risk Summary

| Severity | Count | Key Examples |
|----------|-------|--------------|
| **CRITICAL** | 7 | No signature verification in updaters, compile-time code download, unauthorized room deletion |
| **HIGH** | 12 | CSRF vulnerabilities, memory leaks in FFI, WebSocket origin validation missing |
| **MEDIUM** | 25+ | Parser DoS vulnerabilities, 100+ TODOs, excessive panics, unbounded memory growth |
| **LOW** | 30+ | Code smells, magic numbers, incomplete features |

### Positive Findings

- `#![forbid(unsafe_code)]` in most crates - excellent memory safety
- No obfuscated code or hidden functionality detected
- Well-structured modular architecture
- Good test coverage in many areas
- Parameterized SQL queries prevent injection
- Firebase API keys properly gitignored

---

## CRITICAL Security Vulnerabilities

### 1. Updater Downloads Without Signature Verification

**Files:**
- `crate/oottracker-updater/src/main.rs:143-183`
- `crate/oottracker-updater-bizhawk/src/main.rs:178-386`

**Issue:** Both updater applications download executables from GitHub and run them **without any cryptographic verification**.

```rust
// Downloads and executes without verification
let mut data = response.bytes_stream();
let mut exe_file = File::create(path).await?;
while let Some(chunk) = data.try_next().await? {
    exe_file.write_all(chunk.as_ref()).await?;
}
// Then executes the downloaded file
std::process::Command::new(path).spawn()?;
```

**Impact:** Supply chain attack vector - compromised GitHub release = compromised users.

**Recommendation:** Implement Ed25519 signature verification with hardcoded public key.

---

### 2. Compile-Time Code Download Without Verification

**File:** `crate/ootr-static-derive/src/lib.rs:154-160`

```rust
let rando_download = client
    .get("https://github.com/Roman971/OoT-Randomizer/archive/Dev-R.zip")
    .send()?
    .bytes()?;
ZipArchive::new(Cursor::new(rando_download))?.extract(cache_dir)?;
```

**Issue:** The derive macro downloads Python code from GitHub during compilation without checksum or signature verification. The "Dev-R" branch can change at any time.

**Impact:** Build-time supply chain attack - compromised upstream = compromised builds.

---

### 3. WebSocket Security - No Origin Validation

**File:** `crate/oottracker-web/src/websocket.rs:611-619`

```rust
pub(crate) async fn ws_handler(
    pool: PgPool, rooms: Rooms, restreams: Restreams, mw_rooms: MwRooms, ws: warp::ws::Ws,
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |ws| client_connection(pool, rooms, restreams, mw_rooms, ws)))
}
```

**Issue:** No `Origin` header validation. Any website can connect to the WebSocket.

**Impact:** Cross-site WebSocket hijacking enables unauthorized state manipulation.

---

### 4. Unauthorized Room Operations

**File:** `crate/oottracker-web/src/websocket.rs:392-394, 358-363`

```rust
// Anyone can delete any room
ClientMessage::MwDeleteRoom { room } => {
    mw_rooms.write().await.remove(&room);
}

// Anyone can overwrite entire room state
ClientMessage::SetRaw { room, state } => {
    edit_room(pool, &rooms, room, |room| {
        room.model = state;
        Ok(())
    }).await?
}
```

**Issue:** No authorization checks - any WebSocket client can delete rooms or overwrite state.

**Impact:** Complete data destruction or takeover possible.

---

### 5. CSRF Vulnerabilities

**File:** `crate/oottracker-web/src/http.rs:419-439`

```rust
#[rocket::get("/room/<name>/click/<cell_id>")]
async fn click(...) -> Result<Redirect, Error>
```

**Issue:** State-changing operations use GET requests without CSRF protection.

**Attack:** `<img src="https://oottracker.fenhl.net/room/victim/click/5">` modifies state.

---

### 6. Embedded Secrets in Binaries

**Files:**
- `crate/oottracker-utils/src/version.rs:50`
- `crate/oottracker-utils/src/release.rs:50, 187`

```rust
.bearer_auth(include_str!("../../../assets/release-token"))
```

**Issue:** GitHub API tokens and database credentials compiled into binaries. Extractable with `strings`.

**Recommendation:** Use environment variables or secrets management.

---

### 7. Memory Safety Issues in FFI Layer

**File:** `crate/oottracker-bizhawk/OotAutoTracker/src/MainForm.cs:443-447`

```csharp
IntPtr data = Marshal.AllocHGlobal(length);  // ALLOCATED
Marshal.Copy(...);
Native.model_set_tracker_ctx(this, length, data);
// NEVER FREED - Memory leak
```

**Additional Issues:**
- Use-after-free risk in `ToRam()` (line 534-544)
- Null pointer assertions that crash instead of return errors
- No ABI version checking between C#/Rust

---

## HIGH Severity Issues

### Parser/Lexer Vulnerabilities

**File:** `crate/ootmm/src/expr/parser.rs:99`

- **Stack overflow:** Recursive `parse_unary()` has no depth limit
- **Integer overflow:** `lexer.rs:113` panics on large numbers
- **UTF-8 handling:** `lexer.rs:32` uses `bytes[pos] as char` - broken for non-ASCII

### Broken JSON Parser

**File:** `crate/ootr-dynamic/src/lib.rs:276-279`

```rust
buf.push_str(&line_buf.split('#').next()...);  // Breaks JSON strings containing #
```

**Issue:** Naive comment stripping corrupts JSON values like `{"name": "Player#1"}`.

### No Rate Limiting

**Files:** All HTTP/WebSocket endpoints

**Issue:** No rate limiting on requests, connections, or messages enables DoS attacks.

### Missing Security Headers

**Issue:** No CSP, X-Frame-Options, X-Content-Type-Options, or HSTS headers.

---

## MEDIUM Severity Issues

### Excessive Panics (88+ instances)

**Examples:**
- `save.rs:381`: `.expect("there are only 4 bottles")`
- `ctx.rs:119`: `unimplemented!("unknown boss reward index")`
- `ui.rs:617,1261,1380,4111`: Multiple `unimplemented!()` in production code
- `main.rs:601`: `unimplemented!("config version from the future")`

**Impact:** Application crashes instead of graceful error handling.

### Silent Failures Hide Bugs

**File:** `crate/ootmm/src/expr/eval.rs:173`

```rust
// Unknown identifiers silently return false instead of error
// "has(HOKSHOT)" (typo) evaluates to false, not error
```

### Misleading Code

**File:** `crate/oottracker/src/ui.rs:378-449`

```rust
fn increment(&mut self, key: DungeonReward) {
    // Can DELETE entries when name says "increment"
    Some(LinksPocket) => self.remove(&key),
}
```

**File:** `crate/ootmm/src/checks.rs:108`

Documentation says default is "Adult Link" but code defaults to `is_adult: false` (Child Link).

**File:** `crate/ootr/src/region.rs:42-55`

`PartialEq` only compares `name` and `dungeon`, ignoring all other fields - surprising behavior.

### Unbounded Memory Growth

**File:** `crate/oottracker-web/src/mw.rs:44, 68`

```rust
let (incoming_queue, mut rx) = mpsc::unbounded_channel();
let mut delay_queue = VecDeque::default();
```

**Issue:** Queues can grow without limit, causing memory exhaustion.

### Technical Debt (100+ TODOs)

**Examples:**
- `checks.rs:127-260`: 130+ unimplemented check tracking items
- `info_tables.rs:49-58`: 6 TODOs for trial clear events
- `websocket.rs:283,285,382,384`: "TODO verify that the client has access?"
- `token.rs:2`: "TODO: Implement full token handling (Issue #10)"

---

## Code Smells

### Large Files Violating SRP

| File | Lines | Issue |
|------|-------|-------|
| `ui.rs` | 4000+ | Massive UI module with mixed concerns |
| `mm_save.rs` | 2000+ | Save parsing, stubs, tests all in one file |
| `main.rs` (GUI) | 1400+ | State machine, UI, networking combined |

### Giant Functions

| Function | File | Lines |
|----------|------|-------|
| `State::update()` | main.rs | 260 lines |
| `State::view()` | main.rs | 250 lines |
| `by_name()` | oot.rs/mm.rs | 170 lines each |
| `add_oot_items_to_table()` | rando.rs | 173 lines |

### Magic Numbers

```rust
// mm_save.rs - Hardcoded offsets throughout
0x0020, 0x002C, 0x0070

// oot.rs:646-683 - Item limits without constants
40, 30, 36, 100, 999

// time.rs:112 - Unexplained tolerance
const TOLERANCE: i32 = 15;  // Why 15?
```

### Excessive Cloning (62+ instances)

Particularly in `firebase.rs` (24 clones), `save.rs` (12 clones).

### Dead Code

```rust
// item_ids.rs:1
#![allow(unused)]  // Entire file marked potentially unused

// ootr-dynamic/src/region.rs:11-12
#[allow(unused)]
dungeon: Option<String>,  // Deserialized but never used
```

---

## Architectural Issues

### Global State

**File:** `crate/oottracker/src/lib.rs:43`

```rust
static WORLD_DB: Lazy<Arc<ootr::World<ootr_static::Rando>>> = ...
```

Global static makes testing harder and prevents parallel tests.

### Tight Python Coupling

**File:** `crate/ootr-dynamic/src/lib.rs`

Entire dynamic implementation depends on Python runtime and OoT Randomizer's internal structure.

### Missing Abstractions

**File:** `crate/oottracker/src/ui.rs:452-535`

```rust
pub enum TrackerCellKind {
    // 20+ variants with boxed closures
    // No abstraction for common patterns
}
```

### Dependency Injection Issues

HTTP clients hardcoded in structs, preventing mock injection for testing.

---

## Recommendations

### Immediate (CRITICAL)

1. **Implement signature verification** for all updaters
2. **Remove compile-time code download** or add verification
3. **Add WebSocket origin validation**
4. **Implement authorization** for room operations
5. **Fix CSRF** - use POST + tokens
6. **Remove embedded secrets** from source code
7. **Fix FFI memory leaks**

### Short-Term (HIGH)

8. Add rate limiting
9. Add security headers (CSP, X-Frame-Options, etc.)
10. Fix parser DoS vulnerabilities (depth limits, bounds checking)
11. Replace panics with proper error handling
12. Add ABI version checking for FFI

### Medium-Term (MEDIUM)

13. Split large files (ui.rs, mm_save.rs)
14. Extract duplicate logic
15. Replace magic numbers with constants
16. Address TODO backlog systematically
17. Add integration tests with mocks
18. Fix misleading code (increment that deletes, etc.)

### Long-Term

19. Implement proper authentication system
20. Add comprehensive audit logging
21. Security audit of all network code
22. Reduce cloning with references
23. Create abstraction layer for UI cells

---

## Conclusion

**This codebase is legitimate, well-intentioned game tracking software with no malicious code.** However, it has accumulated significant technical debt and has critical security vulnerabilities in its update mechanism and web components.

The most urgent issues are:
1. **Updaters with no signature verification** - supply chain attack vector
2. **Web security gaps** - CSRF, missing auth, no origin validation
3. **FFI memory safety** - leaks and potential crashes

The code shows good practices in many areas (forbidding unsafe code, parameterized SQL, type safety), but needs focused security hardening and refactoring to address the identified issues.

**Overall Assessment:** MEDIUM-HIGH RISK for production deployment without addressing critical issues.

---

*This analysis was performed class-by-class using parallel subagent analysis. All findings include specific file:line references for verification.*
