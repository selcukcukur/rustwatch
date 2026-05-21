use crate::record::Record;
use super::Formatter;

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
