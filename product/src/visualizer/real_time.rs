use plotly::{Plot, Scatter};
use plotly::common::{Mode, Marker};
use std::collections::BTreeMap;
use log::{info, warn, error};

/// Visualize anomalies using Plotly.
/// Creates a line graph with anomalies highlighted as points.
pub fn visualize_anomalies(
    data: Vec<BTreeMap<String, String>>,
    key: &str,
    threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Starting visualization. Key: '{}', Threshold: {}",
        key, threshold
    );

    if data.is_empty() {
        warn!("No data available for visualization.");
        return Err("No data to visualize".into());
    }

    let values: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .filter_map(|(i, record)| {
            record.get(key).and_then(|value| value.parse::<f64>().ok()).map(|v| (i as f64, v))
        })
        .collect();

    if values.is_empty() {
        warn!("No valid numeric data found for key '{}'.", key);
        return Err(format!("No valid data to visualize for key '{}'", key).into());
    }

    let (x, y): (Vec<f64>, Vec<f64>) = values.iter().cloned().unzip();

    let anomalies: Vec<(f64, f64)> = values
        .iter()
        .cloned()
        .filter(|&(_, v)| v > threshold)
        .collect();

    let (anomaly_x, anomaly_y): (Vec<f64>, Vec<f64>) = anomalies.iter().cloned().unzip();

    // Log anomaly details
    if anomalies.is_empty() {
        info!("No anomalies detected above the threshold.");
    } else {
        info!(
            "Detected {} anomalies above the threshold. Highlighting in visualization.",
            anomalies.len()
        );
    }

    let line_trace = Scatter::new(x.clone(), y.clone())
        .mode(Mode::LinesMarkers)
        .name("Data Points")
        .marker(Marker::new().color("blue"));

    let anomaly_trace = Scatter::new(anomaly_x.clone(), anomaly_y.clone())
        .mode(Mode::Markers)
        .name("Anomalies")
        .marker(Marker::new().color("red").size(10));

    let mut plot = Plot::new();
    plot.add_trace(line_trace);
    plot.add_trace(anomaly_trace);

    plot.show(); // Open in browser for interactivity

    info!("Visualization completed successfully.");
    Ok(())
}
