/// Represents the severity level of a log record.
///
/// Includes common predefined levels (Info, Debug, Warn, Error, Trace)
/// and also supports custom user-defined levels.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Level {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
  Custom(String),
}

impl Level {
  /// Create a custom level from a string.
  pub fn custom<S: Into<String>>(name: S) -> Self {
    Level::Custom(name.into())
  }

  /// Convert level to string for display.
  pub fn as_str(&self) -> &str {
    match self {
      Level::Trace => "TRACE",
      Level::Debug => "DEBUG",
      Level::Info => "INFO",
      Level::Warn => "WARN",
      Level::Error => "ERROR",
      Level::Custom(s) => s.as_str(),
    }
  }
}
