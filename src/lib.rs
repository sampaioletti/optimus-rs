#![deny(clippy::pedantic)]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod optimus;

pub use crate::error::OptimusError;
pub use crate::optimus::Optimus;
