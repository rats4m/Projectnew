#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use project::data_ingestion::preprocessor::{normalize_timestamps, remove_duplicates, validate_records};

    use super::*;

    #[test]
    fn test_normalize_timestamps() {
        let mut data = vec![
            BTreeMap::from([("timestamp".to_string(), "2024-12-13 10:00:00".to_string())]),
        ];
        normalize_timestamps(&mut data, "timestamp", "%Y-%m-%d %H:%M:%S");
        assert_eq!(data[0]["timestamp"], "2024-12-13 10:00:00");
    }

    #[test]
    fn test_remove_duplicates() {
        let mut data = vec![
            BTreeMap::from([("id".to_string(), "1".to_string())]),
            BTreeMap::from([("id".to_string(), "1".to_string())]),
            BTreeMap::from([("id".to_string(), "2".to_string())]),
        ];
        remove_duplicates(&mut data, "id");
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_validate_records() {
        let mut data = vec![
            BTreeMap::from([("key1".to_string(), "value1".to_string())]),
            BTreeMap::from([("key2".to_string(), "value2".to_string())]),
        ];
        validate_records(&mut data, &["key1"]);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0]["key1"], "value1");
    }
}