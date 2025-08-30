use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parsing error")]
    Parse(#[from] serde_json::Error),
    #[error("Filesystem access error")]
    Io(#[from] io::Error),
    #[error("Other error")]
    Other(&'static str),
}
