use super::Handler;
use crate::Record;
use crate::formatter::Formatter;
use std::fs::OpenOptions;
use std::io::Write;

pub struct FileHandler {
    path: String,
    formatter: Box<dyn Formatter>,
}

impl FileHandler {
    pub fn new(path: &str, formatter: Box<dyn Formatter>) -> Self {
        Self {
            path: path.to_string(),
            formatter,
        }
    }
}

impl Handler for FileHandler {
    /// Log the record to a file.
    ///
    /// **Returns**
    /// - `true` → If the record was successfully written
    /// - `false` → If writing failed
    fn log(&mut self, record: &Record) -> bool {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            Ok(mut file) => {
                if writeln!(file, "{}", self.formatter.format(record)).is_ok() {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }
}
