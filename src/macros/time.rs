#[macro_export]
macro_rules! now {
    () => {{
        $crate::core::now_rfc3339()
    }};
}

#[macro_export]
macro_rules! today {
    () => {{
        $crate::core::today_iso()
    }};
}
