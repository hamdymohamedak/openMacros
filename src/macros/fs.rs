#[macro_export]
macro_rules! file_write {
    ($path:expr, $content:expr) => {{
        $crate::core::write_file($path, $content)
    }};
}

#[macro_export]
macro_rules! file_rm {
    ($path:expr) => {{
        $crate::core::remove_file($path)
    }};
}

#[macro_export]
macro_rules! dir_rm {
    ($path:expr) => {{
        $crate::core::remove_dir($path)
    }};
}

#[macro_export]
macro_rules! dir_wipe {
    ($path:expr) => {{
        $crate::core::remove_dir_all($path)
    }};
}
