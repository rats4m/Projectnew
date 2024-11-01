use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    timestamp: u64,
    source: String,
    destination: String,
    event_type: String,
    data: Option<String>,
}

pub fn serialize_event(event: &Event) -> Result<String, serde_json::Error> {
    serde_json::to_string(event)
}

pub fn deserialize_event(data: &str) -> Result<Event, serde_json::Error> {
    serde_json::from_str(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let event = Event {
            timestamp: 1630456800,
            source: "192.168.1.1".to_string(),
            destination: "192.168.1.2".to_string(),
            event_type: "login".to_string(),
            data: Some("User logged in successfully".to_string()),
        };

        let serialized = serialize_event(&event).unwrap();
        let deserialized: Event = deserialize_event(&serialized).unwrap();

        assert_eq!(event.timestamp, deserialized.timestamp);
        assert_eq!(event.source, deserialized.source);
        assert_eq!(event.destination, deserialized.destination);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.data, deserialized.data);
    }
}
