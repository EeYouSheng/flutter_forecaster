use arima::{acf, estimate, sim};
use flutter_rust_bridge::frb;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use serde_json::json;

use super::utils::*;

#[frb]
pub fn predict_sales(csv_data: Vec<u8>) -> std::string::String {
    let sales = read_sales_data(csv_data);

    /// Get Sales Data Distribution
    let mean_sales = sales.iter().sum::<f64>() / sales.len() as f64;
    let std_sales =
        (sales.iter().map(|x| (x - mean_sales).powi(2)).sum::<f64>() / sales.len() as f64).sqrt();
    let normalized_sales: Vec<f64> = sales.iter().map(|x| (x - mean_sales) / std_sales).collect();

    /// Set Maximum Lag for large dataset
    let max_lag = Some(if sales.len() > 1600 {
        40
    } else {
        (sales.len() as f64).sqrt().round() as usize // Use sqrt(n) otherwise
    });
    let acf_values = acf::acf(&sales, max_lag, false).unwrap();

    let cov0 = acf::acf(&sales, Some(0), true).unwrap()[0];

    let pacf_values = acf::pacf_rho_cov0(&acf_values, cov0, max_lag).unwrap();

    let mut p = 0;
    let mut q = 0;
    let d = if acf_values[1].abs() > 0.5 { 1 } else { 0 };

    let threshold = 1.96 / (sales.len() as f64).sqrt();

    for (i, &val) in pacf_values.iter().enumerate().skip(1) {
        if val.abs() < threshold {
            break;
        }
        p = i;
    }

    // Step 4: Estimate q (MA order) where ACF cuts off
    for (i, &val) in acf_values.iter().enumerate().skip(1) {
        if val.abs() < threshold {
            break;
        }
        q = i;
    }

    let coef = estimate::fit(&normalized_sales, p, d, q).unwrap();

    let intercept = coef[0]; // First coefficient is intercept
    let phi = if p > 0 { Some(&coef[1..(1 + p)]) } else { None };
    let theta = if q > 0 {
        Some(&coef[(1 + p)..(1 + p + q)])
    } else {
        None
    };

    let residuals = estimate::residuals(&normalized_sales, intercept, phi, theta).unwrap();

    let normal = Normal::new(0.0, 0.0).unwrap();

    let predictions = sim::arima_forecast(
        &normalized_sales,
        1,
        phi,
        theta,
        1,
        &|_, rng| normal.sample(rng),
        &mut thread_rng(),
    )
    .unwrap();

    let denormalized_predictions: Vec<f64> = predictions
        .iter()
        .map(|x| x * std_sales + mean_sales)
        .collect();

    json!({
        "p": p,
        "q": q,
        "ac": acf_values,
        "pacf": pacf_values,
        "coef": coef,
        "phi": phi,
        "theta": theta,
        "residual": residuals,
        "predictions": denormalized_predictions,
    })
    .to_string()
}
