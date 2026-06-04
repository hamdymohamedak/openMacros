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
macro_rules! trim {
    ($value:expr) => {{
        ($value).trim().to_owned()
    }};
}

#[macro_export]
macro_rules! split {
    ($value:expr, $pattern:expr) => {{
        ($value)
            .split($pattern)
            .map(str::to_owned)
            .collect::<Vec<String>>()
    }};
}

#[macro_export]
macro_rules! parse_int {
    ($value:expr) => {{
        $crate::core::parse_i64($value)
    }};
}

#[macro_export]
macro_rules! parse_float {
    ($value:expr) => {{
        $crate::core::parse_f64($value)
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
