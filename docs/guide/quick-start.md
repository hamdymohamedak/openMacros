# Quick Start

```rust
use ak_macros::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    say!("Welcome to AK-Macros");

    let profile = ask!("Environment (dev/stage/prod): ");
    when!(profile == "prod" => {
        say!("Production mode enabled");
    }, else => {
        say!("Non-production mode");
    });

    repeat!(i in 0 => 3, {
        say!("step {}", i);
    });

    file_write!("build.log", "pipeline started\n")?;
    let output = cmd!("echo ok")?;
    say!("shell output: {}", output);

    Ok(())
}
```

## Macro Categories

- IO: `say!`, `ask!`
- Control: `when!`, `repeat!`, `each!`
- System: `cmd!`, `open!`, `os!`, `month_now!`, `year_now!`
- Filesystem: `file_write!`, `file_rm!`, `dir_rm!`, `dir_wipe!`
- Value helpers: `str_make!`, `upper!`, `lower!`, `pos!`, `neg!`
