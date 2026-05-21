use rustlog::logger::Logger;
use rustlog::level::Level;
use rustlog::formatter::{line::LineFormatter, json::JsonFormatter};
use rustlog::processor::context::ContextProcessor;
use rustlog::handler::{console::ConsoleHandler, file::FileHandler};

fn main() {
  // Create a new logger instance
  let mut logger = Logger::new();

  // Add console handler with line formatter
  logger.add_handler(Box::new(ConsoleHandler::new(Box::new(LineFormatter))));

  // Add file handler with JSON formatter
  logger.add_handler(Box::new(FileHandler::new("app.log", Box::new(JsonFormatter))));

  // Add a processor to enrich log records with context
  logger.add_processor(Box::new(ContextProcessor {
    key: "user".into(),
    value: "selcuk".into(),
  }));

  // Log some messages
  logger.log(Level::Info, "Application started");
  logger.log(Level::Debug, "Debugging details here");
  logger.log(Level::Warn, "This is a warning");
  logger.log(Level::Error, "Something went wrong!");
}
