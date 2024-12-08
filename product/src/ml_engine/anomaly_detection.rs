use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::cluster::kmeans::{ KMeans, KMeansParameters };
use std::collections::HashSet;

pub struct AnomalyDetectionResult {
    pub anomalies: HashSet<usize>,
    pub cluster_labels: Vec<usize>,
}

pub fn detect_anomalies(
    data: Vec<Vec<f64>>,
    num_clusters: usize,
    threshold: f64
) -> Result<AnomalyDetectionResult, String> {
    if data.is_empty() || data[0].is_empty() {
        return Err("Input data must not be empty or malformed.".to_string());
    }

    let rows = data.len();
    let cols = data[0].len();
    let flat_data: Vec<f64> = data.into_iter().flatten().collect();
    let matrix = DenseMatrix::from_array(rows, cols, &flat_data);

    let kmeans = KMeans::fit(&matrix, KMeansParameters::default().with_k(num_clusters)).map_err(|e|
        format!("KMeans fitting failed: {}", e)
    )?;

    let cluster_labels_f64 = kmeans
        .predict(&matrix)
        .map_err(|e| format!("Prediction failed: {}", e))?;

    let cluster_labels: Vec<usize> = cluster_labels_f64
        .into_iter()
        .map(|label| label as usize)
        .collect();

    let mut centroids = vec![vec![0.0; cols]; num_clusters];
    let mut cluster_sizes = vec![0; num_clusters];

    for (index, &label) in cluster_labels.iter().enumerate() {
        let row_start = index * cols;
        let row_end = row_start + cols;
        let row = &flat_data[row_start..row_end];

        for (j, &value) in row.iter().enumerate() {
            centroids[label][j] += value;
        }
        cluster_sizes[label] += 1;
    }

    for (centroid, &size) in centroids.iter_mut().zip(cluster_sizes.iter()) {
        if size > 0 {
            for value in centroid.iter_mut() {
                *value /= size as f64;
            }
        }
    }

    let mut anomalies = HashSet::new();

    for (index, &label) in cluster_labels.iter().enumerate() {
        let row_start = index * cols;
        let row_end = row_start + cols;
        let row = &flat_data[row_start..row_end];
        let centroid = &centroids[label];
        let distance: f64 = row
            .iter()
            .zip(centroid.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum();

        if distance.sqrt() > threshold {
            anomalies.insert(index);
        }
    }

    Ok(AnomalyDetectionResult {
        anomalies,
        cluster_labels,
    })
}
