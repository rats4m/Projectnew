#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use project::data_ingestion::preprocessor::{remove_duplicates, normalize_fields, filter_irrelevant_data};
    use project::ml_engine::anomaly_detection::detect_anomalies;

    #[test]
    fn test_preprocessing_to_anomaly_detection() {
        let raw_data = vec![
            {
                let mut record = BTreeMap::new();
                record.insert("event_type".to_string(), "suspicious".to_string());
                record
            },
            {
                let mut record = BTreeMap::new();
                record.insert("event_type".to_string(), "normal".to_string());
                record
            },
        ];

        let preprocessed_data = filter_irrelevant_data(
            normalize_fields(remove_duplicates(raw_data)),
            vec!["event_type".to_string()],
        );

        let anomalies = detect_anomalies(preprocessed_data);

        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0].get("event_type").unwrap(), "suspicious");
    }
}
