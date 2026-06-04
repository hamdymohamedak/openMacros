#[macro_export]
macro_rules! script {
    ($($body:tt)*) => {
        fn main() {
            if let Err(err) = (|| -> $crate::AkResult<()> {
                $($body)*
                Ok(())
            })() {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($msg:expr) => {
        return Err($crate::AkError::Validation($msg))
    };
}

#[macro_export]
macro_rules! bail_if {
    ($cond:expr, $msg:expr) => {
        if $cond {
            bail!($msg);
        }
    };
}
