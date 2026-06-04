# Changelog

## [2.0.0] - 2026-06-04

### Added

- `script!` macro for zero-boilerplate `main` with `AkResult` error handling.
- `prelude` module with `AkError`, `AkResult`, `ShellOutput`, `bail_err`, `bail_if_fn`.
- `bail!`, `bail_if!` macros and `ensure_msg!`, `unless!`, `defer!` control helpers.
- Filesystem: `read!`, `read_bytes!`, `lines!`, `file_read!`, `file_exists!`, `dir_mk!`, `dir_mkp!`, `path_join!`.
- Environment: `env_get!`, `env_or!`, `env_set!`.
- String/parsing: `trim!`, `split!`, `parse_int!`, `parse_float!`.
- Shell: `cmd_ok!` (v1 stdout-only), `cmd_out!`, `cmd_err!`.
- Extended `AkError` with `Command` and `Parse` variants.
- Optional features: `serde` (`json_read!`, `json_write!`), `time` (`now!`, `today!`).
- Examples: `hello_script.rs`; `enterprise_job.rs` migrated to `script!`.
- Tests: `tests/fs_env.rs`; expanded `tests/macro_api.rs`.

### Changed

- **Breaking:** `cmd!` now fails when the shell command exits with a non-zero status.
- Version bumped to 2.0.0; README and API contract updated for v2.

## [1.0.0] - 2026-04-25

### Changed

- Full reboot with a new script-like macro API.
- Removed all legacy 0.x macro names and signatures.
- Added `core` helper module to centralize filesystem, shell, time, and validation logic.
- Added modular macro layout under `src/macros/*` for maintainability at scale.
- Added unified error model (`AkError`, `AkResult`) for consistent enterprise integration.
- Standardized error behavior: fallible operations return `Result`.
- Rewrote project documentation to define the v1 API contract.
- Added integration tests for flow, loop, string, and validation macros.
- Added CI pipeline for format, clippy, and tests.
