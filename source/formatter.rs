use crate::record::Record;

pub trait Formatter {
  fn format(&self, record: &Record) -> String;
}

pub struct LineFormatter;
impl Formatter for LineFormatter {
  fn format(&self, record: &Record) -> String {
    format!("[{}] {:?}: {} {}",
            record.timestamp,
            record.level,
            record.message,
            record.context.clone().unwrap_or_default()
    )
  }
}

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
