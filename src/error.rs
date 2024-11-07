use thiserror::Error;

#[derive(Debug, Error)]
pub enum StripsterError {
    #[error("Command execution failed: {0}")]
    CommandError(String),

    #[error("Output parsing failed: {0}")]
    OutputParsingError(String),

    #[error("Serialization or deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("UTF-8 conversion failed: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("I/O operation failed: {0}")]
    IoError(#[from] std::io::Error),
}
