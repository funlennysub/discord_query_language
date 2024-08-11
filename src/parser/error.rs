use nom::error::ErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse query: {0:?}")]
    Failed(ErrorKind)
}