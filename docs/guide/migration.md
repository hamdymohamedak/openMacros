# Migration from 0.x

Version `1.x` is intentionally breaking.

## What changed

- Legacy macro names were removed.
- API is normalized around consistent naming and signatures.
- Fallible operations now use the crate error model (`AkResult` / `AkError`).
- Internal implementation moved to modular architecture (`src/macros`, `src/core`, `src/error`).

## Migration examples

- `akp!` -> `say!`
- `input_prompt!` -> `ask!`
- `terminal!` -> `cmd!`
- `remove_file!` -> `file_rm!`
- `remove_all_folders!` -> `dir_wipe!`
