#[macro_export]
macro_rules! info {
    ($msg:expr $(, $ctx:expr)?) => {
        $crate::LogRecord::new(
            "default",
            $crate::Level::Info,
            $msg,
            serde_json::json!({ $( "context": $ctx )? }),
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr $(, $ctx:expr)?) => {
        $crate::LogRecord::new(
            "default",
            $crate::Level::Warning,
            $msg,
            serde_json::json!({ $( "context": $ctx )? }),
        )
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr $(, $ctx:expr)?) => {
        $crate::LogRecord::new(
            "default",
            $crate::Level::Error,
            $msg,
            serde_json::json!({ $( "context": $ctx )? }),
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr $(, $ctx:expr)?) => {
        $crate::LogRecord::new(
            "default",
            $crate::Level::Debug,
            $msg,
            serde_json::json!({ $( "context": $ctx )? }),
        )
    };
}
