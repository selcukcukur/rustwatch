use crate::level::Level;
use chrono::{Local, DateTime};
use serde_json::Value;

/// Represents a single log entry in the `rustlog` system
///
/// **Fields**
/// - `level` - Log severity level (Info, Error, etc.)
/// - `message` - The actual log message string
/// - `timestamp` - Exact timestamp when the record was created
/// - `context` - Flexible JSON context for extra metadata
/// - `channel` - Logical channel or category (e.g. "auth", "db", "network")
///
/// # Example
/// ```
/// use rustlog::level::Level;
/// use rustlog::record::Record;
///
/// let mut record = Record::new(Level::Error, "Database connection failed")
///     .with_channel("db")
///     .with_context(serde_json::json!({ "user": "selcuk", "request_id": "abc-123" }));
///
/// println!("{:?}", record);
/// ```
#[derive(Debug, Clone)]
pub struct Record {
  ///< Log severity
  pub level: Level,
  ///< Log message
  pub message: String,
  ///< Exact timestamp
  pub timestamp: DateTime<Local>,
  /// Flexible JSON context
  pub context: Value,
  /// Logical channel/category
  pub channel: Option<String>,
}

impl Record {
  /// Create a new log record with default empty context.
  ///
  /// **Parameters**
  /// - `level` - The severity level of the log (e.g. Info, Warn, Error)
  /// - `message` - The log message string
  ///
  /// **Usage**
  /// ```
  /// use rustlog::level::Level;
  /// use rustlog::record::Record;
  ///
  /// let record = Record::new(Level::Info, "Application started");
  ///
  /// println!("{:?}", record);
  /// ```
  pub fn new(level: Level, message: &str) -> Self {
    Self {
      level,
      message: message.to_string(),
      timestamp: Local::now(),
      context: Value::Null,
      channel: None,
    }
  }

  /// Attach a channel/category to the record.
  ///
  /// **Parameters**
  /// - `channel` - A string representing the logical channel/category
  ///
  /// **Usage**
  /// ```
  /// use rustlog::level::Level;
  /// use rustlog::record::Record;
  ///
  /// let record = Record::new(Level::Warn, "Slow query detected")
  ///     .with_channel("database");
  ///
  /// println!("{:?}", record);
  /// ```
  pub fn with_channel(mut self, channel: &str) -> Self {
    self.channel = Some(channel.to_string());
    self
  }

  /// Attach custom context to the record.
  ///
  /// **Parameters**
  /// - `context` - Any type that can be converted into **serde_json::value**
  ///
  /// **Usage**
  /// ```
  /// use rustlog::level::Level;
  /// use rustlog::record::Record;
  ///
  /// Record::new(Level::Info, "User login successful")
  ///   .with_context(json!({
  ///     "user": "selcuk",
  ///     "ip": "192.168.1.10",
  ///     "request_id": "abc-123"
  ///   }));
  ///
  /// println!("{:?}", record);
  /// ```
  pub fn with_context<T: Into<Value>>(mut self, context: T) -> Self {
    self.context = context.into();
    self
  }
}
