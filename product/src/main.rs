use std::collections::BTreeMap;

use data_ingestion::{file_loader, preprocessor};
use ml_engine::anomaly_detection;
use visualizer::real_time;
use utils::{logger, error::PipelineError};

mod data_ingestion;
mod ml_engine;
mod visualizer;
mod utils;

fn main() -> Result<(), PipelineError> {
    // Initialize logging
    logger::init_logger();

    // Step 1: Load data from CSV
    log::info!("Loading data from CSV file...");
    let mut data = file_loader::load_csv("product/datasets/dataset.csv")
        .map_err(|e| PipelineError::FileLoadError(format!("Failed to load CSV: {}", e)))?;

    log::info!("Data successfully loaded. Total records: {}", data.len());

    // Step 2: Preprocess the data
    log::info!("Preprocessing data...");
    preprocessor::normalize_timestamps(&mut data, "timestamp", "%Y-%m-%d %H:%M:%S");
    preprocessor::remove_duplicates(&mut data, "id");
    preprocessor::validate_records(&mut data, &["id", "timestamp", "value"]);
    log::info!("Preprocessing completed. Remaining records: {}", data.len());

    // Step 3: Detect anomalies
    log::info!("Detecting anomalies...");
    let threshold = 10.0; // Example threshold
    let anomalies = anomaly_detection::detect_anomalies(&data, "value", threshold);

    if anomalies.is_empty() {
        log::warn!("No anomalies detected.");
    } else {
        log::info!("Detected {} anomalies.", anomalies.len());
    }

    // Step 4: Visualize anomalies in real-time
    log::info!("Visualizing anomalies...");
    real_time::visualize_anomalies(data, "value", threshold)
        .map_err(|e| PipelineError::DetectionError(format!("Visualization error: {}", e)))?;

    log::info!("Processing and visualization completed successfully.");
    Ok(())
}
