---
name: openmacros
description: >-
  Writes and maintains Rust automation using open_macros (openMacros v2.0):
  script!, text/tpl/regex macros, script_async!, AkResult, optional serde/time/async.
  Use when editing this repo, adding CLI scripts, migrating v1→v2, or when the user
  mentions openMacros, open_macros, script-like Rust, or short Rust automation.
---

# openMacros (open_macros v2)

Rust crate `open_macros` (package name) / **openMacros** (project). Goal: Python/JS-style brevity in scripts while keeping Rust performance, types, and explicit errors.

## When to apply

- User works in `openMacros` repo or depends on `open_macros`.
- Task is CLI automation, glue scripts, file/shell/env workflows—not async web servers (no tokio in core).
- Prefer **macros + thin `core` helpers**; do **not** add proc-macros unless user explicitly asks.

## Dependency setup

```toml
[dependencies]
open_macros = "2"

# Optional:
open_macros = { version = "2.0", features = ["text", "async", "serde", "time"] }
```

| Feature | Enables |
|---------|---------|
| *(core)* | `contains!`, `replace!`, `replace_all!`, `tpl!` |
| `text` | `regex_match!`, `regex_find!`, `regex_replace!`, `regex_split!` |
| `async` | `script_async!`, `sleep_async!`, `spawn!`, `file_*_async!`, `cmd_async!` |
| `advanced` | `text` + `async` |
| `serde` | `json_read!`, `json_write!` |
| `time` | `now!`, `today!` (chrono) |

MSRV: **1.74**. Edition **2021**. Default build has no required deps.

## Default agent workflow

1. Add `use open_macros::*;` at the top of binaries/examples.
2. Wrap executable body in `script! { ... }` instead of manual `fn main() -> Result<(), Box<dyn Error>>`.
3. Use `?` on fallible macros inside `script!` (body runs in `AkResult<()>` closure).
4. Keep business logic in normal functions; use macros for I/O, shell, FS, flow.
5. Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets --all-features`.

## Entry pattern (always prefer this)

```rust
use open_macros::*;

script! {
    say!("Starting");
    let data = file_read!("config.txt")?;
    when!(data.is_empty() => {
        bail!("config empty");
    });
    cmd!("echo ok")?;
}
```

- `script!` generates `main`, prints errors to stderr, exits `1` on failure.
- No trailing `Ok(())` needed inside `script!`.
- For library code or tests returning `AkResult`, use functions + `?` without `script!`.

## Types and errors

```rust
// prelude (types only — macros still need `use open_macros::*`)
use open_macros::prelude::{AkError, AkResult, ShellOutput, bail_err, bail_if_fn};

