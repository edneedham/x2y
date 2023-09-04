use std::error;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum X2YError {
    #[error("the input was incorrect: `{0}`")]
    InvalidInput(String),
    #[error("file system issue")]
    IO(#[from] io::Error),
    #[error("could not transcode: `{0}`")]
    Transcode(#[from] Box<dyn error::Error>),
}
