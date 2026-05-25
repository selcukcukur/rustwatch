use chrono::Utc;
use chrono_tz::UTC;
use rustwatch::{Level, Record};
use serde_json::{Value, json};

fn now() -> chrono::DateTime<chrono_tz::Tz> {
    Utc::now().with_timezone(&UTC)
}

fn base_record() -> Record {
    Record::new(Level::Info, "Init", "main", None, now())
}

#[test]
fn record_defaults() {
    let ts = now();
    let record = Record::new(Level::Info, "Hello", "main", None, ts);

    assert_eq!(record.level, Level::Info);
    assert_eq!(record.message, "Hello");
    assert_eq!(record.channel, "main");
    assert_eq!(record.context, json!({}));
    assert_eq!(record.timestamp, ts);
    assert!(record.formatted.is_none());
}

#[test]
fn record_with_context() {
    let ts = now();

    let record = Record::new(
        Level::Error,
        "Boom",
        "network",
        Some(json!({"ip": "192.168.1.1"})),
        ts,
    );

    assert_eq!(record.context, json!({"ip": "192.168.1.1"}));
    assert_eq!(record.level, Level::Error);
}

#[test]
fn custom_level_without_severity() {
    let record = Record::new(
        Level::Custom("CustomLevel".to_string(), None),
        "msg",
        "channel",
        None,
        now(),
    );

    match record.level {
        Level::Custom(ref name, None) => assert_eq!(name, "CustomLevel"),
        _ => panic!("invalid level"),
    }

    assert_eq!(record.severity(), -1);
}

#[test]
fn custom_level_with_severity() {
    let record = Record::new(
        Level::Custom("CustomLevel".to_string(), Some(42)),
        "msg",
        "channel",
        None,
        now(),
    );

    assert_eq!(record.severity(), 42);

    match record.level {
        Level::Custom(_, Some(sev)) => assert_eq!(sev, 42),
        _ => panic!("invalid level"),
    }
}

#[test]
fn getters_work_correctly() {
    let record = base_record();

    assert!(matches!(record.level(), Level::Info));
    assert_eq!(record.message(), "Init");
    assert_eq!(record.channel(), "main");
    assert_eq!(record.context(), &json!({}));
}

#[test]
fn to_array_outputs_expected_fields() {
    let record = Record::new(Level::Warning, "Warning", "auth", None, now());

    let arr = record.to_array();

    assert_eq!(arr.get("message").unwrap(), "Warning");
    assert_eq!(arr.get("level").unwrap(), "warning");
    assert_eq!(arr.get("channel").unwrap(), "auth");
}

#[test]
fn to_json_contains_core_fields() {
    let record = Record::new(Level::Warning, "Warning", "auth", None, now());

    let json_str = record.to_json();

    let parsed: Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["message"], "Warning");
    assert_eq!(parsed["level"], "warning");
}

#[test]
fn with_overrides_only_updates_selected_fields() {
    let record = base_record();

    let updated = record.with(
        Some(Level::Error),
        Some("Updated"),
        Some("other"),
        Some(json!({"key": "value"})),
        None,
        Some("formatted"),
    );

    assert_eq!(updated.level, Level::Error);
    assert_eq!(updated.message, "Updated");
    assert_eq!(updated.channel, "other");
    assert_eq!(updated.context, json!({"key": "value"}));
    assert_eq!(updated.formatted, Some("formatted".to_string()));
}

#[test]
fn empty_context_is_normalized_to_object() {
    let record = Record::new(Level::Info, "msg", "main", None, now());

    assert_eq!(record.context, json!({}));
}

#[test]
fn explicit_empty_json_context_is_preserved() {
    let record = Record::new(Level::Info, "msg", "main", Some(json!({})), now());

    assert_eq!(record.context, json!({}));
}

#[test]
fn original_record_is_immutable_after_with() {
    let record = base_record();

    let cloned = record.clone();

    let _updated = record.with(Some(Level::Error), Some("changed"), None, None, None, None);

    assert_eq!(record.message, cloned.message);
    assert_eq!(record.level, cloned.level);
}

