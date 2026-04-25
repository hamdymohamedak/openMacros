# Error Model

AK-Macros standardizes fallible operations around:

- `AkError`
- `AkResult<T> = Result<T, AkError>`

## `AkError` variants

- `AkError::Io(std::io::Error)`
- `AkError::Validation(&'static str)`

## Why this matters

- Unified handling in applications
- Consistent behavior across macro categories
- Better integration with enterprise logging and observability layers

## Example

```rust
use ak_macros::*;

fn run() -> AkResult<()> {
    file_write!("audit.log", "started\n")?;
    let port = pos!(8080)?;
    say!("port {}", port);
    Ok(())
}
```
