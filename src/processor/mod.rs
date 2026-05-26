use crate::Record;

/// Core trait for processing log records.
///
/// A `Processor` is responsible for mutating a [`Record`] before it is
/// passed to handlers or formatters in the logging pipeline.
///
/// It can enrich the record with additional contextual metadata such as
/// request identifiers, hostname, environment information, or runtime
/// metrics. Processors are executed in registration order.
pub trait Processor: Send + Sync {
    /// Mutates the given `Record` by attaching additional metadata.
    ///
    /// **Parameters**
    /// - `record` - The log record that will be modified in place.
    fn process(&self, record: &mut Record);

    /// Returns the execution order of this processor.
    ///
    /// Lower values are executed first, allowing fine-grained control
    /// over processor ordering within the pipeline.
    fn order(&self) -> usize {
        0
    }
}

// Internal modules for processor implementations
mod context;

// Re-export all processor implementations
pub use context::*;
