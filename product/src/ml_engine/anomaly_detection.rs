use std::collections::BTreeMap;

pub fn detect_anomalies(
    data: Vec<BTreeMap<String, String>>,
    key: &str,
    threshold: f64,
) -> Vec<BTreeMap<String, String>> {
    let mut anomalies = Vec::new();

    for record in data {
        if let Some(value) = record.get(key) {
            match value.parse::<f64>() {
                Ok(parsed_value) if parsed_value > threshold => {
                    anomalies.push(record.clone());
                    log::info!(
                        "Anomaly detected: {:?} exceeds threshold {}",
                        record,
                        threshold
                    );
                }
                Ok(_) => continue,
                Err(_) => log::warn!("Non-numeric value encountered for key '{}': {}", key, value),
            }
        } else {
            log::warn!("Missing key '{}' in record: {:?}", key, record);
        }
    }
    anomalies
}
