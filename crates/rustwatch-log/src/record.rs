use crate::Level;
use chrono::{DateTime};
use chrono_tz::Tz;
use serde_json::{Value};

#[derive(Debug, Clone)]
pub struct Record {
  pub level: Level,
  pub message: String,
  pub timestamp: DateTime<Tz>,
  pub context: Value,
  pub channel: String,
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


  /// Get the channel/category of this record.
  pub fn channel(&self) -> &String {
    &self.channel
  }

  /// Get the JSON context attached to this record.
  pub fn context(&self) -> &Value {
    &self.context
  }

  /// Get the log level of this record.
  pub fn level(&self) -> &Level {
    &self.level
  }

  /// Get the severity value derived from the log level.
  pub fn severity(&self) -> i8 {
    self.level.severity()
  }

  /// Get the timestamp of when this record was created.
  pub fn timestamp(&self) -> &DateTime<Tz> {
    &self.timestamp
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

  /// Convert record to a JSON map.
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

  /// Convert record to a JSON string.
  pub fn to_json(&self) -> String {
    Value::Object(self.to_array()).to_string()
  }
}
