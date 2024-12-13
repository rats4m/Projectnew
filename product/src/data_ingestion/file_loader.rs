use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use log::{error, info};
use serde_json::Value;

/// Loads a CSV file into a vector of BTreeMap (key-value pairs).
pub fn load_csv(file_path: &str) -> Result<Vec<BTreeMap<String, String>>, Box<dyn std::error::Error>> {
    info!("Attempting to load CSV file: {}", file_path);

    let file = File::open(file_path).map_err(|e| {
        error!("Failed to open CSV file: {}", e);
        e
    })?;
    let mut records = Vec::new();

    let mut lines = io::BufReader::new(file).lines();
    if let Some(header_line) = lines.next() {
        let headers: Vec<String> = header_line?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        if headers.is_empty() {
            return Err("CSV file has an empty header line".into());
        }

        for line in lines {
            let line = line?;
            let values: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();

            if values.len() == headers.len() {
                let record: BTreeMap<String, String> = headers
                    .iter()
                    .cloned()
                    .zip(values.into_iter())
                    .collect();
                records.push(record);
            } else {
                error!("Mismatched header and value count in line: {}", line);
            }
        }

        info!("Successfully loaded {} records from CSV.", records.len());
    } else {
        error!("CSV file is empty: {}", file_path);
        return Err("CSV file is empty".into());
    }

    Ok(records)
}

/// Loads a JSON file into a vector of BTreeMap (key-value pairs).
pub fn load_json(file_path: &str) -> Result<Vec<BTreeMap<String, String>>, Box<dyn std::error::Error>> {
    info!("Attempting to load JSON file: {}", file_path);

    let file = File::open(file_path).map_err(|e| {
        error!("Failed to open JSON file: {}", e);
        e
    })?;
    let data: Value = serde_json::from_reader(file).map_err(|e| {
        error!("Failed to parse JSON file: {}", e);
        e
    })?;

    if let Some(array) = data.as_array() {
        let records: Vec<BTreeMap<String, String>> = array
            .iter()
            .filter_map(|item| {
                item.as_object().map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.to_string()))
                        .collect()
                })
            })
            .collect();

        info!("Successfully loaded {} records from JSON.", records.len());
        Ok(records)
    } else {
        error!("JSON file does not contain a valid array: {}", file_path);
        Err("JSON file does not contain a valid array".into())
    }
}
