use crate::Record;

/// Expose Handler trait at crate root
pub trait Handler {
    fn log(&mut self, record: &Record) -> bool;

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
