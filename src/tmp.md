// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
// #[wasm_bindgen]
// pub fn read_csv(file: web_sys::File) -> Result<String, JsValue> {
//     let reader = ReaderBuilder::new()
//         .has_headers(true)
//         .from_reader(file);
//     let mut records = Vec::new();