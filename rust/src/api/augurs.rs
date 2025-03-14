use augurs::{
    ets::AutoETS,
    forecaster::{
        transforms::{LinearInterpolator, Logit, MinMaxScaler},
        Forecaster, Transformer,
    },
    mstl::MSTLModel,
};
use flutter_rust_bridge::frb;
use serde_json::json;

use super::utils::*;

#[frb]
pub fn augurs_forecaster(csv_data: Vec<u8>, frequency: String) -> std::string::String {
    let sales = read_sales_data(csv_data);
    let n_frequency = get_frequency(frequency);

    let data = sales.as_slice();

    let ets = AutoETS::non_seasonal().into_trend_model();
    let mstl = MSTLModel::new(vec![n_frequency], ets);

    // Set up the transformers.
    let transformers = vec![
        LinearInterpolator::new().boxed(),
        MinMaxScaler::new().boxed(),
        Logit::new().boxed(),
    ];

    // Create a forecaster using the transforms.
    let mut forecaster = Forecaster::new(mstl).with_transformers(transformers);

    // Fit the data into the forecaster through the MSTL model
    forecaster.fit(&data).expect("model should fit");

    // Generate a limited number of predictions (X amount) to focus on short-term accuracy.
    // Short-term predictions are preferred to prevent large deviations
    let in_sample = forecaster
        .predict(3, 0.95)
        .expect("in-sample predictions should work");

    let intervals = in_sample.intervals.map(|forecast| {
        (
            format!("{:?}", forecast.level),
            format!("{:?}", forecast.upper),
            format!("{:?}", forecast.lower),
        )
    });

    let (confidence, upper, lower) =
        intervals.unwrap_or(("None".to_string(), "None".to_string(), "None".to_string()));

    json!({
        "predictions": in_sample.point,
        "confidence": confidence,
        "upper": upper,
        "lower": lower,
    })
    .to_string()
}
