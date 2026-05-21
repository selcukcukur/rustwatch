use crate::record::Record;

pub trait Handler {
  fn log(&mut self, record: &Record) -> bool;
}

pub mod console;
pub mod file;
