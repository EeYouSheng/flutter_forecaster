use std::io::Cursor;
use csv::ReaderBuilder;

pub fn read_sales_data(csv_data: Vec<u8>) -> Vec<f64>{
    // Parse CSV data
    let mut converted_csv = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(csv_data));

    // Convert Parsed Csv Data to Vector List
    let mut sales: Vec<f64> = Vec::new();
    for result in converted_csv.records() {
        let record = result.unwrap();
        let amount: f64 = record[1].parse().unwrap();
        sales.push(amount);
    }

    sales
}

pub fn get_frequency(frequency: String) -> usize{
    // Get Forecast Value
    let n_forecast = match frequency.as_str() {
        "Weekly" => 52,   // Predict next week
        "Monthly" => 12, // Predict next month
        _ => panic!("Invalid frequency provided"),
    };

    n_forecast
}