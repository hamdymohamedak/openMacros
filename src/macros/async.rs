#[macro_export]
macro_rules! script_async {
    ($($body:tt)*) => {
        #[tokio::main]
        async fn main() {
            if let Err(err) = async_main().await {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }

        async fn async_main() -> $crate::AkResult<()> {
            $($body)*
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! sleep_async {
    ($millis:expr) => {{
        $crate::core::async_ops::sleep_ms($millis).await
    }};
}

#[macro_export]
macro_rules! spawn {
    ($($body:tt)*) => {{
        $crate::core::async_ops::spawn_task(async move {
            $($body)*
        })
    }};
}

#[macro_export]
macro_rules! file_read_async {
    ($path:expr) => {{
        $crate::core::async_ops::read_file_async($path)
    }};
}

#[macro_export]
macro_rules! file_write_async {
    ($path:expr, $content:expr) => {{
        $crate::core::async_ops::write_file_async($path, $content)
    }};
}

#[macro_export]
macro_rules! cmd_async {
    ($command:expr) => {{
        $crate::core::async_ops::run_default_shell_async($command)
    }};
    ($shell:expr, $command:expr) => {{
        $crate::core::async_ops::run_shell_async($shell, $command)
    }};
}
