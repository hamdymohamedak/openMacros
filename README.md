# AK-Macros

[![Crates.io](https://img.shields.io/crates/v/ak_macros.svg)](https://crates.io/crates/ak_macros)
[![Documentation](https://docs.rs/ak_macros/badge.svg)](https://docs.rs/ak_macros)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)

**A declarative macro library for Rust** that accelerates development by providing ergonomic, compile-time code generation for common tasks—from I/O and control flow to system introspection and shell integration.

---

## Overview

AK-Macros is an **open-source** Rust crate that exposes a curated set of procedural-style macros for everyday development. By moving repetitive patterns into macros, the library reduces boilerplate, improves readability, and keeps your codebase expressive without sacrificing Rust’s safety or performance.

### Why macros?

- **Metaprogramming** — Generate code at compile time from concise patterns.
- **Domain-specific ergonomics** — Terse, intent-revealing syntax for I/O, conditionals, and loops.
- **Consistency** — Standardized patterns for printing, prompts, file operations, and shell commands.
- **Zero-cost abstraction** — Macros expand to straightforward Rust; no runtime overhead beyond what you’d write by hand.

---

## Features

| Area | Capabilities |
|------|--------------|
| **I/O & CLI** | `akp!`, `input_prompt!`, `terminal!` — print, prompt, and run shell commands |
| **Control flow** | `if_cond!`, `use_loop!`, `use_loopAt!` — conditional and iterative logic |
| **Filesystem** | `remove_folder!`, `remove_file!`, `remove_all_folders!`, `use_createFile!` |
| **Strings & numbers** | `set_String!`, `use_upper_case!`, `use_lower_case!`, `Positive_number!`, `Negative_number!` |
| **System** | `this_OS!`, `this_month!`, `this_year!`, `Ram_size!`, `Get_CPU!` (Linux) |
| **Integration** | `open_Web!` — open URLs in the default browser (cross-platform) |

**Optional features** (require adding dependencies to your `Cargo.toml`):

- **`use_rand!`** — random number generation (`rand`)
- **`use_fetch!`** — HTTP requests (`reqwest`, `tokio`, `serde`, `serde_json`)
- **`use_zip!`** — ZIP extraction (`zip-extract`)

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ak_macros = "0.2"
```

For optional macros, uncomment or add in `Cargo.toml` as needed:

```toml
# rand = "0.8"        # for use_rand!
# reqwest = { version = "0.12", features = ["json"] }
# tokio = { version = "1", features = ["full"] }
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"
# zip-extract = "0.1"
```

---

## Quick start

```rust
use ak_macros::*;

fn main() {
    let name = "AK-Macros";
    akp!("Project: {}", name);
}
```

### Minimal interactive example

```rust
use ak_macros::*;

fn main() {
    akp!("Hello, world!");

    let name = input_prompt!("Enter the name of your favorite crate: ");

    if_cond!(
        name,
        name == "ak-macros",
        akp!("AK-Macros makes Rust feel simple."),
        akp!("Give AK-Macros a try—you might love it.")
    );
}
```

### Running shell commands

```rust
use ak_macros::*;

fn main() {
    // On Unix-like systems use "sh"; on Windows use "cmd"
    let result = terminal!("sh", "ls");
    akp!("{}", result);
    // e.g.: Cargo.lock  Cargo.toml  README.md  src  target
}
```

---

## API reference (summary)

| Macro | Purpose |
|-------|--------|
| `akp!(...)` | `println!` shorthand |
| `input_prompt!(prompt)` | Read a line from stdin after printing `prompt` |
| `terminal!(shell, command)` | Run `command` via `shell`, return stdout as `String` |
| `if_cond!(var, cond, then_branch, else_branch)` | If/else; variants for single-branch and multiple conditions |
| `use_loop!(run, start, end, var, body)` | `for var in start..end { body }` with optional execution flag |
| `use_loopAt!(array, var, body)` | Iterate over an array with a named variable |
| `remove_folder!(path)` | Remove a directory (non-recursive) |
| `remove_file!(path)` | Remove a file |
| `remove_all_folders!(path)` | Remove a directory and its contents recursively |
| `use_createFile!(name, path, content)` | Create a file with given content |
| `set_String!(literal)` | `String::from(literal)` |
| `use_upper_case!(ident)` / `use_lower_case!(ident)` | In-place string case conversion |
| `Positive_number!(n)` / `Negative_number!(n)` | Validate and use positive/negative numbers (panic on wrong sign) |
| `this_OS!()` | Current OS identifier |
| `this_month!()` / `this_year!()` | Current month/year (from system time) |
| `open_Web!(url)` | Open `url` in the default browser |
| `Ram_size!()` | Total RAM in GB (Linux `/proc/meminfo`) |
| `Get_CPU!()` | CPU model string (Linux `/proc/cpuinfo`) |
| `use_rand!(var, type?, block)` | Generate random value and run block (requires `rand`) |
| `use_fetch!(url, METHOD)` | Async HTTP request (requires `reqwest`, `tokio`, etc.) |
| `use_zip!(zip_path, extract_path)` | Extract a ZIP file (requires `zip-extract`) |

Full doc comments and examples are available in the crate docs and source.

---

## Rust code vs AK-Macros

The following screenshots illustrate the same logic expressed with plain Rust versus AK-Macros.

| Plain Rust | With AK-Macros |
|------------|----------------|
| ![Rust code comparison](target/IMG_20240424_064108_421.png) | ![AK-Macros comparison](target/IMG_20240424_064108_839.png) |

![Example](target/Example.png)

---

## Contributing

AK-Macros is **open source** and we welcome contributions.

### How to contribute

1. **Fork** the [repository](https://github.com/hamdymohamedak/AK-macro/).
2. **Clone** your fork and create a branch (`git checkout -b feature/your-feature`).
3. **Implement** your change (new macro, fix, or docs) and add or update tests/examples.
4. **Format & lint**: `cargo fmt` and `cargo clippy`.
5. **Commit** with clear messages and **open a pull request** against `main`.

### Ideas for contributions

- New macros for common patterns (e.g. retries, timeouts, structured logging).
- Cross-platform improvements for `Ram_size!` and `Get_CPU!` (e.g. macOS/Windows).
- Better error handling or configurable behavior (e.g. no `panic!` where appropriate).
- Documentation, examples, and doc-tests.
- Optional feature flags to pull in optional dependencies only when needed.

By contributing, you agree that your work may be distributed under the project’s MIT license.

---

## Resources

- **Crate**: [crates.io/crates/ak_macros](https://crates.io/crates/ak_macros)
- **Docs**: [docs.rs/ak_macros](https://docs.rs/ak_macros)
- **Repository**: [github.com/hamdymohamedak/AK-macro](https://github.com/hamdymohamedak/AK-macro/)
- **Homepage**: [ak-macros.vercel.app](https://ak-macros.vercel.app)

---

## License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.
