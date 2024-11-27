// src/lib.rs
use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use js_sys::{ Array, Object };

#[wasm_bindgen]
pub fn parse_csv(csv_content: &str) -> Result<JsValue, JsValue> {
    // creatre a csv reader
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(csv_content.as_bytes());

    // get headers
    // let headers = reader.headers().map_err(|e| JsValue::from_str(&format!("Error reading headers: {}", &e.to_string()))?;
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
