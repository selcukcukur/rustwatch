use crate::LogRecord;
use super::Formatter;

/// A formatter that outputs log records in a simple line format.
///
/// This formatter is human‑readable and suitable for console output.
/// It includes timestamp, level, channel (if any), message, and context.
/// Context is serialized into JSON string form for readability.
///
/// # Example
/// ```
/// use rustlog::level::Level;
/// use rustlog::record::Record;
/// use rustlog::formatter::line::LineFormatter;
///
/// let record = Record::new(Level::Warn, "Slow query detected")
///     .with_channel("database")
///     .with_context(serde_json::json!({ "duration_ms": 1200 }));
///
/// let formatter = LineFormatter;
/// println!("{}", formatter.format(&record));
/// ```
pub struct LineFormatter;

impl Formatter for LineFormatter {
  fn format(&self, record: &LogRecord) -> String {
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
