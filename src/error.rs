use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<std::result::Result<T, AppError>, UnexpectedError>;

#[derive(Error, Debug)]
pub enum UnexpectedError {
    #[error("")]
    IOError {
        #[from]
        source: io::Error,
    },
    #[error("")]
    FormatError,

    #[error("")]
    FileNotExistError,
}

#[derive(Error, Debug)]
pub enum AppError {}
