use crate::Record;

/// Core trait for converting a [`Record`] into its output representation.
///
/// A `Formatter` is stateless and receives a shared reference to the
/// record — it must not mutate it. Formatting is a pure transformation.
///
/// **Custom**
/// ```rust,ignore
/// use rustwatch::{Record, Formatter};
///
/// struct CsvFormatter;
///
/// impl Formatter for CsvFormatter {
///     fn format(&self, record: &Record) -> String {
///         format!(
///             "{},{},{},\"{}\"",
///             record.timestamp().to_rfc3339(),
///             record.level(),
///             record.channel(),
///             record.message().replace('"', "\"\""),
///         )
///     }
/// }
/// ```
pub trait Formatter: Send + Sync {
    /// Renders the record into a string suitable for a handler's destination.
    fn format(&self, record: &Record) -> String;

    /// Optional sort weight for deterministic ordering when multiple
    /// formatters are evaluated in sequence (currently unused by the
    /// built-in pipeline; available for custom orchestration).
    fn order(&self) -> usize { 0 }
}

// Internal modules for formatter implementations
mod json;
mod line;

// Re-export all formatter implementations for direct use via `formatters::`
pub use json::*;
pub use line::*;
