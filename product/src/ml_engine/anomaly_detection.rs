use std::collections::BTreeMap;

pub fn identify_anomalies(
    data: Vec<BTreeMap<String, String>>,
    threshold: f64
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

pub fn identify_anomalies_percentile(
    data: Vec<BTreeMap<String, String>>,
    percentile: f64
) -> Vec<BTreeMap<String, String>> {
    let mut metric_values: Vec<f64> = data
        .iter()
        .filter_map(|record| record.get("metric_value").and_then(|v| v.parse().ok()))
        .collect();

    if metric_values.is_empty() {
        return Vec::new();
    }

    metric_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let index = ((percentile / 100.0) * ((metric_values.len() as f64) - 1.0)).round() as usize;
    let threshold = metric_values[index];

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
