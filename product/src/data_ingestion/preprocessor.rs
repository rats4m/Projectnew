use std::collections::BTreeMap;
use chrono::NaiveDateTime;

/// Normalize timestamps to a standard format.
pub fn normalize_timestamps(
    data: &mut Vec<BTreeMap<String, String>>,
    timestamp_key: &str,
    format: &str,
) {
    for record in data.iter_mut() {
        if let Some(timestamp) = record.get(timestamp_key) {
            if let Ok(parsed) = NaiveDateTime::parse_from_str(timestamp, format) {
                record.insert(timestamp_key.to_string(), parsed.to_string());
            }
        }
    }
}

/// Remove duplicate records based on a unique key.
pub fn remove_duplicates(data: &mut Vec<BTreeMap<String, String>>, unique_key: &str) {
    let mut seen = std::collections::HashSet::new();
    data.retain(|record| {
        if let Some(value) = record.get(unique_key) {
            seen.insert(value.clone())
        } else {
            true
        }
    });
}

/// Validate records and remove those with missing required keys.
pub fn validate_records(data: &mut Vec<BTreeMap<String, String>>, required_keys: &[&str]) {
    data.retain(|record| {
        required_keys.iter().all(|&key| record.contains_key(key))
    });
}
