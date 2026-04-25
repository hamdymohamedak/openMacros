# Changelog

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
