use std::collections::BTreeMap;

/// Detect anomalies based on a simple threshold.
/// Returns a vector of records marked as anomalies.
pub fn detect_anomalies(
    data: &Vec<BTreeMap<String, String>>,
    value_key: &str,
    threshold: f64,
) -> Vec<BTreeMap<String, String>> {
    data.iter()
        .filter(|record| {
            record
                .get(value_key)
                .and_then(|value| value.parse::<f64>().ok())
                .map_or(false, |v| v > threshold)
        })
        .cloned()
        .collect()
}
