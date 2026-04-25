use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{AkError, AkResult};

pub fn prompt(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_owned()
}

pub fn run_shell(shell: &str, command: &str) -> AkResult<String> {
    let output = Command::new(shell).arg("-c").arg(command).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub fn run_default_shell(command: &str) -> AkResult<String> {
    #[cfg(target_os = "windows")]
    let shell = "cmd";
    #[cfg(not(target_os = "windows"))]
    let shell = "sh";
    run_shell(shell, command)
}

pub fn write_file(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> AkResult<()> {
    fs::write(path, content)?;
    Ok(())
}

pub fn remove_file(path: impl AsRef<Path>) -> AkResult<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn remove_dir(path: impl AsRef<Path>) -> AkResult<()> {
    fs::remove_dir(path)?;
    Ok(())
}

pub fn remove_dir_all(path: impl AsRef<Path>) -> AkResult<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

pub fn month_now() -> u64 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    (seconds % (365 * 24 * 60 * 60)) / (30 * 24 * 60 * 60) + 1
}

pub fn year_now() -> u64 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    1970 + (seconds / (365 * 24 * 60 * 60))
}

pub fn open_url(url: &str) -> AkResult<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(url)
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        Command::new("xdg-open").arg(url).spawn()?;
    }
    Ok(())
}

pub fn ensure_positive(value: i64) -> AkResult<usize> {
    if value < 0 {
        return Err(AkError::Validation("value must be positive"));
    }
    usize::try_from(value).map_err(|_| AkError::Validation("value is out of range"))
}

pub fn ensure_negative(value: i64) -> AkResult<isize> {
    if value > 0 {
        return Err(AkError::Validation("value must be negative"));
    }
    isize::try_from(value).map_err(|_| AkError::Validation("value is out of range"))
}
