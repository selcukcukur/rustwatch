use serde::Serialize;

/// Represents the log levels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Level {
  /// System is unusable, requires immediate attention
  Emergency,
  /// Immediate action must be taken, critical alert
  Alert,
  /// Critical conditions, serious failures
  Critical,
  /// Error conditions, runtime errors
  Error,
  /// Warning conditions, potential issues
  Warning,
  /// Normal but significant events, noteworthy conditions
  Notice,
  /// Informational messages, general system events
  Info,
  /// Debug-level messages, detailed diagnostic information
  Debug,
  /// User-defined custom log level, allows arbitrary string values
  /// with an optional numeric severity for ordering.
  Custom(String, Option<i32>),
}

impl Level {
  /// Returns the standardized name of this log level in lowercase.
  ///
  /// - `Level::Emergency` - **emergency** (System is unusable, requires immediate attention)
  /// - `Level::Alert` - **alert** (Immediate action must be taken, critical alert)
  /// - `Level::Critical` - **critical** (Critical conditions, serious failures)
  /// - `Level::Error` - **error** (Error conditions, runtime errors)
  /// - `Level::Warning` - **warning** (Warning conditions, potential issues)
  /// - `Level::Notice` - **notice** (Normal but significant events, noteworthy conditions)
  /// - `Level::Info` - **info** (Informational messages, general system events)
  /// - `Level::Debug` - **debug** (Debug-level messages, detailed diagnostic information)
  ///
  /// Custom levels are converted to lowercase strings.
  pub fn name(&self) -> String {
    match self {
      Level::Emergency => "emergency".into(),
      Level::Alert => "alert".into(),
      Level::Critical => "critical".into(),
      Level::Error => "error".into(),
      Level::Warning => "warning".into(),
      Level::Notice => "notice".into(),
      Level::Info => "info".into(),
      Level::Debug => "debug".into(),
      Level::Custom(s, _) => s.to_lowercase(),
    }
  }

  /// Returns true if this level is part of the given list of levels.
  ///
  /// **Parameters**
  /// - `levels` - A slice of `Level` values to check against.
  pub fn filter(&self, levels: &[Level]) -> bool {
    levels.contains(self)
  }

  /// Returns the numeric severity value for this log level.
  ///
  /// - `Emergency` - **0** System is unusable, requires immediate attention.
  /// - `Alert` - **1** Immediate action must be taken, critical alert.
  /// - `Critical` - **2** Critical conditions, serious failures.
  /// - `Error` - **3** Error conditions, runtime errors.
  /// - `Warning` - **4** Warning conditions, potential issues.
  /// - `Notice` - **5** Normal but significant events, noteworthy conditions.
  /// - `Info` - **6** Informational messages, general system events.
  /// - `Debug` - **7** Debug-level messages, detailed diagnostic information.
  ///
  /// Custom levels return their defined severity if provided,
  /// otherwise `-1` because they do not have a defined numeric mapping.
  pub fn severity(&self) -> i32 {
    match self {
      Level::Debug => 7,
      Level::Info => 6,
      Level::Notice => 5,
      Level::Warning => 4,
      Level::Error => 3,
      Level::Critical => 2,
      Level::Alert => 1,
      Level::Emergency => 0,
      Level::Custom(_, Some(v)) => *v,
      Level::Custom(_, None) => -1,
    }
  }

  /// Returns true if this level has a lower severity than the other.
  ///
  /// **Parameters**
  /// - `other` - The `Level` to compare against.
  pub fn is_lower_than(&self, other: &Level) -> bool {
    self.severity() < other.severity()
  }

  /// Returns true if this level has a higher severity than the other.
  ///
  /// **Parameters**
  /// - `other` - The `Level` to compare against.
  pub fn is_higher_than(&self, other: &Level) -> bool {
    self.severity() > other.severity()
  }

  /// Constructs a `Level` from a string name (case-insensitive).
  ///
  /// Each name corresponds to a standard log level:
  /// - `"debug"` - `Debug`
  /// - `"info"` - `Info`
  /// - `"notice"` - `Notice`
  /// - `"warning"` - `Warning`
  /// - `"error"` - `Error`
  /// - `"critical"` - `Critical`
  /// - `"alert"` - `Alert`
  /// - `"emergency"` - `Emergency`
  ///
  /// Any other string will be mapped to a `Custom` level
  /// with no predefined severity (`None`).
  pub fn from_name(name: &str) -> Self {
    match name.to_lowercase().as_str() {
      "debug" => Level::Debug,
      "info" => Level::Info,
      "notice" => Level::Notice,
      "warning" => Level::Warning,
      "error" => Level::Error,
      "critical" => Level::Critical,
      "alert" => Level::Alert,
      "emergency" => Level::Emergency,
      other => Level::Custom(other.to_string(), None),
    }
  }

  /// Constructs a `Level` from its numeric severity value.
  ///
  /// - `0` - **Emergency** (System is unusable, requires immediate attention)
  /// - `1` - **Alert** (Immediate action must be taken, critical alert)
  /// - `2` - **Critical** (Critical conditions, serious failures)
  /// - `3` - **Error** (Error conditions, runtime errors)
  /// - `4` - **Warning** (Warning conditions, potential issues)
  /// - `5` - **Notice** (Normal but significant events, noteworthy conditions)
  /// - `6` - **Info** (Informational messages, general system events)
  /// - `7` - **Debug** (Debug-level messages, detailed diagnostic information)
  ///
  /// Any other numeric value will be mapped to a `Custom` level
  /// with the numeric severity preserved.
  pub fn from_value(value: i32) -> Self {
    match value {
      0 => Level::Emergency,
      1 => Level::Alert,
      2 => Level::Critical,
      3 => Level::Error,
      4 => Level::Warning,
      5 => Level::Notice,
      6 => Level::Info,
      7 => Level::Debug,
      other => Level::Custom(other.to_string(), Some(other)),
    }
  }
}
