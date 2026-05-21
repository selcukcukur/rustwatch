use crate::level::Level;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct Record {
  pub level: Level,
  pub message: String,
  pub timestamp: String,
  pub context: Option<String>,
}

impl Record {
  pub fn new(level: Level, message: &str) -> Self {
    Self {
      level,
      message: message.to_string(),
      timestamp: Local::now().to_rfc3339(),
      context: None,
    }
  }
}
