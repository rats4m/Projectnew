pub mod integration_test;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use project::ml_engine::anomaly_detection::detect_anomalies;

    #[test]
    fn test_detect_anomalies() {
        let mut record1 = BTreeMap::new();
        record1.insert("event_type".to_string(), "normal".to_string());

        let mut record2 = BTreeMap::new();
        record2.insert("event_type".to_string(), "suspicious".to_string());

        let data = vec![record1, record2];
        let anomalies = detect_anomalies(data);

        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0].get("event_type").unwrap(), "suspicious");
    }
}
