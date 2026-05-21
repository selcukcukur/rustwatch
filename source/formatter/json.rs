use crate::record::Record;
use super::Formatter;
use serde_json::json;

/// A formatter that serializes a log record into JSON.
///
/// This formatter converts all core fields of a `Record`
/// (timestamp, level, message, channel, context) into a JSON string.
/// It is useful for structured logging, log aggregation systems,
/// or exporting logs to external services.
///
/// # Example
/// ```
/// use rustlog::level::Level;
/// use rustlog::record::Record;
/// use rustlog::formatter::json::JsonFormatter;
///
/// let record = Record::new(Level::Info, "Application started")
///     .with_channel("system")
///     .with_context(serde_json::json!({ "user": "selcuk" }));
///
/// let formatter = JsonFormatter;
/// println!("{}", formatter.format(&record));
/// ```
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
  fn format(&self, record: &Record) -> String {
    json!({
            "timestamp": record.timestamp.to_rfc3339(),
            "level": format!("{:?}", record.level),
            "message": record.message,
            "channel": record.channel,
            "context": record.context
        })
      .to_string()
  }
}
