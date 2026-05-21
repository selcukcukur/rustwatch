use crate::record::Record;
use super::Formatter;
use serde_json::json;

/// A formatter that serializes a log record into JSON.
///
/// This formatter converts all core fields of a `Record`
/// (timestamp, level, message, channel, context) into a JSON string.
/// It supports both predefined levels (Info, Debug, Warn, Error, Trace)
/// and custom levels via `Level::Custom`.
///
/// # Example
/// ```
/// use rustlog::level::Level;
/// use rustlog::record::Record;
/// use rustlog::formatter::json::JsonFormatter;
///
/// let record = Record::new(Level::custom("SECURITY"), "Unauthorized access attempt")
///     .with_channel("auth")
///     .with_context(serde_json::json!({ "ip": "192.168.1.10" }));
///
/// let formatter = JsonFormatter;
/// println!("{}", formatter.format(&record));
/// ```
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
  fn format(&self, record: &Record) -> String {
    json!({
            "timestamp": record.timestamp.to_rfc3339(),
            "level": record.level.as_str(),
            "message": record.message,
            "channel": record.channel,
            "context": record.context
        })
      .to_string()
  }
}
