# openMacros v1

A script-like macro toolkit for Rust, designed with production-grade structure.

openMacros v1 is a full reboot of the project with a clean naming contract, predictable macro signatures, and professional project standards.

## v1 API Contract

This release is intentionally **breaking**. Old macros from `0.x` are removed.

### Naming rules

- Verb-first names for actions: `say!`, `ask!`, `cmd!`, `open!`
- Resource-first names for file system: `file_write!`, `file_rm!`, `dir_rm!`, `dir_wipe!`
- Short utility names for transformations: `upper!`, `lower!`, `str_make!`, `pos!`, `neg!`
- Flow macros use explicit script-like form: `when!`, `repeat!`, `each!`
- Smart flow helpers: `retry!`, `measure_ms!`, `ensure!`

### Signature rules

- Any macro that can fail returns `AkResult<_>`.
- Pure transform/value macros return direct values.
- Conditional/loop macros keep Rust-native block semantics to stay readable and safe.

## Project Structure

```text
openMacros/
  src/
    lib.rs              # public crate entry
    error.rs            # unified error model
    core.rs             # internal helper runtime
    macros/
      control.rs        # when/repeat/each
      io.rs             # say/ask
      fs.rs             # file_* and dir_* macros
      system.rs         # cmd/open/os/date macros
      string.rs         # str/upper/lower/pos/neg
  tests/
    macro_api.rs
  examples/
    enterprise_job.rs
```

## Macro Reference

### Output and input

- `say!(...)` prints like `println!`.
- `ask!(prompt)` returns trimmed `String`.

### Control flow

- `when!(cond => { ... })`
- `when!(cond => { ... }, else => { ... })`
- `repeat!(i in 0 => 10, { ... })`
- `each!(item in items, { ... })`
- `retry!(3 => cmd!("echo ok"))?`
- `retry!(3 => cmd!("echo ok"), delay_ms => 200)?`
- `let (value, elapsed_ms) = measure_ms!(cmd!("echo ok"));`
- `ensure!(workers > 0, "workers must be positive");`

### Shell and system

- `cmd!("ls")` runs with default shell (`sh` on Unix, `cmd` on Windows).
- `cmd!("sh", "ls -la")` runs with explicit shell.
- `os!()` returns current OS.
- `open!("https://example.com")` opens URL in default browser.
- `month_now!()`, `year_now!()` return date values from system time.

### Filesystem

- `file_write!("/tmp/a.txt", "hello")`
- `file_rm!("/tmp/a.txt")`
- `dir_rm!("/tmp/empty")`
- `dir_wipe!("/tmp/non-empty")`

### String and number helpers

- `str_make!("hello") -> String`
- `upper!("hello") -> String`
- `lower!("HELLO") -> String`
- `pos!(5) -> AkResult<usize>`
- `neg!(-3) -> AkResult<isize>`

## Quick Start

```rust
use open_macros::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    say!("Welcome to openMacros v1");

    let lang = ask!("Favorite language: ");
    when!(lang == "rust" => {
        say!("Great choice.");
    }, else => {
        say!("Give Rust a shot.");
    });

    repeat!(i in 0 => 3, {
        say!("tick {}", i);
    });

    file_write!("example.txt", "hello from v1")?;
    let out = cmd!("echo done")?;
    say!("command output: {}", out);
    Ok(())
}
```

## Advanced Usage

- Use `examples/enterprise_job.rs` as a baseline for automation jobs in large repos.
- Keep business rules in normal Rust functions; use openMacros for orchestration and readability.
- Enforce CI quality gates (`fmt`, `clippy`, `test`) on every pull request.

## Web Documentation (VitePress)

```bash
cd ../AK-Macros-docs
npm install
npm run docs:dev
```

Build static docs:

```bash
cd ../AK-Macros-docs
npm run docs:build
```

## Migration from 0.x

- No compatibility layer is provided in v1.
- Replace old macro names directly with the new contract above.
- Adopt `Result` handling for operations that can fail.

## Quality

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`

## License

MIT
