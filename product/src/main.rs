mod data_ingestion;
mod ml_engine;
mod visualizer;
mod utils;

use data_ingestion::file_loader::load_csv;
use data_ingestion::preprocessor::{remove_duplicates, normalize_fields};
use ml_engine::anomaly_detection::detect_anomalies;
use visualizer::real_time::visualize_anomalies;
use utils::{logger::init_logger, error::CustomError};
use std::io::BufRead;
use std::{thread, time};
use std::collections::BTreeMap;
use std::process;

fn main() {
    init_logger();

    match run_pipeline() {
        Ok(_) => log::info!("Pipeline executed successfully."),
        Err(e) => {
            log::error!("Pipeline execution failed: {}", e);
            process::exit(1);
        }
    }
}

fn calculate_statistics(data: &[BTreeMap<String, String>], key: &str, threshold: f64) {
    let total_records = data.len();
    let anomalies: Vec<_> = data
        .iter()
        .filter(|record| record.get(key).and_then(|v| v.parse::<f64>().ok()).map_or(false, |v| v > threshold))
        .collect();
    let num_anomalies = anomalies.len();
    let anomaly_percentage = (num_anomalies as f64 / total_records as f64) * 100.0;

    println!("--- Statistics ---");
    println!("Total Records: {}", total_records);
    println!("Anomalies Detected: {} ({:.2}%)", num_anomalies, anomaly_percentage);
    if let Some(min) = data
        .iter()
        .filter_map(|record| record.get(key).and_then(|v| v.parse::<f64>().ok()))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
    {
        println!("Minimum Value: {:.2}", min);
    }
    if let Some(max) = data
        .iter()
        .filter_map(|record| record.get(key).and_then(|v| v.parse::<f64>().ok()))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
    {
        println!("Maximum Value: {:.2}", max);
    }
}

fn simulate_real_time(file_path: &str) {
    let file = std::fs::File::open(file_path).expect("Failed to open sample CSV");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(record) => {
                println!("Processing: {}", record);
                // Simulate processing delay
                thread::sleep(time::Duration::from_millis(500));
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}


fn run_pipeline() -> Result<(), CustomError> {
    let file_path = "product/datasets/sample.csv";

    println!("Starting real-time simulation...");
    simulate_real_time(file_path);

    let raw_data = load_csv(file_path)
        .map_err(|e| CustomError::FileLoadError(e.to_string()))?;

    let raw_data: Vec<BTreeMap<String, String>> = raw_data
        .into_iter()
        .map(|record| record.into_iter().collect())
        .collect();
    let cleaned_data = normalize_fields(remove_duplicates(raw_data));

    let threshold = 50.0;
    let anomalies = detect_anomalies(cleaned_data.clone(), "metric_value", threshold);

    calculate_statistics(&cleaned_data, "metric_value", threshold);

    visualize_anomalies(anomalies, "metric_value", threshold)
        .map_err(|e| CustomError::VisualizationError(e.to_string()))?;

    Ok(())
}

