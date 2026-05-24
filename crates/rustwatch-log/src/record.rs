use crate::Level;
use chrono::{DateTime};
use chrono_tz::Tz;
use serde_json::{Value};

/// Represents a single log record in the logging system.
///
/// A `Record` encapsulates all metadata associated with a log entry,
/// including severity level, message, timestamp, channel, context, and
/// optional formatted output.
///
/// **Usage**
/// ```
/// use rustwatch_log::{Record, Level};
/// use chrono::Utc;
/// use chrono_tz::UTC;
///
/// // Create a new log record with all fields populated
/// let record = Record {
///     // Log severity level (Error, Info, Debug, etc.)
///     level     : Level::Error,
///
///     // The actual log message string
///     message   : "Database connection failed".to_string(),
///
///     // Exact timezone-aware timestamp when the record was created
///     timestamp : Utc::now().with_timezone(&UTC),
///
///     // JSON metadata for additional context (e.g. request id, user info)
///     context   : serde_json::json!({ "request_id": "45db62f4-2b83-4704-9d83-e00947413948" }),
///
///     // Channel/category name (e.g. "auth", "db")
///     channel   : "db".to_string()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Record {
  /// Log severity level (Info, Error, Debug, etc.)
  pub level: Level,

  /// The actual log message string
  pub message: String,

  /// Exact timezone-aware timestamp when the record was created
  pub timestamp: DateTime<Tz>,

  /// Metadata for additional context (e.g. user, request id)
  pub context: Value,

  /// Channel/category name (e.g. "auth", "db")
  pub channel: String,

  /// Optional pre-formatted string representation of the record
  pub formatted: Option<String>,
}

impl Record {
  /// Create a new log record with all fields configurable.
  ///
  /// **Parameters**
  /// - `level` - Log severity level
  /// - `message` - Log message string
  /// - `channel` - Channel/category name
  /// - `context` - JSON context metadata (default: `{}`)
  /// - `timestamp` - Exact timestamp (default: Local::now())
  ///
  /// **Returns**
  /// - `Record` → A fully initialized log record
  pub fn new(
    level: Level,
    message: &str,
    channel: &str,
    context: Option<Value>,
    timestamp: DateTime<Tz>,
  ) -> Self {
    Self {
      level,
      message: message.to_string(),
      timestamp,
      context: context.unwrap_or_else(|| Value::Object(serde_json::Map::new())),
      channel: channel.to_string(),
      formatted: None,
    }
  }

  /// Get the channel/category name of this log record.
  ///
  /// **Returns**
  /// - `String` → A reference to the channel string associated with the record
  pub fn channel(&self) -> &String {
    &self.channel
  }

  /// Get the context metadata attached to this log record.
  ///
  /// **Returns**
  /// - `&Value` → A reference to the JSON context associated with the record
  pub fn context(&self) -> &Value {
    &self.context
  }

  /// Get the log severity level of this record.
  ///
  /// **Returns**
  /// - `&Level` → A reference to the log level associated with the record
  pub fn level(&self) -> &Level {
    &self.level
  }

  /// Get the numeric severity value derived from the log level.
  ///
  /// **Returns**
  /// - `i8` → The numeric severity value (0–7 for standard levels, or custom-defined value)
  pub fn severity(&self) -> i8 {
    self.level.severity()
  }

  /// Get the timestamp of when this record was created.
  ///
  /// **Returns**
  /// - `DateTime<Tz>` → A reference to the timezone-aware timestamp of the record
  pub fn timestamp(&self) -> &DateTime<Tz> {
    &self.timestamp
  }

  /// Convert this log record into a JSON map representation.
  ///
  /// **Returns**
  /// - `Map<String, Value>` → A JSON object containing all record fields.
  pub fn to_array(&self) -> serde_json::Map<String, Value> {
    let mut map = serde_json::Map::new();
    map.insert("message".to_string(), Value::String(self.message.clone()));
    map.insert("level".to_string(), Value::String(self.level.name().into()));
    map.insert("severity".to_string(), Value::Number(self.severity().into()));
    map.insert("channel".to_string(), Value::String(self.channel.clone()));
    map.insert("timestamp".to_string(), Value::String(self.timestamp.to_rfc3339()));
    map.insert("context".to_string(), self.context.clone());
    if let Some(f) = &self.formatted {
      map.insert("formatted".to_string(), Value::String(f.clone()));
    }
    map
  }

  /// Convert this log record into a JSON string representation.
  ///
  /// **Returns**
  /// - `String` → A JSON string containing all record fields.
  pub fn to_json(&self) -> String {
    Value::Object(self.to_array()).to_string()
  }

  /// Override fields of the record with new values.
  ///
  /// **Parameters**
  /// - `level` - Optional new log level
  /// - `message` - Optional new message string
  /// - `channel` - Optional new channel name
  /// - `context` - Optional new json context
  /// - `timestamp` - Optional new timestamp
  /// - `formatted` - Optional new formatted string
  ///
  /// **Returns**
  /// - `Record` - A new record with updated fields
  pub fn with(
    &self,
    level: Option<Level>,
    message: Option<&str>,
    channel: Option<&str>,
    context: Option<Value>,
    timestamp: Option<DateTime<Tz>>,
    formatted: Option<&str>,
  ) -> Self {
    Self {
      level: level.unwrap_or(self.level.clone()),
      message: message.unwrap_or(&self.message).to_string(),
      timestamp: timestamp.unwrap_or(self.timestamp),
      context: context.unwrap_or(self.context.clone()),
      channel: channel.unwrap_or(&self.channel).to_string(),
      formatted: formatted.map(|f| f.to_string()).or_else(|| self.formatted.clone()),
    }
  }
}
