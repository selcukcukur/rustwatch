use crate::Level;
use crate::Record;
use crate::handler::Handler;
use crate::processor::Processor;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde_json::Value;

/// Core logger instance responsible for processing and dispatching log records.
///
/// A `Logger` represents a named logging pipeline that processes log records
/// through a sequence of processors and dispatches them to configured handlers.
///
/// It maintains internal runtime state such as recursion depth and cycle
/// detection to safely support nested logging scenarios.
pub struct Logger {
    /// Logger identifier used for grouping and filtering logs
    name: String,

    /// Timezone used when formatting timestamps in log records
    timezone: String,

    /// Tracks nesting level of logging calls
    depth: usize,

    /// Tracks execution context depth (e.g. async/fiber systems)
    fiber_depth: usize,

    /// Prevents infinite logging recursion loops
    cycles: bool,

    /// Consume and output processed log records
    handlers: Vec<Box<dyn Handler>>,

    /// Transform or enrich log records before output
    processors: Vec<Box<dyn Processor>>,
}

impl Logger {
    /// Creates a new **Logger** instance.
    ///
    /// **Arguments**
    /// - `name` - Logger identifier used for grouping and filtering logs
    /// - `timezone` - Timezone identifier used for timestamp formatting
    ///
    /// **Returns**
    /// - `self` - A fresh logger with no handlers or processors registered.
    pub fn new(name: impl Into<String>, timezone: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            timezone: timezone.into(),
            depth: 0,
            fiber_depth: 0,
            cycles: false,
            handlers: vec![],
            processors: vec![],
        }
    }

    /// Logs a message at **emergency** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn emergency(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Emergency, message, context);
    }

    /// Logs a message at **alert** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn alert(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Alert, message, context);
    }

    /// Logs a message at **critical** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn critical(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Critical, message, context);
    }

    /// Logs a message at **error** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn error(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Error, message, context);
    }

    /// Logs a message at **warning** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn warning(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Warning, message, context);
    }

    /// Logs a message at **notice** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn notice(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Notice, message, context);
    }

    /// Logs a message at **info** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn info(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Info, message, context);
    }

    /// Logs a message at **debug** level.
    ///
    /// **Arguments**
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn debug(&mut self, message: &str, context: Option<Value>) {
        self.write(Level::Debug, message, context);
    }

    /// Logs a message with an explicit severity level.
    ///
    /// **Arguments**
    /// - `level` - Severity level of the log
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` - if at least one handler processed the record
    pub fn log(&mut self, level: Level, message: &str, context: Option<Value>) -> bool {
        self.write(level, message, context)
    }

    /// Processes and dispatches a log record through the pipeline.
    ///
    /// - `Stage 1` - Tracks recursion depth and prevents infinite loops
    /// - `Stage 2` - Resolves timestamp using configured timezone
    /// - `Stage 3` - Builds a **record** instance
    /// - `Stage 5` - Runs processors (mutation/enrichment phase)
    /// - `Stage 5` - Dispatches to handlers (output phase)
    /// - `Stage 6` - Restores internal state after execution
    ///
    /// **Arguments**
    /// - `level` - Severity level of the log
    /// - `message` - Log message
    /// - `context` - Optional structured metadata
    ///
    /// **Returns**
    /// - `true` if at least one handler successfully processed the record
    ///
    pub fn write(&mut self, level: Level, message: &str, context: Option<Value>) -> bool {
        let log_depth = if self.cycles {
            self.fiber_depth += 1;
            self.fiber_depth
        } else {
            self.depth += 1;
            self.depth
        };

        if log_depth == 3 {
            self.warning(
                "A possible infinite logging loop was detected and aborted. \
             It appears some of your handler code is triggering logging.",
                None,
            );
            return false;
        } else if log_depth >= 5 {
            return false;
        }

        // timezone string → Tz enum parse
        let tz: Tz = self.timezone.parse().unwrap_or(chrono_tz::UTC);
        let timestamp: DateTime<Tz> = Utc::now().with_timezone(&tz);

        let mut record = Record::new(level, message, &self.name, context, timestamp);

        let mut handled = false;

        for processor in &self.processors {
            processor.process(&mut record);
        }

        for handler in self.handlers.iter_mut() {
            if handler.log(&record) {
                handled = true;
                break;
            }
        }

        if self.cycles {
            if self.fiber_depth > 0 {
                self.fiber_depth -= 1;
            }
        } else {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }

        handled
    }

    /// Registers a new log handler in the logger pipeline.
    ///
    /// **Arguments**
    /// - `handler` - The handler instance to register
    pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.handlers.push(handler);
        self.handlers.sort_by(|b, a| b.order().cmp(&a.order()));
    }

    /// Registers a new log processor in the logger pipeline.
    ///
    /// **Arguments**
    /// - `processor` - The processor instance to register
    pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
        self.processors.push(processor);
        self.processors.sort_by(|b, a| b.order().cmp(&a.order()));
    }

    /// Removes the most recently added log handler.
    ///
    /// **Returns**
    /// - `Some(handler)` if a handler existed
    /// - `None` if no handlers are registered
    pub fn pop_handler(&mut self) -> Option<Box<dyn Handler>> {
        self.handlers.pop()
    }

    /// Removes the most recently added log processor
    ///
    /// **Returns**
    /// - `Some(processor)` if a processor existed
    /// - `None` if no processors are registered
    pub fn pop_processor(&mut self) -> Option<Box<dyn Processor>> {
        self.processors.pop()
    }

    /// Resets the logger to its initial state.
    pub fn flush(&mut self) {
        self.handlers.clear();
        self.processors.clear();
        self.depth = 0;
        self.fiber_depth = 0;
        self.cycles = false;
    }

    /// Removes all registered handlers from the logger.
    pub fn flush_handlers(&mut self) {
        self.handlers.clear();
    }

    /// Removes all registered processors from the logger.
    pub fn flush_processors(&mut self) {
        self.processors.clear();
    }

    /// Resets internal execution state of the logger.
    pub fn flush_depth(&mut self) {
        self.depth = 0;
        self.fiber_depth = 0;
        self.cycles = false;
    }
}
