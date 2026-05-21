use crate::record::Record;

pub trait Processor {
  fn process(&self, record: &mut Record);
}

pub mod context;
