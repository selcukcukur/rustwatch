use thiserror::Error;

/// Core error type for the logging system.
///
/// Represents all possible failures that can occur in level parsing
/// and core logging infrastructure.
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid log level name was provided.
    #[error("invalid log level: {0}")]
    InvalidLevel(String),

    /// Invalid numeric severity value was provided.
    #[error("invalid severity value: {0}")]
    InvalidSeverity(i8),
}