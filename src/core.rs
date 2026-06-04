use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{AkError, AkResult};

pub struct ShellOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

pub struct DeferGuard<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> Drop for DeferGuard<F> {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            action();
        }
    }
}

pub fn defer<F: FnOnce()>(action: F) -> DeferGuard<F> {
    DeferGuard {
        action: Some(action),
    }
}

pub fn prompt(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_owned()
}

fn shell_command(shell: &str, command: &str) -> std::io::Result<std::process::Output> {
    Command::new(shell).arg("-c").arg(command).output()
}

fn output_to_strings(output: &std::process::Output) -> (String, String, i32) {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let status = output.status.code().unwrap_or(-1);
    (stdout, stderr, status)
}

pub fn run_shell(shell: &str, command: &str) -> AkResult<String> {
    let output = shell_command(shell, command)?;
    let (stdout, stderr, status) = output_to_strings(&output);
    if output.status.success() {
        Ok(stdout)
    } else {
        Err(AkError::Command { status, stderr })
    }
}

pub fn run_shell_stdout_only(shell: &str, command: &str) -> AkResult<String> {
    let output = shell_command(shell, command)?;
    let (stdout, _, _) = output_to_strings(&output);
    Ok(stdout)
}

pub fn run_shell_full(shell: &str, command: &str) -> AkResult<ShellOutput> {
    let output = shell_command(shell, command)?;
    let (stdout, stderr, status) = output_to_strings(&output);
    Ok(ShellOutput {
        stdout,
        stderr,
        status,
    })
}

pub fn run_default_shell(command: &str) -> AkResult<String> {
    run_shell(default_shell(), command)
}

pub fn run_default_shell_stdout_only(command: &str) -> AkResult<String> {
    run_shell_stdout_only(default_shell(), command)
}

pub fn run_default_shell_stderr(command: &str) -> AkResult<String> {
    let output = shell_command(default_shell(), command)?;
    let (_, stderr, status) = output_to_strings(&output);
    if output.status.success() {
        Ok(stderr)
    } else {
        Err(AkError::Command { status, stderr })
    }
}

fn default_shell() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "cmd"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "sh"
    }
}

pub fn read_file(path: impl AsRef<Path>) -> AkResult<String> {
    let bytes = fs::read(path)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn read_file_bytes(path: impl AsRef<Path>) -> AkResult<Vec<u8>> {
    Ok(fs::read(path)?)
}

pub fn read_file_lines(path: impl AsRef<Path>) -> AkResult<Vec<String>> {
    let content = read_file(path)?;
    Ok(content.lines().map(str::to_owned).collect())
}

pub fn path_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

pub fn create_dir(path: impl AsRef<Path>) -> AkResult<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn create_dir_all(path: impl AsRef<Path>) -> AkResult<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn path_join(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path
}

pub fn env_get(key: &str) -> AkResult<String> {
    env::var(key).map_err(|_| AkError::Validation("environment variable not set"))
}

pub fn env_get_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_owned())
}

pub fn env_set(key: &str, value: &str) -> AkResult<()> {
    env::set_var(key, value);
    Ok(())
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

pub fn parse_i64(value: &str) -> AkResult<i64> {
    value
        .trim()
        .parse::<i64>()
        .map_err(|_| AkError::Parse(format!("invalid integer: {value}")))
}

pub fn parse_f64(value: &str) -> AkResult<f64> {
    value
        .trim()
        .parse::<f64>()
        .map_err(|_| AkError::Parse(format!("invalid float: {value}")))
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

#[cfg(feature = "time")]
pub fn now_rfc3339() -> AkResult<String> {
    Ok(chrono::Utc::now().to_rfc3339())
}

#[cfg(feature = "time")]
pub fn today_iso() -> AkResult<String> {
    Ok(chrono::Utc::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string())
}