#[test]
fn with_updates_only_message() {
    let record = base_record();

    let updated = record.with(None, Some("new message"), None, None, None, None);

    assert_eq!(updated.message, "new message");
    assert_eq!(updated.level, record.level);
    assert_eq!(updated.channel, record.channel);
}

#[test]
fn with_updates_only_level() {
    let record = base_record();

    let updated = record.with(Some(Level::Error), None, None, None, None, None);

    assert_eq!(updated.level, Level::Error);
    assert_eq!(updated.message, record.message);
}

#[test]
fn custom_level_name_is_lowercased_in_json() {
    let record = Record::new(
        Level::Custom("MyLevel".to_string(), None),
        "msg",
        "main",
        None,
        now(),
    );

    let json: serde_json::Value = serde_json::from_str(&record.to_json()).unwrap();

    assert_eq!(json["level"], "mylevel");
}

#[test]
fn severity_defaults_to_negative_one() {
    let record = Record::new(
        Level::Custom("X".to_string(), None),
        "msg",
        "main",
        None,
        now(),
    );

    assert_eq!(record.severity(), -1);
}

#[test]
fn json_contains_required_fields() {
    let record = Record::new(Level::Info, "msg", "main", None, now());

    let json: serde_json::Value = serde_json::from_str(&record.to_json()).unwrap();

    assert!(json.get("message").is_some());
    assert!(json.get("level").is_some());
    assert!(json.get("channel").is_some());
    assert!(json.get("timestamp").is_some());
}

#[test]
fn formatted_field_survives_with_update() {
    let record = base_record();

    let updated = record.with(None, None, None, None, None, Some("formatted output"));

    assert_eq!(updated.formatted, Some("formatted output".to_string()));
}

#[test]
fn timestamp_is_never_modified_by_with() {
    let record = base_record();

    let ts = record.timestamp;

    let updated = record.with(Some(Level::Error), None, None, None, None, None);

    assert_eq!(updated.timestamp, ts);
}

#[test]
fn json_roundtrip_does_not_panic() {
    let record = Record::new(
        Level::Error,
        "critical failure",
        "system",
        Some(json!({"code": 500})),
        now(),
    );

    let json_str = record.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["message"], "critical failure");
}

#[test]
fn empty_channel_is_accepted() {
    let record = Record::new(Level::Info, "msg", "", None, now());

    assert_eq!(record.channel, "");
}

#[test]
fn whitespace_channel_is_preserved() {
    let record = Record::new(Level::Info, "msg", "   ", None, now());

    assert_eq!(record.channel, "   ");
}

#[test]
fn multiline_message_is_preserved() {
    let msg = "line1\nline2\nline3";

    let record = Record::new(Level::Info, msg, "main", None, now());

    assert_eq!(record.message, msg);
}

#[test]
fn unicode_message_is_preserved() {
    let msg = "🚀 rustwatch ✨ logs";

    let record = Record::new(Level::Info, msg, "main", None, now());

    assert_eq!(record.message, msg);
}

#[test]
fn json_handles_special_characters() {
    let record = Record::new(Level::Info, "line1\nline2", "main", None, now());

    let json_str = record.to_json();

    assert!(json_str.contains("\\n"));
}

#[test]
fn custom_level_empty_string() {
    let record = Record::new(
        Level::Custom("".to_string(), None),
        "msg",
        "main",
        None,
        now(),
    );

    match record.level {
        Level::Custom(name, _) => assert_eq!(name, ""),
        _ => panic!("expected custom"),
    }
}

#[test]
fn custom_level_long_name() {
    let name = "x".repeat(500);

    let record = Record::new(
        Level::Custom(name.clone(), None),
        "msg",
        "main",
        None,
        now(),
    );

    match record.level {
        Level::Custom(n, _) => assert_eq!(n.len(), 500),
        _ => panic!(),
    }
}

#[test]
fn array_and_json_are_consistent() {
    let record = Record::new(Level::Warning, "msg", "auth", None, now());

    let arr = record.to_array();
    let json: serde_json::Value = serde_json::from_str(&record.to_json()).unwrap();

    assert_eq!(arr.get("message").unwrap(), &json["message"]);
    assert_eq!(arr.get("level").unwrap(), &json["level"]);
    assert_eq!(arr.get("channel").unwrap(), &json["channel"]);
}
