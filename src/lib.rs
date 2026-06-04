#![doc = include_str!("../README.md")]

pub mod core;
pub mod error;
mod macros;

pub use core::ShellOutput;
pub use error::{AkError, AkResult};

/// Returns a validation error for use with `?`.
pub fn bail_err(msg: &'static str) -> AkResult<()> {
    Err(AkError::Validation(msg))
}

/// Returns a validation error when `condition` is true.
pub fn bail_if_fn(condition: bool, msg: &'static str) -> AkResult<()> {
    if condition {
        Err(AkError::Validation(msg))
    } else {
        Ok(())
    }
}

pub mod prelude {
    pub use crate::{bail_err, bail_if_fn, AkError, AkResult, ShellOutput};
}
