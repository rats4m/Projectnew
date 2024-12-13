use std::collections::BTreeMap;
use log::{info, warn, error};

/// Detect anomalies based on a simple threshold.
/// Returns a vector of records marked as anomalies.
pub fn detect_anomalies(
    data: &Vec<BTreeMap<String, String>>,
    value_key: &str,
    threshold: f64,
) -> Vec<BTreeMap<String, String>> {
    info!("Starting anomaly detection on key '{}' with threshold {}", value_key, threshold);

    if data.is_empty() {
        warn!("No data provided for anomaly detection.");
        return Vec::new();
    }

    let anomalies: Vec<BTreeMap<String, String>> = data
        .iter()
        .filter(|record| {
            match record.get(value_key) {
                Some(value) => match value.parse::<f64>() {
                    Ok(parsed_value) => parsed_value > threshold,
                    Err(_) => {
                        warn!(
                            "Value '{}' under key '{}' is not a valid number. Record: {:?}",
                            value, value_key, record
                        );
                        false
                    }
                },
                None => {
                    warn!(
                        "Key '{}' not found in record: {:?}",
                        value_key, record
                    );
                    false
                }
            }
        })
        .cloned()
        .collect();

    info!(
        "Anomaly detection completed. Total anomalies detected: {}",
        anomalies.len()
    );

    anomalies
}
