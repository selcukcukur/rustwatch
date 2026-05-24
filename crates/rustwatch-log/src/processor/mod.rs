use crate::LogRecord;

pub trait Processor {
  fn process(&self, record: &mut LogRecord);

  fn severity(&self) -> usize {
    0
  }
}

pub mod context;
