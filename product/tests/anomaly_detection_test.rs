#[cfg(test)]
mod tests {
    use project::ml_engine::anomaly_detection::detect_anomalies;
    use std::collections::BTreeMap;

    #[test]
    fn test_valid_anomalies() {
        let data = vec![
            [("metric_value", "60.0")]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            [("metric_value", "30.0")]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        ];
        let threshold = 50.0;
        let result = detect_anomalies(data, "metric_value", threshold);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("metric_value").unwrap(), "60.0");
    }

    #[test]
    fn test_non_numeric_values() {
        let data = vec![
            [("metric_value", "not_a_number")]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            [("metric_value", "25.0")]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        ];
        let threshold = 20.0;
        let result = detect_anomalies(data, "metric_value", threshold);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("metric_value").unwrap(), "25.0");
    }

    #[test]
    fn test_missing_keys() {
        let data = vec![[("some_key", "100.0")]
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()];
        let threshold = 50.0;
        let result = detect_anomalies(data, "metric_value", threshold);
        assert!(result.is_empty());
    }
}
