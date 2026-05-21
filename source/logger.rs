use crate::level::Level;
use crate::record::Record;
use crate::processor::Processor;
use crate::handler::Handler;

pub struct Logger {
  handlers: Vec<Box<dyn Handler>>,
  processors: Vec<Box<dyn Processor>>,
}

impl Logger {
  pub fn new() -> Self {
    Self { handlers: vec![], processors: vec![] }
  }

  pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
    self.handlers.push(handler);
  }

  pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
    self.processors.push(processor);
  }

  pub fn log(&mut self, level: Level, message: &str) {
    let mut record = Record::new(level, message);

    for p in &self.processors {
      p.process(&mut record);
    }

    for h in self.handlers.iter_mut() {
      h.log(&record);
    }
  }
}
