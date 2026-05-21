use crate::level::Level;
use crate::record::Record;
use crate::processor::Processor;
use crate::handler::Handler;
use serde_json::Value;
use chrono::{Utc, DateTime, Local};
use chrono_tz::Tz;

/// Central logging component that manages handlers and processors.
///
/// The `Logger` is responsible for creating log records, enriching them
/// with processors, and dispatching them to handlers. Handlers determine
/// where the logs are written (console, file, external service), while
/// processors enrich records with metadata (e.g. user, request id).
pub struct Logger {
  pub name: String,
  pub timezone: String,
  pub depth: usize,
  pub fiber_depth: usize,
  pub cycles: bool,
  handlers: Vec<Box<dyn Handler>>,
  processors: Vec<Box<dyn Processor>>,
}

impl Logger {
  /// Create a new logger with empty handler and processor lists.
  pub fn new(name: impl Into<String>, timezone: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      timezone: timezone.into(),
      depth: 0,
      fiber_depth: 0,
      cycles: false,
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

  /// Log an emergency message to the logs.
  pub fn emergency(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Emergency, message, context);
  }

  /// Log an alert message to the logs.
  pub fn alert(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Alert, message, context);
  }

  /// Log a critical message to the logs
  pub fn critical(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Critical, message, context);
  }

  /// Log an error message to the logs
  pub fn error(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Error, message, context);
  }

  /// Log a warning message to the logs
  pub fn warning(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Warning, message, context);
  }

  /// Log a notice to the logs
  pub fn notice(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Notice, message, context);
  }

  /// Log an informational message to the logs
  pub fn info(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Info, message, context);
  }

  /// Log a debug message to the logs
  pub fn debug(&mut self, message: &str, context: Option<Value>) {
    self.log(Level::Debug, message, context);
  }

  /// Log a message with the given level and optional metadata.
  ///
  /// **Parameters**
  /// - `level` - Severity of the log
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn log(&mut self, level: Level, message: &str, context: Option<Value>) -> bool {
    let log_depth = if self.cycles {
      self.fiber_depth += 1;
      self.fiber_depth
    } else {
      self.depth += 1;
      self.depth
    };

    if log_depth == 3 {
      self.warning(
        "A possible infinite logging loop was detected and aborted. \
             It appears some of your handler code is triggering logging.",
        None,
      );
      return false;
    } else if log_depth >= 5 {
      return false;
    }

    // timezone string → Tz enum parse
    let tz: Tz = self.timezone.parse().unwrap_or(chrono_tz::UTC);
    let timestamp: DateTime<Tz> = Utc::now().with_timezone(&tz);

    let mut record = Record::new(
      level,
      message,
      &self.name,
      context,
      timestamp,
    );


    let mut handled = false;

    for processor in &self.processors {
      processor.process(&mut record);
    }

    for handler in self.handlers.iter_mut() {
      if handler.log(&record) {
        handled = true;
        break;
      }
    }

    if self.cycles {
      if self.fiber_depth > 0 {
        self.fiber_depth -= 1;
      }
    } else {
      if self.depth > 0 {
        self.depth -= 1;
      }
    }

    handled
  }
}
