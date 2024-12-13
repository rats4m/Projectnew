#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use project::ml_engine::anomaly_detection::detect_anomalies;

    use super::*;

    #[test]
    fn test_detect_anomalies() {
        let data = vec![
            BTreeMap::from([("value".to_string(), "5.0".to_string())]),
            BTreeMap::from([("value".to_string(), "10.0".to_string())]),
            BTreeMap::from([("value".to_string(), "2.0".to_string())]),
        ];
        let anomalies = detect_anomalies(&data, "value", 4.0);
        assert_eq!(anomalies.len(), 2);
        assert_eq!(anomalies[0]["value"], "5.0");
        assert_eq!(anomalies[1]["value"], "10.0");
    }
}