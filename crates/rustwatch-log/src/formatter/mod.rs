use crate::LogRecord;

pub trait Formatter {
  fn format(&self, record: &LogRecord) -> String;
}

pub mod line;
pub mod json;
