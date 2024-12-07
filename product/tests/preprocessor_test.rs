#[cfg(test)]
mod tests {
    use project::data_ingestion::preprocessor::{
        filter_irrelevant_data,
        normalize_fields,
        remove_duplicates,
    };
    use std::collections::BTreeMap;

    #[test]
    fn test_remove_duplicates() {
        let data = vec![
            [
                ("key1", "value1"),
                ("key2", "value2"),
            ]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>(),
            [
                ("key1", "value1"),
                ("key2", "value2"),
            ]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>()
        ];
        let result = remove_duplicates(data);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_normalize_fields() {
        let data = vec![{
            let mut map = BTreeMap::new();
            map.insert("ip_address".to_string(), " 192.168.1.1 ".to_string());
            map.insert("timestamp".to_string(), "2023-12-01T10:00:00Z".to_string());
            map
        }];
        let result = normalize_fields(data);
        assert_eq!(result[0]["ip_address"], "192.168.1.1");
    }

    #[test]
    fn test_filter_irrelevant_data() {
        let data = vec![
            [("key1", "value1")]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>(),
            [("irrelevant_key", "value")]
                .iter()
                .cloned()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<String, String>>()
        ];
        let relevant_keys = vec!["key1".to_string()];
        let result = filter_irrelevant_data(data, relevant_keys);
        assert_eq!(result.len(), 1);
    }
}
