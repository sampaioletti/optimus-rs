use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptimusError {
    #[error("Argument Provided Not Prime")]
    NotPrime,
    #[error("Cannoot calculate Mod Inverse for Argument Provided")]
    NoModInverse,
}
