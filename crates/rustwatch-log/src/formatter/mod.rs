use crate::Record;

/// Trait for all formatter implementations
pub trait Formatter {
  fn format(&self, record: &Record) -> String;
}

// Internal modules for formatter implementations
mod line;
mod json;

// Re-export all formatter implementations for direct use via `formatters::`
pub use line::*;
pub use json::*;
