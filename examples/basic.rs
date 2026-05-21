use rustlog::logger::Logger;
use rustlog::level::Level;
use rustlog::formatter::{line::LineFormatter, json::JsonFormatter};
use rustlog::processor::context::ContextProcessor;
use rustlog::handler::{console::ConsoleHandler, file::FileHandler};
use serde_json::json;

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

  // Log some messages with channel + context
  logger.log(
    Level::Info,
    "Application started",
    Some("system"),
    Some(json!({ "env": "dev", "version": "0.1.0" })),
  );

  logger.log(
    Level::Debug,
    "Debugging details here",
    Some("debug"),
    Some(json!({ "trace_id": "xyz-789" })),
  );

  logger.log(
    Level::Warn,
    "This is a warning",
    Some("security"),
    Some(json!({ "ip": "192.168.1.10" })),
  );

  logger.log(
    Level::Error,
    "Something went wrong!",
    Some("database"),
    Some(json!({ "query": "SELECT * FROM users", "duration_ms": 1200 })),
  );
}
