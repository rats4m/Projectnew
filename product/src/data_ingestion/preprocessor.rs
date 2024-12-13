use std::collections::{BTreeMap, HashSet};
use chrono::{NaiveDateTime, ParseError};
use log::{info, warn, error};

/// Normalize timestamps to a standard format.
pub fn normalize_timestamps(
    data: &mut Vec<BTreeMap<String, String>>,
    timestamp_key: &str,
    format: &str,
) {
    info!("Starting timestamp normalization using format: {}", format);
    for record in data.iter_mut() {
        if let Some(timestamp) = record.get(timestamp_key) {
            match NaiveDateTime::parse_from_str(timestamp, format) {
                Ok(parsed) => {
                    record.insert(timestamp_key.to_string(), parsed.to_string());
                }
                Err(e) => {
                    warn!(
                        "Failed to parse timestamp '{}' in record: {:?} | Error: {}",
                        timestamp, record, e
                    );
                }
            }
        } else {
            warn!(
                "Record missing timestamp key '{}': {:?}",
                timestamp_key, record
            );
        }
    }
    info!("Timestamp normalization completed.");
}

/// Remove duplicate records based on a unique key.
pub fn remove_duplicates(data: &mut Vec<BTreeMap<String, String>>, unique_key: &str) {
    info!("Starting duplicate removal based on key: {}", unique_key);
    let mut seen = HashSet::new();
    let initial_len = data.len();

    data.retain(|record| {
        if let Some(value) = record.get(unique_key) {
            if seen.contains(value) {
                warn!("Duplicate record found and removed: {:?}", record);
                false
            } else {
                seen.insert(value.clone());
                true
            }
        } else {
            warn!(
                "Record missing unique key '{}': {:?}",
                unique_key, record
            );
            false
        }
    });

    let removed = initial_len - data.len();
    info!("Duplicate removal completed. Records removed: {}", removed);
}

/// Validate records and remove those with missing required keys.
pub fn validate_records(data: &mut Vec<BTreeMap<String, String>>, required_keys: &[&str]) {
    info!("Starting record validation for required keys: {:?}", required_keys);
    let initial_len = data.len();

    data.retain(|record| {
        let missing_keys: Vec<&str> = required_keys
            .iter()
            .filter(|&&key| !record.contains_key(key))
            .cloned()
            .collect();

        if !missing_keys.is_empty() {
            warn!(
                "Record missing required keys {:?}: {:?}",
                missing_keys, record
            );
            false
        } else {
            true
        }
    });

    let removed = initial_len - data.len();
    info!("Record validation completed. Invalid records removed: {}", removed);
}
