use super::Formatter;
use crate::Record;
use serde_json::json;

/// A formatter that serializes a log record into JSON.
///
/// This formatter converts all core fields of a `Record`
/// (timestamp, level, message, channel, context) into a JSON string.
/// It supports both predefined levels (Info, Debug, Warn, Error, Trace)
/// and custom levels via `Level::Custom`.
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, record: &Record) -> String {
        json!({
            "timestamp": record.timestamp.to_rfc3339(),
            "level": record.level.name(),
            "message": record.message,
            "channel": record.channel,
            "context": record.context
        })
        .to_string()
    }
}
