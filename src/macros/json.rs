#[macro_export]
macro_rules! json_read {
    ($path:expr, $ty:ty) => {{
        let __content = $crate::core::read_file($path)?;
        serde_json::from_str::<$ty>(&__content).map_err($crate::AkError::from)
    }};
}

#[macro_export]
macro_rules! json_write {
    ($path:expr, $value:expr) => {{
        let __content = serde_json::to_string_pretty($value).map_err($crate::AkError::from)?;
        $crate::core::write_file($path, __content.as_bytes())
    }};
}
