mod data_ingestion;
mod ml_engine;
mod visualizer;
mod utils;

use std::collections::BTreeMap;

use data_ingestion::file_loader::load_csv;
use data_ingestion::preprocessor::{remove_duplicates, normalize_fields};
use ml_engine::anomaly_detection::detect_anomalies;
use visualizer::real_time::visualize_anomalies;
use utils::logger::init_logger;

fn main() {
    init_logger();

    let file_path = "datasets/sample.csv";

    let raw_data = match load_csv(file_path) {
        Ok(data) => data,
        Err(e) => {
            log::error!("Error loading CSV: {}", e);
            return;
        }
    };

    let raw_data: Vec<BTreeMap<String, String>> = raw_data.into_iter()
        .map(|record| record.into_iter().collect())
        .collect();
    let cleaned_data = normalize_fields(remove_duplicates(raw_data));

    let anomalies = detect_anomalies(cleaned_data.clone(), "metric_value", 50.0);

    if let Err(e) = visualize_anomalies(anomalies.clone(), "metric_value") {
        log::error!("Visualization error: {}", e);
    } else {
        log::info!("Pipeline executed successfully");
    }
}
