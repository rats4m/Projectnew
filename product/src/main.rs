use std::collections::BTreeMap;

use data_ingestion::{file_loader, preprocessor};
use ml_engine::anomaly_detection;

mod data_ingestion;
mod ml_engine;
mod visualizer;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data from a CSV file
    let mut data = file_loader::load_csv("data.csv")?;

    // Preprocess the data
    preprocessor::normalize_timestamps(&mut data, "timestamp", "%Y-%m-%d %H:%M:%S");
    preprocessor::remove_duplicates(&mut data, "id");
    preprocessor::validate_records(&mut data, &["id", "timestamp", "value"]);

    // Detect anomalies
    let anomalies = anomaly_detection::detect_anomalies(&data, "value", 10.0);

    // Display anomalies
    println!("Detected anomalies:");
    for anomaly in anomalies {
        println!("{:?}", anomaly);
    }

    Ok(())
}
