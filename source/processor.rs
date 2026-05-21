use crate::record::Record;

pub trait Processor {
  fn process(&self, record: &mut Record);
}

pub struct ContextProcessor {
  pub key: String,
  pub value: String,
}

impl Processor for ContextProcessor {
  fn process(&self, record: &mut Record) {
    record.context = Some(format!("{}={}", self.key, self.value));
  }
}
