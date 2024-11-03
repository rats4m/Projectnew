//! The `data_serialization` module provides functions to serialize and deserialize `Event` data.
//! 
//! This module is responsible for converting `Event` instances to JSON format for storage or transmission,
//! and parsing JSON data back into `Event` structs for analysis.

use serde::{Serialize, Deserialize};
use serde_json;

/// Represents a cybersecurity event.
/// 
/// An `Event` contains metadata such as the timestamp, source, destination, 
/// and event type, along with optional data for additional details.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub timestamp: u64,
    pub source: String,
    pub destination: String,
    pub event_type: String,
    pub data: Option<String>,
}

/// Serializes an `Event` instance to a JSON string.
pub fn serialize_event(event: &Event) -> Result<String, serde_json::Error> {
    serde_json::to_string(event)
}

/// Deserializes a JSON string into an `Event` instance.
pub fn deserialize_event(data: &str) -> Result<Event, serde_json::Error> {
    serde_json::from_str(data)
}

