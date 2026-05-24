use crate::LogRecord;

pub trait Handler {
  fn log(&mut self, record: &LogRecord) -> bool;

  fn severity(&self) -> usize {
    0
  }
}

pub mod console;
pub mod file;
