#[macro_export]
macro_rules! env_get {
    ($key:expr) => {{
        $crate::core::env_get($key)
    }};
}

#[macro_export]
macro_rules! env_or {
    ($key:expr, $default:expr) => {{
        $crate::core::env_get_or($key, $default)
    }};
}

#[macro_export]
macro_rules! env_set {
    ($key:expr, $value:expr) => {{
        $crate::core::env_set($key, $value)
    }};
}
