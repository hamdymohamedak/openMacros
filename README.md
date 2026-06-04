# openMacros v2

A script-like macro toolkit for Rust: short, readable automation code with Rust speed and memory efficiency.

openMacros v2 extends the v1 contract with `script!`, `prelude`, richer filesystem/env helpers, stricter shell behavior, and optional JSON/time features.

## v2 API Contract

### Naming rules

- Verb-first actions: `say!`, `ask!`, `cmd!`, `open!`, `bail!`
- Resource-first filesystem: `file_read!`, `file_write!`, `dir_mk!`, `dir_mkp!`
- Short utilities: `trim!`, `split!`, `parse_int!`, `parse_float!`
- Flow macros: `when!`, `unless!`, `repeat!`, `each!`, `defer!`
- Smart helpers: `retry!`, `measure_ms!`, `ensure!`, `ensure_msg!`
- Entry: `script!` generates `main` with `AkResult` error handling

### Signature rules

- Fallible operations return `AkResult<_>`.
- `cmd!` fails on non-zero exit codes (use `cmd_ok!` for v1 stdout-only behavior).
- Pure transforms return values directly.
- Optional features: `serde` (`json_read!`, `json_write!`), `time` (`now!`, `today!`).

## Project Structure

```text
openMacros/
  src/
    lib.rs
    error.rs
    core.rs
    macros/
      entry.rs        # script!, bail!, bail_if!
      control.rs
      io.rs
      fs.rs
      env.rs
      system.rs
      string.rs
      json.rs         # feature serde
      time.rs         # feature time
  tests/
    macro_api.rs
    fs_env.rs
  examples/
    enterprise_job.rs
    hello_script.rs
```

## Quick Start

```rust
use open_macros::*;

script! {
    say!("Welcome to openMacros v2");

    let lang = ask!("Favorite language: ");
    when!(lang == "rust" => {
        say!("Great choice.");
    }, else => {
        say!("Give Rust a shot.");
    });

    repeat!(i in 0 => 3, {
        say!("tick {}", i);
    });

    file_write!("example.txt", "hello from v2")?;
    let out = cmd!("echo done")?;
    say!("command output: {}", out);
}
```

For explicit types without a glob import, use `open_macros::prelude::{AkError, AkResult, ...}`.

## Macro Reference

### Entry and errors

- `script! { ... }` — `main` with automatic error print + exit code 1
- `bail!("message")` — early return with validation error
- `bail_if!(cond, "message")`
- `bail_err("message")?` / `bail_if_fn(cond, "message")?` — function form in `prelude`

### Output and input

- `say!(...)`, `ask!(prompt)`

### Control flow

- `when!`, `unless!`, `repeat!`, `each!`, `retry!`, `measure_ms!`, `ensure!`, `ensure_msg!`, `defer!`

### Shell and system

- `cmd!("ls")` — stdout on success, fails on non-zero exit
- `cmd_ok!("ls")` — stdout only (v1 behavior)
- `cmd_out!`, `cmd_err!` — stdout / stderr helpers
- `os!()`, `open!(url)`, `month_now!()`, `year_now!()`
- `now!()`, `today!()` with feature `time`

### Filesystem

- `file_read!`, `file_write!`, `file_exists!`, `file_rm!`
- `read!`, `read_bytes!`, `lines!`
- `dir_mk!`, `dir_mkp!`, `dir_rm!`, `dir_wipe!`
- `path_join!("a", "b")`

### Environment

- `env_get!("KEY")`, `env_or!("KEY", "default")`, `env_set!("KEY", "value")`

### String and numbers

- `str_make!`, `upper!`, `lower!`, `trim!`, `split!`
- `parse_int!`, `parse_float!`, `pos!`, `neg!`

### JSON (feature `serde`)

- `json_read!("config.json", Config)?`
- `json_write!("out.json", &value)?`

## Features

```toml
[dependencies]
open_macros = { version = "2", features = ["serde", "time"] }
```

- `serde` — JSON read/write macros
- `time` — accurate `now!` / `today!` via chrono

## Migration v1 → v2

| v1 | v2 |
|----|-----|
| `fn main() -> Result<(), Box<dyn Error>>` | `script! { ... }` |
| `cmd!` ignores exit code | `cmd!` fails on non-zero; use `cmd_ok!` for old behavior |
| Manual `Ok(())` | implicit in `script!` |
| — | `file_read!`, `env_get!`, `unless!`, `bail!`, etc. |

## Quality

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Agent skill

Cursor agents can load `.cursor/skills/openmacros/SKILL.md` for macro API rules, migration notes, and contribution patterns.

## License

MIT
