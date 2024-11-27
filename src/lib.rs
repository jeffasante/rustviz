// src/lib.rs
use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use js_sys::{ Array, Object };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct RegressionResult {
    slope: f64,
    intercept: f64,
    r_squared: f64,
    points: Vec<(f64, f64)>,
}

// TODO: Add more analysis just like `pandas.info()`
#[derive(Serialize, Deserialize)]
pub struct AnalysisResult {}

#[derive(Serialize, Deserialize)]
pub struct ColumnStats {
    mean: f64,
    median: f64,
    std_dev: f64,
    min: f64,
    max: f64,
}

/*
  Basic CSV parsing, returns JavaScript objects
*/

#[wasm_bindgen]
pub fn parse_csv(csv_content: &str) -> Result<JsValue, JsValue> {
    // creatre a csv reader
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(csv_content.as_bytes());

    // get headers
    let headers = reader
        .headers()
        .map_err(|e| JsValue::from_str(&format!("Error reading headers: {}", e)))?
        .clone();

    // create an array to store all rows
    let results = Array::new();

    // process each record
    for result in reader.records() {
        let record = result.map_err(|e|
            JsValue::from_str(&format!("Error reading record: {}", e))
        )?;

        // create a JavaScript object for each row
        let row = Object::new();

        // add each field to the row object
        for (i, field) in record.iter().enumerate() {
            let header = headers
                .get(i)
                .ok_or_else(|| JsValue::from_str("Header index out of boundsr"))?;
            js_sys::Reflect
                ::set(&row, &JsValue::from_str(header), &JsValue::from_str(field))
                .unwrap();
        }
        results.push(&row);
    }
    Ok(results.into())
}

// Extract numerical column from csv
#[wasm_bindgen]
pub fn get_column_values(csv_content: &str, column_name: &str) -> Result<Vec<f64>, JsValue> {
    let mut reader = csv::Reader::from_reader(csv_content.as_bytes());
    let headers = match reader.headers() {
        Ok(headers) => headers,
        Err(err) => return Err(JsValue::from_str(&format!("Error reading CSV headers: {}", err))),
    };

    let column_index = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or_else(|| JsValue::from_str("Column not found"))?;

    let values: Vec<f64> = reader
        .records()
        .map(|record| {
            record
                .map_err(|e| JsValue::from_str(&e.to_string()))
                .and_then(|row| {
                    row.get(column_index)
                        .ok_or_else(|| JsValue::from_str("Invalid record"))
                        .and_then(|val| {
                            val.parse::<f64>()
                                .map_err(|e| JsValue::from_str(&e.to_string()))
                        })
                })
        })
        .collect::<Result<Vec<f64>, JsValue>>()?;

    Ok(values)
}


/*
Basic statistics for a single column
*/
#[wasm_bindgen]
pub fn calculate_stats(values: &[f64]) -> Result<JsValue, JsValue> {
    let n = values.len() as f64;
    let sum: f64 = values.iter().sum();
    let mean = sum / n;

    // lets sort the values
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if values.len() % 2 == 0 {
        (sorted[values.len() / 2 - 1] + sorted[values.len() / 2]) / 2.0
    } else {
        sorted[values.len() / 2]
    };

    // Sample standard deviation with Bessel's correction:: Reference: https://en.wikipedia.org/wiki/Bessel%27s_correction
    let std_dev =
        values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() /
        (n - 1.0) // partially corrects the bias in the estimation of the population standard deviation.
            .sqrt();

    let stats = ColumnStats {
        mean,
        median,
        std_dev,
        min: *sorted.first().unwrap(),
        max: *sorted.last().unwrap(),
    };

    Ok(serde_wasm_bindgen::to_value(&stats)?)
}

/*
Perform linear regression analysis on two arrays
*/
#[wasm_bindgen]
pub fn linear_regression(x_values: &[f64], y_values: &[f64]) -> Result<JsValue, JsValue> {
    if x_values.len() != y_values.len() {
        return Err(JsValue::from_str("X and Y arrays must have the same length"));
    }

    // calculate the slope and intercept
    let n = x_values.len() as f64;
    let sum_x: f64 = x_values.iter().sum();
    let sum_y: f64 = y_values.iter().sum();
    let sum_xy: f64 = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(x, y)| x * y)
        .sum();
    let sum_xx: f64 = x_values
        .iter()
        .map(|x| x * x)
        .sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    let mean_y = sum_y / n;
    let ss_tot: f64 = y_values
        .iter()
        .map(|y| (y - mean_y).powi(2))
        .sum();
    let ss_res: f64 = y_values
        .iter()
        .zip(x_values.iter())
        .map(|(y, x)| (y - slope * x - intercept).powi(2))
        .sum();

    let r_squared = 1.0 - ss_res / ss_tot;

    let points: Vec<(f64, f64)> = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(&x, &y)| (x, y))
        .collect();

    let result = RegressionResult {
        slope,
        intercept,
        r_squared,
        points,
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

// All-in-one analysis function
/*
Combines all analyses into one convenience function
*/
#[wasm_bindgen]
pub fn analyze_data(x_col: &str, y_column: &str, csv_content: &str) -> Result<JsValue, JsValue> {
    let x_values = get_column_values(csv_content, x_col)?;
    let y_values = get_column_values(csv_content, y_column)?;

    let x_stats = calculate_stats(&x_values)?;
    let y_stats = calculate_stats(&y_values)?;
    let regression = linear_regression(&x_values, &y_values)?;

    let result = Object::new();
    js_sys::Reflect::set(&result, &"x_stats".into(), &x_stats)?;
    js_sys::Reflect::set(&result, &"y_stats".into(), &y_stats)?;
    js_sys::Reflect::set(&result, &"regression".into(), &regression)?;

    Ok(result.into())
}
