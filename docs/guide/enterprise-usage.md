# Enterprise Usage

AK-Macros works best as an orchestration layer around business logic.

## Recommended Pattern

1. Keep business logic in regular Rust modules/functions.
2. Use AK macros in command boundaries (CLI, jobs, scripts).
3. Return typed errors from domain code; map to `AkResult` at boundaries.
4. Enforce CI quality gates on every PR.

## Team Conventions

- Treat macro additions as API design changes.
- Document every new macro in `/docs/api/macros.md`.
- Add integration tests under `/tests`.

## CI Checklist

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
