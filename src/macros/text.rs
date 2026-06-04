#[macro_export]
macro_rules! contains {
    ($haystack:expr, $needle:expr) => {{
        ($haystack).contains($needle)
    }};
}

#[macro_export]
macro_rules! replace {
    ($value:expr, $from:expr, $to:expr) => {{
        $crate::core::text::replace_first($value, $from, $to)
    }};
}

#[macro_export]
macro_rules! replace_all {
    ($value:expr, $from:expr, $to:expr) => {{
        ($value).replace($from, $to)
    }};
}

#[macro_export]
macro_rules! tpl {
    ($template:expr $(, $key:ident = $value:expr)*) => {{
        let __pairs: Vec<(&str, String)> = vec![$( (stringify!($key), format!("{}", $value)) ),*];
        $crate::core::text::render_template($template, &__pairs)
    }};
}

#[cfg(feature = "text")]
#[macro_export]
macro_rules! regex_match {
    ($value:expr, $pattern:expr) => {{
        $crate::core::text::regex_match($value, $pattern)
    }};
}

#[cfg(feature = "text")]
#[macro_export]
macro_rules! regex_find {
    ($value:expr, $pattern:expr) => {{
        $crate::core::text::regex_find($value, $pattern)
    }};
}

#[cfg(feature = "text")]
#[macro_export]
macro_rules! regex_replace {
    ($value:expr, $pattern:expr, $replacement:expr) => {{
        $crate::core::text::regex_replace($value, $pattern, $replacement)
    }};
}

#[cfg(feature = "text")]
#[macro_export]
macro_rules! regex_split {
    ($value:expr, $pattern:expr) => {{
        $crate::core::text::regex_split($value, $pattern)
    }};
}
