# Error Handling Policy

This document establishes consistent error handling patterns for the project. These guidelines ensure errors surface predictably and are handled appropriately based on context.

## Core Principles

1. **No silent failures** - All errors must surface. Never swallow exceptions or ignore return codes without explicit justification.

2. **Match mechanism to context** - Use the right error handling approach for each situation (see Patterns by Context below).

3. **Maintain consistency** - Follow the same patterns throughout the codebase to make error behavior predictable.

## Patterns by Context

### C APIs (Cross-DLL Boundaries)

For functions exposed across DLL boundaries, use C-compatible error handling:

- Return error codes (`int` or `enum`) from functions
- Use output parameters for results
- Document all error codes in header comments

```cpp
// GOOD: C API with documented error codes
/**
 * Reads game state from memory.
 * @param[out] state Pointer to receive game state
 * @return 0 on success, -1 if memory read failed, -2 if state invalid
 */
int read_game_state(GameState* state);
```

### C++ Internals

For internal C++ code, choose based on error type:

| Error Type | Mechanism | Example |
|------------|-----------|---------|
| Programming/logic bugs | Exceptions | Null pointer dereference, invalid state |
| Expected "not found" cases | `std::optional<T>` | Cache miss, config key missing |
| Recoverable errors | `std::expected<T, E>` or `Result<T>` | File not found, parse error |

```cpp
// GOOD: std::optional for expected "not found"
std::optional<Player> find_player(uint32_t id) {
    auto it = players.find(id);
    if (it == players.end()) {
        return std::nullopt;
    }
    return it->second;
}

// GOOD: Exception for programming error
void process_item(Item* item) {
    if (!item) {
        throw std::invalid_argument("item cannot be null");
    }
    // ...
}

// GOOD: Result type for recoverable error
Result<Config> load_config(const std::string& path) {
    if (!file_exists(path)) {
        return Error("Config file not found: " + path);
    }
    // ...
}
```

### Resource Acquisition (RAII)

Always use RAII for resource management:

- Wrap resources in RAII types (`ScopedLibrary`, smart pointers)
- Cleanup happens via destructors, not manual calls
- Document ownership transfers explicitly

```cpp
// GOOD: RAII with smart pointers
class TextureManager {
    std::unordered_map<std::string, std::unique_ptr<Texture>> textures;
public:
    // Ownership is clear: TextureManager owns the textures
    Texture* get(const std::string& name);
};

// GOOD: Scoped resource wrapper
class ScopedLibrary {
    HMODULE handle;
public:
    explicit ScopedLibrary(const char* name) : handle(LoadLibrary(name)) {}
    ~ScopedLibrary() { if (handle) FreeLibrary(handle); }

    ScopedLibrary(const ScopedLibrary&) = delete;
    ScopedLibrary& operator=(const ScopedLibrary&) = delete;
};
```

### Logging

Use libultraship logging facilities consistently:

- Use `SPDLOG_*` macros, not `std::cerr` or `printf`
- Gate debug output by log level
- No unconditional stderr output in production code

```cpp
// GOOD: Appropriate log levels
SPDLOG_INFO("Loading save file: {}", path);
SPDLOG_WARN("Config key '{}' missing, using default", key);
SPDLOG_ERROR("Failed to read memory at 0x{:08X}", address);
SPDLOG_DEBUG("Cache hit for player {}", player_id);  // Only in debug builds
```

## Anti-Patterns

Avoid these common mistakes:

```cpp
// BAD: Silent failure - error is swallowed
bool load_data() {
    try {
        data = read_file("data.bin");
    } catch (...) {
        return false;  // Caller has no idea what went wrong
    }
    return true;
}

// BAD: stderr in production code
void process_event(Event* e) {
    if (!e) {
        std::cerr << "null event" << std::endl;  // Use logging instead
        return;
    }
}

// BAD: Manual cleanup without RAII
void load_texture(const char* path) {
    FILE* f = fopen(path, "rb");
    if (!f) return;

    Texture* tex = new Texture();
    if (!tex->load(f)) {
        fclose(f);  // Easy to forget on error paths
        delete tex;
        return;
    }
    fclose(f);  // Duplicated cleanup
    // ...
}
```

## Good Examples

```cpp
// GOOD: Explicit error handling with Result type
Result<SaveData> load_save(const std::string& path) {
    auto file = ScopedFile::open(path, "rb");
    if (!file) {
        return Error("Cannot open save file: " + path);
    }

    auto header = file.read<SaveHeader>();
    if (!header) {
        return Error("Failed to read save header");
    }

    if (header->magic != SAVE_MAGIC) {
        return Error("Invalid save file format");
    }

    SPDLOG_INFO("Loaded save from {}", path);
    return SaveData::from_file(file);
}

// GOOD: RAII ensures cleanup on all paths
class Connection {
    std::unique_ptr<Socket> socket;
    std::unique_ptr<Buffer> buffer;

public:
    static Result<Connection> connect(const std::string& host, int port) {
        auto socket = Socket::connect(host, port);
        if (!socket) {
            SPDLOG_ERROR("Connection failed to {}:{}", host, port);
            return Error("Connection failed");
        }

        auto buffer = std::make_unique<Buffer>(BUFFER_SIZE);
        SPDLOG_INFO("Connected to {}:{}", host, port);

        return Connection{std::move(socket), std::move(buffer)};
    }

    // Destructor handles cleanup automatically
    ~Connection() {
        if (socket) {
            SPDLOG_DEBUG("Closing connection");
        }
    }
};
```

## Summary

| Context | Mechanism | Key Rule |
|---------|-----------|----------|
| C APIs | Error codes + output params | Document in headers |
| Logic bugs | Exceptions | Fail fast |
| Not-found cases | `std::optional` | Caller checks |
| Recoverable errors | `Result<T>` / `std::expected` | Propagate with context |
| Resources | RAII wrappers | No manual cleanup |
| Diagnostics | libultraship logging | No raw stderr |
