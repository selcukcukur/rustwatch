use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use crate::error::Error;

/// Log severity levels used to classify and control log output.
///
/// **Usage**
/// ```rust
/// use rustwatch::Level;
///
/// let level = Level::Critical;
/// let module = "auth_service";
/// let message = "Database connection failed";
///
/// // Early filtering (avoid unnecessary work)
/// if level.below(&Level::Info) {
///     return;
/// }
///
/// // Enrichment step (add context before output)
/// let formatted = format!(
///     "[{}] [{}] {}",
///     level,
///     module,
///     message
/// );
///
/// // Routing decision (where this log should go)
/// match level {
///     Level::Emergency | Level::Alert => {
///         // route to alerting system
///     }
///     Level::Critical => {
///         // route to incident tracking system
///     }
///     Level::Error => {
///         // route to error logging backend
///     }
///     Level::Warning => {
///         // route to standard monitoring
///     }
///     _ => {
///         // route to debug / local output
///     }
/// }
/// ```
///
/// **Scenarios**
/// - **System failures and crashes**
///   Use `Emergency`, `Alert`, or `Critical` for conditions that require
///   immediate attention or indicate system instability.
/// - **Runtime errors**
///   Use `Error` when operations fail but the system can continue running.
/// - **Unexpected but non-critical conditions**
///   Use `Warning` for recoverable or unexpected situations.
/// - **General application events**
///   Use `Notice` for significant lifecycle events such as startup or shutdown.
/// - **Standard operational logging**
///   Use `Info` for normal application behavior and state changes.
/// - **Debugging and diagnostics**
///   Use `Debug` for detailed internal state useful during development.
///
/// **Notes**
/// - The ordering of levels is fixed and used for filtering and comparison.
/// - Lower severity values indicate higher importance.
/// - This type is used across the logging pipeline (logger, processor, handler).
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
    Debug
}

impl Level {
    /// Returns true if this level is more severe than **other**.
    ///
    /// **Arguments**
    /// - `other` - The `Level` to compare against.
    ///
    /// **Returns**
    /// - `true` if this level is more severe than `other`
    pub fn above(&self, other: &Self) -> bool {
        self.severity() < other.severity()
    }

    /// Returns true if this level is less severe than **other**.
    ///
    /// **Arguments**
    /// - `other`: The `Level` to compare against.
    ///
    /// **Returns**
    /// - `true` if this level is less severe than `other`
    pub fn below(&self, other: &Self) -> bool {
        self.severity() > other.severity()
    }

    /// Returns the standardized **name* of this log level in lowercase.
    ///
    /// **Returns**
    /// - `Level::Emergency` - **emergency** (System is unusable, requires immediate attention)
    /// - `Level::Alert` - **alert** (Immediate action must be taken, critical alert)
    /// - `Level::Critical` - **critical** (Critical conditions, serious failures)
    /// - `Level::Error` - **error** (Error conditions, runtime errors)
    /// - `Level::Warning` - **warning** (Warning conditions, potential issues)
    /// - `Level::Notice` - **notice** (Normal but significant events, noteworthy conditions)
    /// - `Level::Info` - **info** (Informational messages, general system events)
    /// - `Level::Debug` - **debug** (Debug-level messages, detailed diagnostic information)
    pub fn name(&self) -> &'static str {
        match self {
            Level::Emergency => "emergency",
            Level::Alert => "alert",
            Level::Critical => "critical",
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Notice => "notice",
            Level::Info => "info",
            Level::Debug => "debug",
        }
    }

    /// Returns the numeric **severity** value associated with this log level.
    ///
    /// **Returns**
    /// - `Emergency` - **0** (System is unusable and requires immediate attention)
    /// - `Alert` - **1** (Immediate action must be taken)
    /// - `Critical` - **2** (Critical conditions and serious failures)
    /// - `Error` - **3** (Error conditions and runtime failures)
    /// - `Warning` - **4** (Potential issues or unexpected situations)
    /// - `Notice` - **5** (Significant but normal events)
    /// - `Info` - **6** (Informational messages and general system events)
    /// - `Debug` - **7** (Detailed diagnostic information)
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
        }
    }

    /// Constructs a log **level** from a lowercase string name.
    ///
    /// **Arguments**
    /// - `name` - Lowercase string representation of the log level.
    ///
    /// **Returns**
    /// - `emergency` - **Emergency** (System is unusable and requires immediate attention)
    /// - `alert` - **Alert** (Immediate action must be taken)
    /// - `critical` - **Critical** (Critical conditions and serious failures)
    /// - `error` - **Error** (Error conditions and runtime failures)
    /// - `warning` - **Warning** (Potential issues or unexpected situations)
    /// - `notice` - **Notice** (Significant but normal events)
    /// - `info` - **Info** (Informational messages and general system events)
    /// - `debug` - **Debug** (Detailed diagnostic information)
    pub fn from_name(name: &str) -> Result<Self, Error> {
        match name.as_bytes() {
            b"debug" => Ok(Self::Debug),
            b"info" => Ok(Self::Info),
            b"notice" => Ok(Self::Notice),
            b"warning" => Ok(Self::Warning),
            b"error" => Ok(Self::Error),
            b"critical" => Ok(Self::Critical),
            b"alert" => Ok(Self::Alert),
            b"emergency" => Ok(Self::Emergency),
            _ => Err(Error::InvalidLevel(name.to_string())),
        }
    }

    /// Constructs a log **level** from its numeric severity value.
    ///
    /// **Arguments**
    /// - `severity` - Numeric severity value of the log level.
    ///
    /// **Returns**
    /// - `0` - **Emergency** (System is unusable, requires immediate attention)
    /// - `1` - **Alert** (Immediate action must be taken, critical alert)
    /// - `2` - **Critical** (Critical conditions, serious failures)
    /// - `3` - **Error** (Error conditions, runtime errors)
    /// - `4` - **Warning** (Warning conditions, potential issues)
    /// - `5` - **Notice** (Normal but significant events, noteworthy conditions)
    /// - `6` - **Info** (Informational messages, general system events)
    /// - `7` - **Debug** (Debug-level messages, detailed diagnostic information)
    pub fn from_severity(severity: i8) -> Result<Self, Error> {
        match severity {
            0 => Ok(Self::Emergency),
            1 => Ok(Self::Alert),
            2 => Ok(Self::Critical),
            3 => Ok(Self::Error),
            4 => Ok(Self::Warning),
            5 => Ok(Self::Notice),
            6 => Ok(Self::Info),
            7 => Ok(Self::Debug),
            _ => Err(Error::InvalidSeverity(severity)),
        }
    }
}

/// Formats the `Level` as a lowercase string.
///
/// This representation is used for log output, serialization,
/// and debugging purposes.
impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Parses a `Level` from its string representation.
///
/// This is typically used when loading configuration, parsing
/// environment variables, or processing user input such as
/// "info", "debug", or "error".
impl FromStr for Level {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s)
    }
}

/// Defines partial ordering for `Level`.
///
/// This enables comparisons such as `level > other_level`.
/// The ordering is based on severity values where lower numbers
/// represent higher severity (more critical logs).
impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Defines total ordering for `Level` based on severity.
///
/// Levels are ordered so that more critical logs come first when sorted.
/// Internally, lower severity values represent higher priority, so the
/// comparison is reversed to achieve descending severity order.
impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.severity().cmp(&other.severity()).reverse()
    }
}