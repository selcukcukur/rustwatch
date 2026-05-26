use crate::Record;

/// Core trait for formatting log records.
///
/// A `Formatter` is responsible for converting a structured [`Record`]
/// into its final string representation suitable for output sinks such as
/// stdout, files, or network transports.
///
/// Implementations may produce different output formats such as
/// plain text, JSON, or custom structured encodings.
pub trait Formatter: Send + Sync {
    /// Formats a log `Record` into a string representation.
    ///
    /// **Parameters**
    /// - `record` - The log record containing message, level, timestamp,
    ///   and additional contextual metadata.
    ///
    /// **Returns**
    /// - `String` - The formatted log output as a string.
    fn format(&self, record: &Record) -> String;

    /// Returns the execution order of this formatter.
    ///
    /// Lower values are executed first, allowing deterministic ordering
    /// of formatters within a logger pipeline.
    fn order(&self) -> usize {
        0
    }
}

// Internal modules for formatter implementations
mod json;
mod line;

// Re-export all formatter implementations for direct use via `formatters::`
pub use json::*;
pub use line::*;
