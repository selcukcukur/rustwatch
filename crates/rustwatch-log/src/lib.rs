// Internal module declarations for core logging components
mod level;
mod record;
mod logger;
mod formatter;
mod handler;
mod processor;

// Expose level enum at crate root
pub use level::Level;
// Expose all Record types at crate root
pub use record::*;
// Expose all Logger types at crate root
pub use logger::*;

// Expose formatter trait at crate root
pub use formatter::Formatter;
// Expose handler trait at crate root
pub use handler::Handler;
// Expose processor trait at crate root
pub use processor::Processor;

// Re-export all formatter implementations
pub mod formatters {
  pub use super::formatter::*;
}

// Re-export all handler implementations
pub mod handlers {
  pub use super::handler::*;
}

// Re-export all processor implementations
pub mod processors {
  pub use super::processor::*;
}
