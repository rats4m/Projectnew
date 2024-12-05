use std::collections::BTreeMap;

pub fn detect_anomalies(data: Vec<BTreeMap<String, String>>) -> Vec<BTreeMap<String, String>> {
    data.into_iter()
        .filter(|record| {
            record.get("event_type").map_or(false, |val| val == "suspicious")
        })
        .collect()
}
