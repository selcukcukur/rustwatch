use rustlog::logger::Logger;
use rustlog::level::Level;
use rustlog::formatter::{line::LineFormatter, json::JsonFormatter};
use rustlog::processor::context::ContextProcessor;
use rustlog::handler::{console::ConsoleHandler, file::FileHandler};
use serde_json::json;

fn main() {
  // Logger instance with name + timezone
  let mut logger = Logger::new("app", "Europe/Istanbul");

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
    Some(json!({ "env": "dev", "version": "0.1.0" })),
  );

  logger.log(
    Level::Debug,
    "Debugging details here",
    Some(json!({ "trace_id": "xyz-789" })),
  );

  logger.log(
    Level::Warning,
    "This is a warning",
    Some(json!({ "ip": "192.168.1.10" })),
  );

  logger.log(
    Level::Error,
    "Something went wrong!",
    Some(json!({ "query": "SELECT * FROM users", "duration_ms": 1200 })),
  );

  logger.log(
    Level::Custom("SECURITY".to_string(), Some(8)),
    "Unauthorized access attempt",
    Some(json!({ "ip": "10.0.0.5", "method": "POST /login" })),
  );

  logger.log(
    Level::Custom("PERF".to_string(), Some(9)),
    "High memory usage detected",
    Some(json!({ "memory_mb": 2048, "threshold": 1024 })),
  );
}
