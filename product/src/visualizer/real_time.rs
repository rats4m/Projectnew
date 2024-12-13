use plotly::{Plot, Scatter};
use plotly::common::{Mode, Marker};
use std::collections::BTreeMap;

pub fn visualize_anomalies(
    data: Vec<BTreeMap<String, String>>,
    key: &str,
    threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .filter_map(|(i, record)| {
            record.get(key).and_then(|value| value.parse::<f64>().ok()).map(|v| (i as f64, v))
        })
        .collect();

    if values.is_empty() {
        eprintln!("No valid data to visualize.");
        return Err("No valid data".into());
    }

    let (x, y): (Vec<f64>, Vec<f64>) = values.iter().cloned().unzip();

    let anomalies: Vec<(f64, f64)> = values
        .iter()
        .cloned()
        .filter(|&(_, v)| v > threshold)
        .collect();

    let (anomaly_x, anomaly_y): (Vec<f64>, Vec<f64>) = anomalies.iter().cloned().unzip();

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

    plot.show(); // Opens in browser for interactivity

    Ok(())
}
