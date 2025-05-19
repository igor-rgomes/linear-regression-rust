pub fn calculate_r2(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let mean_y: f64 = y_true.iter().sum::<f64>() / y_true.len() as f64;
    let ss_tot: f64 = y_true.iter().map(|y| (y - mean_y).powi(2)).sum();
    let ss_res: f64 = y_true.iter().zip(y_pred.iter()).map(|(y, y_hat)| (y - y_hat).powi(2)).sum();
    1.0 - ss_res / ss_tot
}

pub fn calculate_mse(y_true: &[f64], y_pred: &[f64]) -> f64 {
    y_true.iter().zip(y_pred.iter()).map(|(y, y_hat)| (y - y_hat).powi(2)).sum::<f64>() / y_true.len() as f64
}
 
