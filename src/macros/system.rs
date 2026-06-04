#[macro_export]
macro_rules! cmd {
    ($command:expr) => {{
        $crate::core::run_default_shell($command)
    }};
    ($shell:expr, $command:expr) => {{
        $crate::core::run_shell($shell, $command)
    }};
}

#[macro_export]
macro_rules! cmd_ok {
    ($command:expr) => {{
        $crate::core::run_default_shell_stdout_only($command)
    }};
    ($shell:expr, $command:expr) => {{
        $crate::core::run_shell_stdout_only($shell, $command)
    }};
}

#[macro_export]
macro_rules! cmd_out {
    ($command:expr) => {{
        $crate::core::run_default_shell($command)
    }};
    ($shell:expr, $command:expr) => {{
        $crate::core::run_shell($shell, $command)
    }};
}

#[macro_export]
macro_rules! cmd_err {
    ($command:expr) => {{
        $crate::core::run_default_shell_stderr($command)
    }};
}

#[macro_export]
macro_rules! os {
    () => {
        std::env::consts::OS
    };
}

#[macro_export]
macro_rules! year_now {
    () => {{
        $crate::core::year_now()
    }};
}

#[macro_export]
macro_rules! month_now {
    () => {{
        $crate::core::month_now()
    }};
}

#[macro_export]
macro_rules! open {
    ($url:expr) => {{
        $crate::core::open_url($url)
    }};
}
