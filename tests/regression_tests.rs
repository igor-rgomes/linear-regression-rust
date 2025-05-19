use linear_regression_rust::regression::linear_regression;
use linear_regression_rust::metrics::{calculate_r2, calculate_mse};

#[test]
fn test_regression_coefficients() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 4.0, 6.0];
    let (slope, intercept) = linear_regression(&x, &y).unwrap();
    assert!((slope - 2.0).abs() < 1e-6);
    assert!((intercept).abs() < 1e-6);
}