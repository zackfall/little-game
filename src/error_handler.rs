use std::{fmt, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("error reading the config file: {0}")]
    ReadConfigFileError(#[from] io::Error),
    #[error("error parsing the config file: {0}")]
    ParseConfigFileError(#[from] serde_json::Error),
    #[error("error trying to build the ui: {0}")]
    UiBuildError(#[from] UiError),
}

#[derive(Debug, Clone, Error)]
pub struct UiError(String);

impl UiError {
    pub fn new(description: String) -> Self {
        UiError(description)
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Result<T> = std::result::Result<T, GameError>;
