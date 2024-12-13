use plotters::prelude::*;
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

    let root_area = BitMapBackend::new("anomalies_graph.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let (min_value, max_value) = values.iter().fold((f64::MAX, f64::MIN), |(min, max), &(_, v)| {
        (min.min(v), max.max(v))
    });

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Anomaly Detection Graph", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..values.len() as f64, min_value..max_value)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        values.iter().map(|&(x, y)| (x, y)),
        &BLUE,
    ))?;

    chart.draw_series(values.iter().filter_map(|&(x, y)| {
        if y > threshold {
            Some(Circle::new((x, y), 5, RED.filled()))
        } else {
            None
        }
    }))?
    .label("Anomalies")
    .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
