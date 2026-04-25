#![doc = include_str!("../README.md")]

pub mod core;
pub mod error;
mod macros;

pub use error::{AkError, AkResult};
