# openMacros v2

A script-like macro toolkit for Rust: short, readable automation code with Rust speed and memory efficiency.

## Quick Start

```rust
use open_macros::*;

script! {
    say!("Welcome to openMacros");

    let msg = tpl!("Hello {{who}}!", who = "Rust")?;
    when!(contains!(msg, "Rust") => {
        say!("{}", msg);
    });

    file_write!("example.txt", "hello")?;
    let out = cmd!("echo done")?;
    say!("{}", out);
}
```

## Features

```toml
[dependencies]
open_macros = "2.0"

# Optional:
open_macros = { version = "2.0", features = ["text", "async", "serde", "time"] }
```

| Feature | Description |
|---------|-------------|
| *(default)* | Core macros: I/O, FS, env, shell, flow, text (`contains`, `replace`, `tpl`) |
| `text` | Regex: `regex_match!`, `regex_find!`, `regex_replace!`, `regex_split!` |
| `async` | Tokio: `script_async!`, `sleep_async!`, `spawn!`, async file/shell |
| `advanced` | Enables `text` + `async` |
| `serde` | `json_read!`, `json_write!` |
| `time` | `now!`, `today!` |

## Text macros

| Macro | Returns |
|-------|---------|
| `contains!(hay, needle)` | `bool` |
| `replace!(s, from, to)` | `String` (first match) |
| `replace_all!(s, from, to)` | `String` |
| `tpl!("Hi {{name}}", name = x)` | `AkResult<String>` |

With feature `text`:

| Macro | Returns |
|-------|---------|
| `regex_match!(s, pat)` | `AkResult<bool>` |
| `regex_find!(s, pat)` | `AkResult<Option<String>>` |
| `regex_replace!(s, pat, repl)` | `AkResult<String>` |
| `regex_split!(s, pat)` | `AkResult<Vec<String>>` |

## Async macros (feature `async`)

```rust
use open_macros::*;

script_async! {
    sleep_async!(50);
    file_write_async!("target/a.txt", "data").await?;
    let body = file_read_async!("target/a.txt").await?;
    let out = cmd_async!("echo ok").await?;
    say!("{}", body);
}
```

| Macro | Notes |
|-------|-------|
| `script_async! { }` | `#[tokio::main]` + `AkResult` error handling |
| `sleep_async!(ms)` | Sleeps inline (no extra `.await`) |
| `spawn! { ... }` | `tokio::spawn` background task |
| `file_read_async!` / `file_write_async!` | Tokio FS |
| `cmd_async!` | Fails on non-zero exit (like `cmd!`) |

## Entry and errors

- `script! { ... }` — sync `main` wrapper
- `bail!`, `bail_if!`, `ensure!`, `ensure_msg!`
- `prelude::{AkError, AkResult, ...}` for types

## Project structure

```text
src/
  core.rs, core/text.rs, core/async_ops.rs
  macros/entry.rs, text.rs, async.rs, ...
examples/
  hello_script.rs, enterprise_job.rs, async_fetch.rs
tests/
  macro_api.rs, fs_env.rs, text.rs, async.rs
```

## Migration

| From | To |
|------|-----|
| v1 `main` + `Box<dyn Error>` | `script! { }` |
| v1 `cmd!` (ignore exit) | `cmd_ok!` or v2 `cmd!` |
| — | v2.0 `tpl!`, `contains!`, `script_async!` |

## Quality

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Agent skill

See `SKILL.md` or `.cursor/skills/openmacros/SKILL.md`.

## License

MIT
