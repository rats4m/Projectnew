use cybersecurity_visualization::backend::data_serialization::{serialize_event, deserialize_event, Event};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_event() {
        let event = Event {
            timestamp: 1627846261,
            source: "192.168.1.1".to_string(),
            destination: "192.168.1.2".to_string(),
            event_type: "login".to_string(),
            data: Some("User logged in successfully".to_string()),
        };

        let serialized = serialize_event(&event).unwrap();
        eprintln!("Serialized Event JSON: {}", serialized);

        assert!(serialized.contains("\"timestamp\":1627846261"));
        eprintln!("Timestamp check passed.");
        
        assert!(serialized.contains("\"source\":\"192.168.1.1\""));
        eprintln!("Source check passed.");
        
        assert!(serialized.contains("\"destination\":\"192.168.1.2\""));
        eprintln!("Destination check passed.");
        
        assert!(serialized.contains("\"event_type\":\"login\""));
        eprintln!("Event type check passed.");
        
        assert!(serialized.contains("\"data\":\"User logged in successfully\""));
        eprintln!("Data field check passed.");
    }
}
