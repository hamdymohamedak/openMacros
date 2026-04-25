#[macro_export]
macro_rules! str_make {
    ($value:expr) => {
        String::from($value)
    };
}

#[macro_export]
macro_rules! upper {
    ($value:expr) => {{
        ($value).to_uppercase()
    }};
}

#[macro_export]
macro_rules! lower {
    ($value:expr) => {{
        ($value).to_lowercase()
    }};
}

#[macro_export]
macro_rules! pos {
    ($value:expr) => {{
        $crate::core::ensure_positive($value)
    }};
}

#[macro_export]
macro_rules! neg {
    ($value:expr) => {{
        $crate::core::ensure_negative($value)
    }};
}
