use serde::{Deserialize, Serialize};
use serde_json;

pub struct Event {
    timestamp: u64,
    source: String,
    destination: String,
    event_type: String,
    data: Option<String>, //May be needed, but can be option (for additional information)
}

pub fn serialize_event(event: &Event) -> Result<String, serde_json::Error> {
    serde_json::to_string(event)
}

pub fn deserialize_event(data: &str) -> Result<Event, serde_json::Error> {
    serde_json::from_str(data)
}