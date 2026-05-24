use crate::Level;
use crate::LogRecord;
use crate::processor::Processor;
use crate::handler::Handler;
use serde_json::Value;
use chrono::{Utc, DateTime};
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

/// Unified component type for Logger with optional severity.
pub enum LoggerComponent {
  Handler(Box<dyn Handler>),
  Processor(Box<dyn Processor>),
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

  /// Log an emergency message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn emergency(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Emergency, message, context);
  }

  /// Log an alert message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn alert(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Alert, message, context);
  }

  /// Log a critical message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn critical(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Critical, message, context);
  }

  /// Log an error message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn error(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Error, message, context);
  }

  /// Log a warning message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn warning(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Warning, message, context);
  }

  /// Log a notice to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn notice(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Notice, message, context);
  }

  /// Log an informational message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn info(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Info, message, context);
  }

  /// Log a debug message to the logs.
  ///
  /// **Parameters**
  /// - `message` - Log message string
  /// - `context` - Optional JSON context (default: `{}`)
  ///
  /// **Returns**
  /// - `bool` → Whether the record was handled by any handler
  pub fn debug(&mut self, message: &str, context: Option<Value>) {
    self.write(Level::Debug, message, context);
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
  pub fn write(&mut self, level: Level, message: &str, context: Option<Value>) -> bool {
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

    let mut record = LogRecord::new(
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

  pub fn push(&mut self, component: LoggerComponent) {
    match component {
      LoggerComponent::Handler(handler) => {
        self.handlers.push(handler);
        self.handlers.sort_by(|b, a| b.severity().cmp(&a.severity()));
      }
      LoggerComponent::Processor(processor) => {
        self.processors.push(processor);
        self.processors.sort_by(|b, a| b.severity().cmp(&a.severity()));
      }
    }
  }

  pub fn pop(&mut self, is_handler: bool) -> Option<LoggerComponent> {
    if is_handler {
      self.handlers.pop().map(LoggerComponent::Handler)
    } else {
      self.processors.pop().map(LoggerComponent::Processor)
    }
  }

  /// Flush the entire logger state (handlers, processors, depth)
  pub fn flush(&mut self) {
    self.handlers.clear();
    self.processors.clear();
    self.depth = 0;
    self.fiber_depth = 0;
    self.cycles = false;
  }

  /// Flush logger handlers
  pub fn flush_handlers(&mut self) {
    self.handlers.clear();
  }

  /// Flush logger processors
  pub fn flush_processors(&mut self) {
    self.processors.clear();
  }

  /// Flush logger depth state
  pub fn flush_depth(&mut self) {
    self.depth = 0;
    self.fiber_depth = 0;
    self.cycles = false;
  }
}
