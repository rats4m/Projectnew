#[cfg(test)]
mod tests {
    use project::visualizer::real_time::visualize_anomalies;
    use std::collections::BTreeMap;

    #[test]
    fn test_visualize_anomalies_interactive() {
        let mut record1 = BTreeMap::new();
        record1.insert("value".to_string(), "1.0".to_string());

        let mut record2 = BTreeMap::new();
        record2.insert("value".to_string(), "2.0".to_string());

        let mut record3 = BTreeMap::new();
        record3.insert("value".to_string(), "10.0".to_string());

        let data = vec![record1, record2, record3];

        let result = visualize_anomalies(data, "value", 5.0);
        assert!(result.is_ok(), "The visualization function should not return an error.");
    }
}
