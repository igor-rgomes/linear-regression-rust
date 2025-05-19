mod regression;
mod metrics;
mod forecast;

use regression::linear_regression;
use metrics::{calculate_r2, calculate_mse};
use forecast::forecast_values;

fn main() {
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();

    let (slope, intercept) = linear_regression(&x, &y).unwrap();
    println!("Coeficientes: slope = {}, intercept = {}", slope, intercept);

    let predictions: Vec<f64> = x.iter().map(|xi| slope * xi + intercept).collect();
    let r2 = calculate_r2(&y, &predictions);
    let mse = calculate_mse(&y, &predictions);

    println!("R²: {:.4}", r2);
    println!("MSE: {:.4}", mse);

    let future_preds = forecast_values(&x, slope, intercept, 3);
    println!("Previsões futuras: {:?}", future_preds);
}