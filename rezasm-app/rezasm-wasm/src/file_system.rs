use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn wasm_copy(_from: &str, _to: &str) -> u64 { todo!() }

#[wasm_bindgen]
pub fn wasm_read_to_string(_path: &str) -> String { todo!() }

#[wasm_bindgen]
pub fn wasm_create_dir(_path: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_read_dir(_path: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_create_dir_with_parents(_path: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_create_file(_path: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_remove_file(_path: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_rename(_from: &str, _to: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_remove_dir(_path: &str) { todo!() }

#[wasm_bindgen]
pub fn wasm_remove_dir_recursive(_path: &str) { todo!() }
