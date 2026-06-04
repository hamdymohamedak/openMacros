#[macro_export]
macro_rules! read {
    ($path:expr) => {{
        $crate::core::read_file($path)
    }};
}

#[macro_export]
macro_rules! read_bytes {
    ($path:expr) => {{
        $crate::core::read_file_bytes($path)
    }};
}

#[macro_export]
macro_rules! lines {
    ($path:expr) => {{
        $crate::core::read_file_lines($path)
    }};
}

#[macro_export]
macro_rules! file_read {
    ($path:expr) => {{
        $crate::core::read_file($path)
    }};
}

#[macro_export]
macro_rules! file_exists {
    ($path:expr) => {{
        $crate::core::path_exists($path)
    }};
}

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
macro_rules! dir_mk {
    ($path:expr) => {{
        $crate::core::create_dir($path)
    }};
}

#[macro_export]
macro_rules! dir_mkp {
    ($path:expr) => {{
        $crate::core::create_dir_all($path)
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

#[macro_export]
macro_rules! path_join {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        $crate::core::path_join(&[$first $(, $rest)*])
    }};
}
