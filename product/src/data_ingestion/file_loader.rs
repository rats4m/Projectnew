use csv::ReaderBuilder;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn load_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Error opening CSV file at '{}': {}", file_path, e))?;
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let headers = csv_reader
        .headers()
        .map_err(|e| format!("Error reading CSV headers: {}", e))?
        .clone();

    let mut records = Vec::new();
    for csv_record in csv_reader.records() {
        let record = csv_record.map_err(|e| format!("Error reading CSV record: {}", e))?;
        let parsed_record: HashMap<String, String> = record
            .iter()
            .enumerate()
            .map(|(i, value)| (headers[i].to_string(), value.to_string()))
            .collect();
        records.push(parsed_record);
    }
    Ok(records)
}

pub fn load_json(file_path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Error opening JSON file at '{}': {}", file_path, e))?;
    let reader = BufReader::new(file);

    let parsed_json: Value = serde_json::from_reader(reader)
        .map_err(|e| format!("Error parsing JSON file at '{}': {}", file_path, e))?;

    if let Value::Array(entries) = parsed_json {
        let records = entries
            .iter()
            .filter_map(|entry| {
                if let Value::Object(obj) = entry {
                    Some(
                        obj.iter()
                            .filter_map(|(key, value)| {
                                if let Value::String(text) = value {
                                    Some((key.clone(), text.clone()))
                                } else {
                                    None
                                }
                            })
                            .collect::<HashMap<String, String>>(),
                    )
                } else {
                    None
                }
            })
            .collect();
        Ok(records)
    } else {
        Err("Expected JSON to contain an array of objects.".to_string())
    }
}
