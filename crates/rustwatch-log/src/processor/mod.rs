use crate::Record;

pub trait Processor {
  fn process(&self, record: &mut Record);

  fn severity(&self) -> usize {
    0
  }
}

pub mod context;
