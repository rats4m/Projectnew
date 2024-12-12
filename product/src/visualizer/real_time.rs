use plotters::chart::ChartBuilder;
use plotters::prelude::*;
use plotters::style::colors::RED;
use std::collections::BTreeMap;

pub fn visualize_anomalies(
    data: Vec<BTreeMap<String, String>>,
    key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<f64> = data
        .iter()
        .filter_map(|record| record.get(key))
        .filter_map(|value| value.parse::<f64>().ok())
        .collect();

    if values.is_empty() {
        log::warn!(
            "No numeric values found for visualization under key '{}'",
            key
        );
        return Err("No valid data to visualize".into());
    }

    let root_area = BitMapBackend::new("anomalies.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let max_value = *values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_value = *values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Anomaly Visualization", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..values.len() as u32, min_value..max_value)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        values.iter().enumerate().map(|(i, &v)| (i as u32, v)),
        &RED,
    ))?;

    log::info!("Anomalies visualized and saved to anomalies.png");

    Ok(())
}
