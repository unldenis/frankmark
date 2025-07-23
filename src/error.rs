use std::error::Error;
use thiserror::Error;
use askama;

// Custom error type for Frankmark using thiserror
#[derive(Error, Debug)]
pub enum FrankmarkError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Directory error: {0}")]
    #[allow(dead_code)]
    DirectoryError(String),
    
    #[error("File error: {0}")]
    #[allow(dead_code)]
    FileError(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Inner error: {0}")]
    InnerError(Box<dyn Error>),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] toml::de::Error),
}


impl From<askama::Error> for FrankmarkError {
    fn from(err: askama::Error) -> Self {
        FrankmarkError::TemplateError(err.to_string())
    }
}

pub type FrankmarkResult<T> = std::result::Result<T, FrankmarkError>; 