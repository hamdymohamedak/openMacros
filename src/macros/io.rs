#[macro_export]
macro_rules! say {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! ask {
    ($prompt:expr) => {{
        $crate::core::prompt($prompt)
    }};
}
