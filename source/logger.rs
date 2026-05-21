use crate::level::Level;
use crate::record::Record;
use crate::processor::Processor;
use crate::handler::Handler;
use serde_json::Value;

/// Central logging component that manages handlers and processors.
///
/// The `Logger` is responsible for creating log records, enriching them
/// with processors, and dispatching them to handlers. Handlers determine
/// where the logs are written (console, file, external service), while
/// processors enrich records with metadata (e.g. user, request id).
///
/// # Example
/// ```
/// use rustlog::logger::Logger;
/// use rustlog::level::Level;
/// use rustlog::formatter::line::LineFormatter;
/// use rustlog::handler::console::ConsoleHandler;
///
/// let mut logger = Logger::new();
/// logger.add_handler(Box::new(ConsoleHandler::new(Box::new(LineFormatter))));
///
/// logger.log(Level::Info, "Application started");
/// ```
pub struct Logger {
  handlers: Vec<Box<dyn Handler>>,
  processors: Vec<Box<dyn Processor>>,
}

impl Logger {
  /// Create a new logger with empty handler and processor lists.
  pub fn new() -> Self {
    Self {
      handlers: vec![],
      processors: vec![],
    }
  }

  /// Add a handler to the logger (e.g. console, file, external service).
  pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
    self.handlers.push(handler);
  }

  /// Add a processor to the logger (e.g. context enrichment).
  pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
    self.processors.push(processor);
  }

  /// Log a message with the given level and optional metadata.
  ///
  /// **Parameters**
  /// - `level` - Severity of the log
  /// - `message` - Log message string
  /// - `channel` - Optional channel/category (default: None)
  /// - `context` - Optional JSON context (default: Null)
  pub fn log(&mut self, level: Level, message: &str, channel: Option<&str>, context: Option<Value>) {
    let mut record = Record::new(level, message);

    if let Some(ch) = channel {
      record.channel = Some(ch.to_string());
    }

    if let Some(ctx) = context {
      record.context = ctx;
    }

    // Processors enrich the record
    for p in &self.processors {
      p.process(&mut record);
    }

    // Handlers output the record
    for h in self.handlers.iter_mut() {
      h.log(&record);
    }
  }
}
