use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptimusError {
    #[error("Argument Provided Not Prime")]
    NotPrime,
}
