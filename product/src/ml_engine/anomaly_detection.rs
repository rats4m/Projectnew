use std::collections::BTreeMap;

pub fn identify_anomalies(
    data: Vec<BTreeMap<String, String>>,
    threshold: f64,
) -> Vec<BTreeMap<String, String>> {
    data.into_iter()
        .filter(|record| {
            if let Some(value) = record.get("metric_value") {
                value.parse::<f64>().map_or(false, |v| v > threshold)
            } else {
                false
            }
        })
        .collect()
}
