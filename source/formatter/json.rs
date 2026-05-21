use crate::record::Record;
use super::Formatter;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
  fn format(&self, record: &Record) -> String {
    serde_json::json!({
            "timestamp": record.timestamp,
            "level": format!("{:?}", record.level),
            "message": record.message,
            "context": record.context
        }).to_string()
  }
}
