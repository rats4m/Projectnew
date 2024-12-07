use chrono;
use std::collections::{ BTreeMap, HashSet };

pub fn remove_duplicates(data: Vec<BTreeMap<String, String>>) -> Vec<BTreeMap<String, String>> {
    let mut seen = HashSet::new();
    data.into_iter()
        .filter(|record| seen.insert(record.clone()))
        .collect()
}

pub fn normalize_fields(mut data: Vec<BTreeMap<String, String>>) -> Vec<BTreeMap<String, String>> {
    for record in &mut data {
        if let Some(ip) = record.get_mut("ip_address") {
            *ip = ip.trim().to_lowercase();
        }
        if let Some(timestamp) = record.get_mut("timestamp") {
            *timestamp = normalize_timestamp(timestamp);
        }
    }
    data
}

fn normalize_timestamp(timestamp: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => dt.to_rfc3339(),
        Err(_) => timestamp.to_string(),
    }
}

pub fn filter_irrelevant_data(
    data: Vec<BTreeMap<String, String>>,
    relevant_keys: Vec<String>
) -> Vec<BTreeMap<String, String>> {
    data.into_iter()
        .filter(|record| record.keys().any(|key| relevant_keys.contains(key)))
        .collect()
}
