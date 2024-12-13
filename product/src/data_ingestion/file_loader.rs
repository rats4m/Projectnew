use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use serde_json::Value;

/// Loads a CSV file into a vector of BTreeMap (key-value pairs).
pub fn load_csv(file_path: &str) -> Result<Vec<BTreeMap<String, String>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut records = Vec::new();

    let mut lines = io::BufReader::new(file).lines();
    if let Some(header_line) = lines.next() {
        let headers: Vec<String> = header_line?.split(',').map(|s| s.trim().to_string()).collect();

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
            }
        }
    }

    Ok(records)
}

/// Loads a JSON file into a vector of BTreeMap (key-value pairs).
pub fn load_json(file_path: &str) -> Result<Vec<BTreeMap<String, String>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let data: Value = serde_json::from_reader(file)?;

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
        Ok(records)
    } else {
        Err("JSON file does not contain an array".into())
    }
}
