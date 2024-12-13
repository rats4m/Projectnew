mod data_ingestion;
mod ml_engine;
mod visualizer;
mod utils;

use data_ingestion::file_loader::load_csv;
use data_ingestion::preprocessor::{remove_duplicates, normalize_fields};
use ml_engine::anomaly_detection::detect_anomalies;
use visualizer::real_time::visualize_anomalies;
use utils::{logger::init_logger, error::CustomError};
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

fn run_pipeline() -> Result<(), CustomError> {
    let file_path = "product/datasets/sample.csv";

    let raw_data: Vec<BTreeMap<String, String>> = load_csv(file_path)
        .map_err(|e| CustomError::FileLoadError(e.to_string()))?
        .into_iter()
        .map(|record| record.into_iter().collect())
        .collect();

    let cleaned_data = normalize_fields(remove_duplicates(raw_data));

    let anomalies = detect_anomalies(cleaned_data, "metric_value", 50.0);

    visualize_anomalies(anomalies, "metric_value")
        .map_err(|e| CustomError::VisualizationError(e.to_string()))?;

    Ok(())
}
