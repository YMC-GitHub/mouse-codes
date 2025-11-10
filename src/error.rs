use thiserror::Error;

/// Error type for mouse code parsing and mapping operations
#[derive(Debug, Error, PartialEq)]
pub enum MouseParseError {
    /// Unknown mouse button
    #[error("Unknown mouse button: {0}")]
    UnknownButton(String),

    /// Unknown platform
    #[error("Unknown platform")]
    UnknownPlatform,

    /// Duplicate custom button
    #[error("Duplicate custom button: {0}")]
    DuplicateCustomButton(String),

    /// Invalid button code for platform
    #[error("Invalid button code {0} for platform")]
    InvalidButtonCode(usize),

    /// Empty input string
    #[error("Empty input string")]
    EmptyInput,
}
