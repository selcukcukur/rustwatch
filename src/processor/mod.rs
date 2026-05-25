use crate::Record;

/// Expose Processor trait at crate root
pub trait Processor {
    fn process(&self, record: &mut Record);

    fn severity(&self) -> usize {
        0
    }
}

// Internal modules for processor implementations
mod context;

// Re-export all processor implementations
pub use context::*;
