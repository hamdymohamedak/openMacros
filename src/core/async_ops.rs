use std::path::Path;
use std::time::Duration;

use tokio::process::Command;

use crate::error::{AkError, AkResult};

use super::{default_shell, output_to_strings};

async fn shell_command(shell: &str, command: &str) -> std::io::Result<std::process::Output> {
    Command::new(shell).arg("-c").arg(command).output().await
}

pub async fn sleep_ms(millis: u64) {
    tokio::time::sleep(Duration::from_millis(millis)).await;
}

pub async fn read_file_async(path: impl AsRef<Path>) -> AkResult<String> {
    let bytes = tokio::fs::read(path).await?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub async fn write_file_async(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> AkResult<()> {
    tokio::fs::write(path, content).await?;
    Ok(())
}

pub async fn run_shell_async(shell: &str, command: &str) -> AkResult<String> {
    let output = shell_command(shell, command).await?;
    let (stdout, stderr, status) = output_to_strings(&output);
    if output.status.success() {
        Ok(stdout)
    } else {
        Err(AkError::Command { status, stderr })
    }
}

pub async fn run_default_shell_async(command: &str) -> AkResult<String> {
    run_shell_async(default_shell(), command).await
}

pub fn spawn_task<F>(future: F) -> tokio::task::JoinHandle<F::Output>
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future)
}
