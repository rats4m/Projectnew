use smartcore::cluster::kmeans::{KMeans, KMeansParameters};
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use std::collections::BTreeMap;

pub fn identify_anomalies(
    data: Vec<BTreeMap<String, String>>,
    threshold: f64,
    k: usize,
) -> Vec<BTreeMap<String, String>> {
    let numeric_data: Vec<Vec<f64>> = data
        .iter()
        .filter_map(|record| {
            record
                .get("metric_value")
                .and_then(|value| value.parse::<f64>().ok())
                .map(|v| vec![v])
        })
        .collect();

    if numeric_data.len() < k {
        eprintln!("Not enough data points for K-Means clustering");
        return vec![];
    }

    let matrix = DenseMatrix::from_2d_vec(&numeric_data);

    let kmeans = KMeans::fit(&matrix, KMeansParameters::default().with_k(k)).unwrap_or_else(|e| {
        eprintln!("K-Means clustering failed: {}", e);
        panic!("Clustering error");
    });

    let labels = kmeans.predict(&matrix).unwrap();

    let mut anomalies = Vec::new();

    for (index, record) in data.into_iter().enumerate() {
        if let Some(metric_value) = record.get("metric_value") {
            if let Ok(value) = metric_value.parse::<f64>() {
                let cluster_points: Vec<f64> = numeric_data
                    .iter()
                    .zip(labels.iter())
                    .filter_map(|(point, &label)| {
                        if label == labels[index] {
                            Some(point[0])
                        } else {
                            None
                        }
                    })
                    .collect();

                if !cluster_points.is_empty() {
                    let center: f64 =
                        cluster_points.iter().sum::<f64>() / cluster_points.len() as f64;
                    let distance = (value - center).abs();
                    if distance > threshold {
                        anomalies.push(record);
                    }
                }
            }
        }
    }

    anomalies
}
