use crate::record::Record;
use super::Processor;

pub struct ContextProcessor {
  pub key: String,
  pub value: String,
}

impl Processor for ContextProcessor {
  fn process(&self, record: &mut Record) {
    record.context = Some(format!("{}={}", self.key, self.value));
  }
}
