# Macros Reference

This page includes usage examples for every macro in the library.

## IO

### `say!(...)`

Prints to stdout, like `println!`.

```rust
use ak_macros::*;

say!("Hello {}", "AK");
```

### `ask!(prompt)`

Prints a prompt and returns trimmed user input as `String`.

```rust
use ak_macros::*;

let name = ask!("Your name: ");
say!("Hi {}", name);
```

## Control

### `when!(cond => { ... })`

Runs the block when condition is true.

```rust
use ak_macros::*;

let is_prod = true;
when!(is_prod => {
    say!("production mode");
});
```

### `when!(cond => { ... }, else => { ... })`

Conditional branch with explicit else block.

```rust
use ak_macros::*;

let env = "dev";
when!(env == "prod" => {
    say!("strict checks");
}, else => {
    say!("standard checks");
});
```

### `repeat!(i in start => end, { ... })`

Iterates from `start..end`.

```rust
use ak_macros::*;

repeat!(i in 0 => 3, {
    say!("step {}", i);
});
```

### `each!(item in items, { ... })`

Iterates through iterable items.

```rust
use ak_macros::*;

let services = ["api", "worker", "cron"];
each!(service in services, {
    say!("deploy {}", service);
});
```

## System

### `cmd!("shell command")`

Runs command in default shell and returns `AkResult<String>`.

```rust
use ak_macros::*;

let output = cmd!("echo ready")?;
say!("output: {}", output);
# Ok::<(), ak_macros::AkError>(())
```

### `cmd!("shell", "command")`

Runs command using explicit shell.

```rust
use ak_macros::*;

let output = cmd!("sh", "echo explicit-shell")?;
say!("{}", output);
# Ok::<(), ak_macros::AkError>(())
```

### `open!("https://...")`

Opens URL in default OS browser.

```rust
use ak_macros::*;

open!("https://ak-macros.vercel.app")?;
# Ok::<(), ak_macros::AkError>(())
```

### `os!()`

Returns current OS as `&'static str`.

```rust
use ak_macros::*;

let current_os = os!();
say!("running on {}", current_os);
```

### `month_now!()`, `year_now!()`

Time helpers derived from system epoch time.

```rust
use ak_macros::*;

let m = month_now!();
let y = year_now!();
say!("date info: {}/{}", m, y);
```

## Filesystem

### `file_write!(path, content)`

Writes content to file, returns `AkResult<()>`.

```rust
use ak_macros::*;

file_write!("report.txt", "build=ok\n")?;
# Ok::<(), ak_macros::AkError>(())
```

### `file_rm!(path)`

Removes file, returns `AkResult<()>`.

```rust
use ak_macros::*;

file_write!("temp.txt", "tmp")?;
file_rm!("temp.txt")?;
# Ok::<(), ak_macros::AkError>(())
```

### `dir_rm!(path)`

Removes empty directory, returns `AkResult<()>`.

```rust
use ak_macros::*;

std::fs::create_dir("empty_dir")?;
dir_rm!("empty_dir")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

### `dir_wipe!(path)`

Removes directory recursively, returns `AkResult<()>`.

```rust
use ak_macros::*;

std::fs::create_dir_all("tmp/a/b")?;
file_write!("tmp/a/b/file.txt", "x")?;
dir_wipe!("tmp")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Helpers

### `str_make!(value)`

Creates `String`.

```rust
use ak_macros::*;

let service = str_make!("billing-service");
say!("{}", service);
```

### `upper!(value)`, `lower!(value)`

Returns transformed `String`.

```rust
use ak_macros::*;

let up = upper!("ak macros");
let down = lower!("AK MACROS");
say!("{} / {}", up, down);
```

### `pos!(value)`, `neg!(value)`

Validation helpers returning `AkResult<usize>` and `AkResult<isize>`.

```rust
use ak_macros::*;

let workers = pos!(8)?;
let offset = neg!(-2)?;
say!("workers={} offset={}", workers, offset);
# Ok::<(), ak_macros::AkError>(())
```
