use crate::Record;

/// Core trait that all log handlers must implement.
///
/// A `Handler` is responsible for writing a log record to a destination
/// such as a file, console, socket, or external service.
///
/// Each handler may optionally declare a `severity` weight used for ordering
/// within a logger. Lower values are executed first.
///
/// The `log` method determines whether a record is accepted by the handler.
/// If it returns `true`, the record is considered handled and propagation
/// may stop depending on the logger’s bubbling configuration.
pub trait Handler: Send + Sync {
    /// Processes the given log record.
    ///
    /// **Returns**
    /// - `true`  → record was handled/accepted by this handler
    /// - `false` → handler did not accept the record; next handler may try
    fn log(&mut self, record: &Record) -> bool;

    /// Returns the execution priority of this handler.
    ///
    /// Lower values are executed first, allowing deterministic ordering
    /// of handlers within a logger pipeline.
    fn severity(&self) -> usize {
        0
    }
}

// Internal modules for handler implementations
mod console;
mod file;

// Re-export all handler implementations
pub use console::*;
pub use file::*;
