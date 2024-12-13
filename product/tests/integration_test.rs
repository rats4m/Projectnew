#[cfg(test)]
mod tests {
    use project::utils::error::CustomError;
    use project::visualizer::real_time::visualize_anomalies;
    use std::collections::BTreeMap;
    use project::data_ingestion::file_loader::load_csv;
    use project::data_ingestion::preprocessor::{remove_duplicates, normalize_fields};
    use project::ml_engine::anomaly_detection::detect_anomalies;

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

    #[test]
    fn test_pipeline_integration() -> Result<(), CustomError> {
        let file_path = "datasets/sample.csv";
        let raw_data: Vec<BTreeMap<String, String>> = load_csv(file_path)
            .map_err(|e| CustomError::FileLoadError(e.to_string()))?
            .into_iter()
            .map(|record| record.into_iter().collect())
            .collect();
        let cleaned_data = normalize_fields(remove_duplicates(raw_data));
        let anomalies = detect_anomalies(cleaned_data, "metric_value", 50.0);
        assert!(visualize_anomalies(anomalies, "metric_value").is_ok());
        Ok(())
    }
}
