use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum XkpasswdError {
    #[error("words count must be a positive integer")]
    InvalidWordsCount,

    #[error("min word length must be at least 4")]
    MinWordLengthTooSmall,

    #[error("max word length must be at most 10")]
    MaxWordLengthTooLarge,

    #[error("invalid adaptive padding number")]
    InvalidAdaptivePadding,

    #[error("invalid word transform")]
    InvalidTransform,

    #[error("adaptive length is required for adaptive padding strategy")]
    AdaptiveLengthRequired,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file error: {0}")]
    InvalidFile(String),

    #[error("config error at '{field}': {message}")]
    InvalidConfig { field: String, message: String },

    #[error("config ignored")]
    Ignored,
}

impl From<XkpasswdError> for String {
    fn from(err: XkpasswdError) -> String {
        err.to_string()
    }
}
