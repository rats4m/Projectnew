use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashMap;
use serde_json::Value;
use csv::ReaderBuilder;

pub fn load_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open CSV file: {}", e))?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut data = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| format!("Failed to read CSV record: {}", e))?;
        let row: HashMap<String, String> = record
            .iter()
            .enumerate()
            .map(|(i, value)| (reader.headers().unwrap()[i].to_string(), value.to_string()))
            .collect();
        data.push(row);
    }

    Ok(data)
}

pub fn load_json(file_path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open JSON file: {}", e))?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)
        .map_err(|e| format!("Failed to parse JSON file: {}", e))?;

    if let Value::Array(entries) = json {
        let data = entries
            .iter()
            .filter_map(|entry| {
                if let Value::Object(obj) = entry {
                    Some(
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.to_string()))
                            .collect::<HashMap<String, String>>(),
                    )
                } else {
                    None
                }
            })
            .collect();
        Ok(data)
    } else {
        Err("JSON file must contain an array of objects".to_string())
    }
}
