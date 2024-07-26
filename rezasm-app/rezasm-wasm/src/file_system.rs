use wasm_bindgen::prelude::wasm_bindgen;

type StringResult<T> = Result<T, String>;

#[wasm_bindgen]
pub fn wasm_copy_file(_from: &str, _to: &str) -> StringResult<u64> { todo!() }

#[wasm_bindgen]
pub fn wasm_read_to_string(_path: &str) -> StringResult<String> { todo!() }

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

#[wasm_bindgen]
pub fn wasm_write_file(_path: &str, _contents: &str) { todo!() }
