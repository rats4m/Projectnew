#[cfg(test)]
mod tests {
    use project::data_ingestion::file_loader::{load_csv, load_json};

    use super::*;
    use std::fs::write;

    #[test]
    fn test_load_csv() {
        let test_csv = "header1,header2\nvalue1,value2\nvalue3,value4";
        let file_path = "test.csv";
        write(file_path, test_csv).unwrap();

        let result = load_csv(file_path);
        assert!(result.is_ok());

        let records = result.unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["header1"], "value1");
        assert_eq!(records[1]["header2"], "value4");
    }

    #[test]
    fn test_load_json() {
        let test_json = "[{\"key1\": \"value1\", \"key2\": \"value2\"}]";
        let file_path = "test.json";
        write(file_path, test_json).unwrap();

        let result = load_json(file_path);
        assert!(result.is_ok());

        let records = result.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0]["key1"], "value1");
    }
}