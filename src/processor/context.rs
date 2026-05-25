use super::Processor;
use crate::Record;
use serde_json::json;

/// A processor that enriches a log record with a single key/value pair
/// inside its JSON context.
///
/// Useful for attaching metadata such as user IDs, request IDs,
/// environment details, or any other diagnostic information.
///
/// # Fields
/// - `key` - The context key to insert
/// - `value` - The context value to insert
pub struct ContextProcessor {
    pub key: String,
    pub value: String,
}

impl Processor for ContextProcessor {
    fn process(&self, record: &mut Record) {
        let mut ctx = if record.context.is_null() {
            json!({})
        } else {
            record.context.clone()
        };

        if let Some(obj) = ctx.as_object_mut() {
            obj.insert(self.key.clone(), json!(self.value));
        }

        record.context = ctx;
    }
}