// Fallible macros return AkResult<T>
pub enum AkError {
    Io(std::io::Error),
    Validation(&'static str),
    Command { status: i32, stderr: String },
    Parse(String),
}
```

| Helper | Use |
|--------|-----|
| `bail!("msg")` | `return Err(Validation)` inside `AkResult` fn / `script!` |
| `bail_if!(cond, "msg")` | Conditional bail |
| `bail_err("msg")?` | Expression-style (prelude function) |
| `ensure!(cond, "msg")` | Same as bail but validation message |
| `ensure_msg!(cond, "fmt {}", arg)` | `Parse` error with `format!` |

## Macro catalog

### I/O

| Macro | Returns | Notes |
|-------|---------|-------|
| `say!(...)` | `()` | Like `println!` |
| `ask!(prompt)` | `String` | Trimmed stdin line |

### Control flow

| Macro | Syntax |
|-------|--------|
| `when!` | `when!(cond => { }, else => { })` or without else |
| `unless!` | `unless!(cond => { })` — runs when false |
| `repeat!` | `repeat!(i in 0 => 10, { })` — half-open range |
| `each!` | `each!(item in items, { })` |
| `retry!` | `retry!(3 => expr)?` or `retry!(3 => expr, delay_ms => 200)?` |
| `measure_ms!` | `let (val, ms) = measure_ms!(expr);` |
| `defer!` | `defer!({ cleanup(); });` — runs on scope drop |

### Shell / system

| Macro | Returns | Notes |
|-------|---------|-------|
| `cmd!("cmd")` | `AkResult<String>` | **Fails on non-zero exit** (v2 breaking) |
| `cmd!(shell, "cmd")` | `AkResult<String>` | Explicit shell + `-c` |
| `cmd_ok!(...)` | `AkResult<String>` | **v1 behavior**: stdout only, ignores exit code |
| `cmd_out!` | Same as `cmd!` | Alias |
| `cmd_err!("cmd")` | `AkResult<String>` | stderr on success; fails on non-zero |
| `os!()` | `&'static str` | `std::env::consts::OS` |
| `open!(url)` | `AkResult<()>` | Platform browser opener |
| `month_now!()` / `year_now!()` | `u64` | Approximate; not calendar-perfect |
| `now!()` / `today!()` | `AkResult<String>` | Feature **`time`** only |

### Filesystem

| Macro | Returns |
|-------|---------|
| `file_read!` / `read!` | `AkResult<String>` |
| `read_bytes!` | `AkResult<Vec<u8>>` |
| `lines!` | `AkResult<Vec<String>>` |
| `file_write!` | `AkResult<()>` |
| `file_exists!` | `bool` |
| `file_rm!` | `AkResult<()>` |
| `dir_mk!` / `dir_mkp!` | `AkResult<()>` |
| `dir_rm!` / `dir_wipe!` | `AkResult<()>` |
| `path_join!("a", "b")` | `PathBuf` |

### Environment

**Do not use `env!`** — conflicts with `std::env!` (compile-time). Use:

| Macro | Returns |
|-------|---------|
| `env_get!("KEY")` | `AkResult<String>` |
| `env_or!("KEY", "default")` | `String` |
| `env_set!("KEY", "val")` | `AkResult<()>` |

### Strings / numbers

| Macro | Returns |
|-------|---------|
| `str_make!(x)` | `String` |
| `upper!` / `lower!` / `trim!` | `String` |
| `split!(s, pat)` | `Vec<String>` |
| `parse_int!` / `parse_float!` | `AkResult<i64>` / `AkResult<f64>` |
| `pos!(n)` / `neg!(n)` | `AkResult<usize>` / `AkResult<isize>` |

### Text (core + feature `text`)

| Macro | Returns | Feature |
|-------|---------|---------|
| `contains!(hay, needle)` | `bool` | core |
| `replace!(s, from, to)` | `String` | core (first match) |
| `replace_all!(s, from, to)` | `String` | core |
| `tpl!("Hi {{k}}", k = v)` | `AkResult<String>` | core |
| `regex_match!` / `regex_find!` | `AkResult<...>` | `text` |
| `regex_replace!` / `regex_split!` | `AkResult<...>` | `text` |

### Async (feature `async`)

Use `script_async!` instead of `script!`. `sleep_async!(ms)` already awaits internally—do not write `.await` after it.

| Macro | Notes |
|-------|-------|
| `script_async!` | Tokio main + `AkResult` |
| `sleep_async!(ms)` | No trailing `.await` |
| `spawn! { }` | Background task |
| `file_read_async!` / `file_write_async!` / `cmd_async!` | Use `.await?` |

### JSON (feature `serde`)

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Config { name: String }

let cfg: Config = json_read!("config.json", Config)?;
json_write!("out.json", &cfg)?;
```

Requires `Serialize` / `Deserialize` on the type.

## Architecture (for contributors)

```text
src/lib.rs          → pub core, error, prelude; re-exports
src/core.rs         → all fallible logic (test here)
src/macros/*.rs     → thin macro_rules! → core
src/error.rs        → AkError, AkResult
```

- New fallible behavior: implement in `core.rs`, expose via `#[macro_export]` in `src/macros/`.
- Register new macro modules in `src/macros/mod.rs`.
- Gate optional deps with `#[cfg(feature = "...")]` on modules and `Cargo.toml` features.

## v1 → v2 migration

| v1 | v2 |
|----|-----|
| `fn main() -> Box<dyn Error>` | `script! { }` |
| `cmd!` ignores exit | `cmd!` strict; use `cmd_ok!` for old behavior |
| No `file_read` / env | `file_read!`, `env_get!`, etc. |

## Gotchas

1. **`bail!` uses `return`** — only inside functions returning `AkResult` (including `script!` closure).
2. **`ensure!` / `bail!` need `AkResult` return type** in the enclosing fn—not `()` or `Result<(), Box<dyn Error>>`.
3. **`cmd!` on Windows** uses `cmd` with `-c` pattern like Unix `sh -c`; test cross-platform or use `cfg`.
4. **`prelude` does not import macros** — always `use open_macros::*` for macro names in scope.
5. **Async only with feature `async`**; enable in `Cargo.toml`. No HTTP yet (planned 2.2+).
6. **`sleep_async!` is not a Future** — never `sleep_async!(n).await`.
7. **Clippy `all = deny`** in crate—new code must be warning-free.

## Examples in repo

- `examples/hello_script.rs` — FS + `unless!`
- `examples/enterprise_job.rs` — `cmd!`, `when!`, `bail!`
- `examples/async_fetch.rs` — `script_async!`
- `tests/text.rs`, `tests/async.rs` — text/async macros
- `tests/macro_api.rs`, `tests/fs_env.rs`

## Anti-patterns

```rust
// BAD: manual main boilerplate when script! fits
fn main() -> Result<(), Box<dyn std::error::Error>> { ... }

// BAD: std env! name collision risk in docs
let x = env!("HOME");  // compile-time, not runtime

// BAD: ignoring cmd failure in v2
let _ = cmd!("might_fail");  // use ? or handle Err

// GOOD
script! {
    let home = env_get!("HOME")?;
    let out = cmd!("echo ok")?;
}
```

## Extending the library

When user asks for new capabilities:

1. Prefer **macro_rules!** + **core function** (no proc-macro unless requested).
2. Optional heavy deps → new **feature flag** + `cfg` module (like `serde`, `time`).
3. Update `README.md`, `CHANGELOG.md`, tests, and this skill if API changes.
4. Bump semver: breaking shell/FS behavior = major; new macros = minor.

## Additional reference

- Human docs: `README.md`, `CHANGELOG.md` at repo root
- Site: https://openmacros.vercel.app
