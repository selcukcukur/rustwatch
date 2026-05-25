use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
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
    Custom(String, Option<i8>),
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
    pub fn severity(&self) -> i8 {
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

    /// Returns true if this level is more severe than `other`.
    ///
    /// **Parameters**
    /// - `other`: The `Level` to compare against.
    ///
    /// **Returns**
    /// - `true` if this level is more severe than `other`
    pub fn above(&self, other: &Self) -> bool {
        self.severity() < other.severity()
    }

    /// Returns true if this level is less severe than `other`.
    ///
    /// **Parameters**
    /// - `other`: The `Level` to compare against.
    ///
    /// **Returns**
    /// - `true` if this level is less severe than `other`
    pub fn below(&self, other: &Self) -> bool {
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
    pub fn from_severity(severity: i8) -> Self {
        match severity {
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

/// Enables converting `Level` into a human-readable string representation.
///
/// This implementation is used when logging, displaying, or serializing
/// log levels in a user-facing format.
impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name().as_ref())
    }
}

/// Parses a `Level` from its string representation.
///
/// This allows `Level` to be created from config files, environment
/// variables, or user input such as "info", "debug", "error".
impl std::str::FromStr for Level {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_name(s))
    }
}

/// Defines partial ordering behavior for `Level`.
///
/// Required for comparisons like `level > other_level` where ordering
/// is not strictly guaranteed for all possible values.
impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Defines total ordering for `Level` based on severity.
///
/// Higher severity levels are considered "less" in ordering due to
/// `.reverse()`, so that more critical levels come first when sorted.
impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.severity().cmp(&other.severity()).reverse()
    }
}