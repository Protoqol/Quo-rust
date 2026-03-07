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
- **Multiple arguments**: Inspect multiple variables in a single call.

### Installation

Add `quo` to your `Cargo.toml` under `dev-dependencies`:

```toml
[dev-dependencies]
quo = { version = "0.1.7", package = "quo-rust" }
```

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

## License

Quo is open-source software licensed under the [GPL-3 license](.github/LICENSE).

