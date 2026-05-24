use crate::record::Record;

pub trait Handler {
  fn log(&mut self, record: &Record) -> bool;

  fn severity(&self) -> usize {
    0
  }
}

pub mod console;
pub mod file;
