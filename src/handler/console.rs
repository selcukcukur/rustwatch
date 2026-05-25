use super::Handler;
use crate::Record;
use crate::formatter::Formatter;

pub struct ConsoleHandler {
    formatter: Box<dyn Formatter>,
}

impl ConsoleHandler {
    pub fn new(formatter: Box<dyn Formatter>) -> Self {
        Self { formatter }
    }
}

impl Handler for ConsoleHandler {
    /// Log the record to the console.
    ///
    /// **Returns**
    /// - `true` → Always returns true since console output never fails
    fn log(&mut self, record: &Record) -> bool {
        println!("{}", self.formatter.format(record));
        true
    }
}
