use crate::Record;

/// Core trait for enriching a [`Record`] before it reaches any handler.
///
/// Processors run in registration order after the record is created and
/// before it is passed to the handler chain. They mutate the record
/// **in place** — typically adding keys to `record.context`.
///
/// **Safety**
///
/// All processors must be `Send + Sync` because the logger stores them
/// behind an `Arc<Mutex<_>>`.
///
/// **Custom**
/// ```rust,ignore
/// use rustwatch::{Record, Processor};
/// use serde_json::json;
///
/// struct RequestIdProcessor { id: String }
///
/// impl Processor for RequestIdProcessor {
///     fn process(&self, record: &mut Record) {
///         record.set_context_value("request_id", json!(self.id));
///     }
/// }
/// ```
pub trait Processor: Send + Sync {
    /// Mutates `record` in place to attach additional metadata.
    fn process(&self, record: &mut Record);

    /// Execution priority. Lower values run first. Defaults to `0`.
    fn order(&self) -> usize { 0 }
}

// Internal modules for processor implementations
mod context;

// Re-export all processor implementations
pub use context::*;
