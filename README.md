![Quo Preview](https://cms.protoqol.nl/assets/2ecc5f44-5fe5-4f15-95d6-ba365f4fcd5c)

![Build status](https://img.shields.io/github/actions/workflow/status/Protoqol/Quo-rust/test.yml?style=flat-square&color=%23ec135b&logo=rust)
![docs.rs](https://img.shields.io/docsrs/quo-rust?style=flat-square&logo=docsdotrs&color=%23ec135b&link=http%3A%2F%2Fdocs.rs%2Fcrate%2Fquo-rust%2Flatest)
![GPL-3.0 license](https://img.shields.io/crates/l/quo-rust?style=flat-square&color=%23ea135a)

Quo is a cross-platform variable dumper designed to make debugging easier. It receives data from your application and
displays it in a clean desktop interface, allowing you to inspect complex values in real-time without cluttering your
terminal or browser console.

> **Note**: This package requires the [Quo desktop client](https://github.com/Protoqol/Quo) to be running to display the debug data.

### Noteworthy features

- **Debug-only**: The macro only executes in debug mode (`#[cfg(debug_assertions)]`). In release builds, it compiles to nothing, ensuring zero overhead.
- **Multiple arguments**: Inspect multiple variables or expressions in a single call.
- **Rich Metadata**: Capture stack traces, system metrics, memory addresses, and more.

### Installation

Add `quo` to your `Cargo.toml` under `dependencies`:

```toml
[dependencies]
quo = { version = "0.1", package = "quo-rust" }
```

To enable additional data capture, use feature flags:

```toml
[dependencies]
# Enable specific features
quo = { version = "0.1", package = "quo-rust", features = ["stack-trace", "system-info", "hashing"] }

# Or enable everything
quo = { version = "0.1", package = "quo-rust", features = ["full"] }
```

### Available Features

- `stack-trace`: Captures the call stack and the caller function name.
- `system-info`: Captures current CPU and memory usage of the process.
- `hashing`: Generates a reproducible grouping hash for variables (`var_type:name:origin`), allowing the Quo client to group and diff values over time.
- `full`: Enables all the above features.

Basic info like **Thread ID/Name**, **Runtime Environment** (OS/Arch), and **Memory Address** are included by default.

### Usage

Import the macro and pass variables to inspect:

```rust
use quo::quo;

#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}

fn main() {
    let user_id = 42;
    let user = User { id: 1, username: "jdoe".to_string() };

    // Dump a single variable
    quo!(user_id);

    // Dump multiple variables at once
    quo!(user_id, user);
    
    // Some quick maths
    quo!(42 * 42);
}
```

### Configuration

You can customize the Quo server address using environment variables at compile time:

- `QUO_HOST`: The host where Quo is running (default: `http://127.0.0.1`, Quo always listens on 127.0.0.1 so changing this has no use).
- `QUO_PORT`: The port Quo is listening on (default: `7312`)

> The correct port can be found in the bottom left in the Quo client.

> Note: The Quo client always uses 127.0.0.1 as host, it is **not** recommended to have it set to any other host.


You can set these in your `Cargo.toml` as follows

```toml 
[env]
QUO_HOST = "http://127.0.0.1"
QUO_PORT = "7312"
```

---

# Dependency justification

| Crate                                                          | Used for                                                                                                                                                                                 |
|----------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `serde`                                                        | Serializes payload structs to JSON so they can be sent to the Quo desktop client over HTTP.                                                                                              |
| `chrono`                                                       | Captures the current timestamp (date + time) at the moment a `quo!` call is made. The `wasmbind` feature enables correct time resolution in WebAssembly targets.                         |
| `twox-hash`                                                    | Provides the fast, deterministic XXHash-64 algorithm used to generate the grouping hash (`var_type:name:origin`) that lets the Quo client group and diff values over time.               |
| `uuid`                                                         | Generates a random UUIDv4 that uniquely identifies each individual dump event sent to the client.                                                                                        |
| `backtrace` *(optional — `stack-trace` feature)*               | Captures the call stack and the immediate caller's function name at the point where `quo!` is invoked.                                                                                   |
| `sysinfo` *(optional — `system-info` feature)*                 | Reads the current process's CPU and memory usage to include live system metrics in the payload.                                                                                          |
| `ureq` *(native targets only)*                                 | Sends the serialized payload to the Quo desktop client over HTTP. Compiled with `default-features = false` to keep the binary lean; only the `json` feature is enabled.                  |
| `ehttp` *(WASM targets only)*                                  | Performs the HTTP request from within a WebAssembly context, where `ureq` (which relies on OS threads and sockets) cannot be used.                                                       |
| `serde-json-wasm` *(WASM targets only)*                        | Provides `serde_json`-compatible JSON serialization that works in `no_std` / WASM environments (the standard `serde_json` depends on `std` I/O primitives unavailable in WASM).          |
| `getrandom` v0.4 *(WASM targets only)*                         | Supplies the cryptographically secure random-number source that `uuid`'s v4 generator requires; the `wasm_js` feature routes entropy through the browser's `crypto.getRandomValues` API. |
| `getrandom` v0.3 *(WASM targets only, aliased `getrandom_v3`)* | Same purpose as the v0.4 entry above, but satisfies transitive dependencies that pin to the v0.3 API. Both versions must be present because their APIs are incompatible.                 |
| `wasm-bindgen-test` *(WASM dev/test only)*                     | Enables running Rust tests directly inside a headless browser or Node.js, which is required to verify WASM-specific code paths.                                                          |

---
## License

Quo is open-source software licensed under the [GPL-3 license](LICENSE).

