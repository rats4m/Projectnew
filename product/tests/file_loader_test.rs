#[cfg(test)]
mod tests {
    use project::data_ingestion::file_loader::{ load_csv, load_json };
    use std::fs;

    const TEST_CSV_PATH: &str = "test.csv";
    const TEST_JSON_PATH: &str = "test.json";

    const TEST_CSV_CONTENT: &str = "header1,header2\nvalue1,value2";
    const TEST_JSON_CONTENT: &str = r#"[{"key1": "value1", "key2": "value2"}]"#;

    #[test]
    fn test_load_csv() {
        fs::write(TEST_CSV_PATH, TEST_CSV_CONTENT).expect("Failed to create test CSV file.");

        let result = load_csv(TEST_CSV_PATH);
        fs::remove_file(TEST_CSV_PATH).expect("Failed to remove test CSV file.");

        assert!(result.is_ok(), "CSV parsing failed.");
        let records = result.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0]["header1"], "value1");
    }

    #[test]
    fn test_load_json() {
        fs::write(TEST_JSON_PATH, TEST_JSON_CONTENT).expect("Failed to create test JSON file.");

        let result = load_json(TEST_JSON_PATH);
        fs::remove_file(TEST_JSON_PATH).expect("Failed to remove test JSON file.");

        assert!(result.is_ok(), "JSON parsing failed.");
        let records = result.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0]["key1"], "value1");
    }
}
