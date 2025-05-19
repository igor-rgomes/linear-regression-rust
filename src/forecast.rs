pub fn forecast_values(x: &[f64], slope: f64, intercept: f64, future_periods: usize) -> Vec<f64> {
    let mut forecasts = Vec::new();
    let last_x = x.last().copied().unwrap_or(0.0);
    for i in 1..=future_periods {
        let future_x = last_x + i as f64;
        forecasts.push(slope * future_x + intercept);
    }
    forecasts
} 
