use crate::record::Record;
use crate::formatter::Formatter;
use super::Handler;

pub struct ConsoleHandler {
  formatter: Box<dyn Formatter>,
}

impl ConsoleHandler {
  pub fn new(formatter: Box<dyn Formatter>) -> Self {
    Self { formatter }
  }
}

impl Handler for ConsoleHandler {
  fn log(&mut self, record: &Record) {
    println!("{}", self.formatter.format(record));
  }
}
