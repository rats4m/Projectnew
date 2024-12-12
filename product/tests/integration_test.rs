#[cfg(test)]
mod tests {
    use project::visualizer::real_time::visualize_anomalies;
    use std::collections::BTreeMap;

    #[test]
    fn test_visualize_anomalies_with_valid_data() {
        let data = vec![
            [("metric_value", "100.0")].iter().cloned().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            [("metric_value", "150.0")].iter().cloned().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        ];
        assert!(visualize_anomalies(data, "metric_value").is_ok());
    }

    #[test]
    fn test_visualize_anomalies_with_empty_data() {
        let data: Vec<BTreeMap<String, String>> = vec![];
        assert!(visualize_anomalies(data, "metric_value").is_err());
    }
}
