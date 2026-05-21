use crate::record::Record;
use crate::formatter::Formatter;
use std::fs::OpenOptions;
use std::io::Write;

pub trait Handler {
  fn log(&mut self, record: &Record);
}

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

pub struct FileHandler {
  path: String,
  formatter: Box<dyn Formatter>,
}
impl FileHandler {
  pub fn new(path: &str, formatter: Box<dyn Formatter>) -> Self {
    Self { path: path.to_string(), formatter }
  }
}
impl Handler for FileHandler {
  fn log(&mut self, record: &Record) {
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&self.path)
      .unwrap();
    writeln!(file, "{}", self.formatter.format(record)).unwrap();
  }
}
