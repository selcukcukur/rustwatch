use super::Formatter;
use crate::Record;

/// A formatter that outputs log records in a simple line format.
///
/// This formatter is human‑readable and suitable for console output.
/// It includes timestamp, level, channel (if any), message, and context.
/// Context is serialized into JSON string form for readability.
pub struct LineFormatter;

impl Formatter for LineFormatter {
    fn format(&self, record: &Record) -> String {
        let ctx_str = if record.context.is_null() {
            "".to_string()
        } else {
            record.context.to_string()
        };

        format!(
            "[{}] {} [{}]: {} {}",
            record.timestamp.to_rfc3339(),
            record.level.name(),
            record.channel.clone(),
            record.message,
            ctx_str
        )
    }
}
