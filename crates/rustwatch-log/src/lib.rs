// rustwatch-log/src/lib.rs

mod level;
mod record;
mod logger;
mod formatter;
mod handler;
mod processor;

// Core API re-export
pub use level::Level;
pub use record::*;
pub use logger::*;

// Trait’leri direkt export et
pub use formatter::Formatter;
pub use handler::Handler;
pub use processor::Processor;

// Alt implementasyonları modül olarak aç
pub mod formatters {
  pub use super::formatter::*;
}

pub mod handlers {
  pub use super::handler::*;
}

pub mod processors {
  pub use super::processor::*;
}
