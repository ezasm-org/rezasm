extern crate rezasm_core;
extern crate rezasm_web_core;
extern crate serde_wasm_bindgen;
extern crate wasm_bindgen;

use rezasm_core::instructions::implementation::register_instructions;
use rezasm_web_core::{
    get_exit_status, get_memory_bounds, get_memory_slice, get_register_names, get_register_value,
    get_register_values, get_word_size, is_completed, load, receive_input, reset, step, stop,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn eval(command: &str);
}

#[wasm_bindgen]
pub fn wasm_stop() {
    stop();
}

#[wasm_bindgen]
pub fn wasm_reset() {
    reset();
}

#[wasm_bindgen]
pub fn wasm_load(lines: &str) -> Result<(), String> {
    load(lines)
}

#[wasm_bindgen]
pub fn wasm_step() -> Result<(), String> {
    step()
}

#[wasm_bindgen]
pub fn wasm_is_completed() -> bool {
    is_completed()
}

#[wasm_bindgen]
pub fn wasm_get_exit_status() -> i64 {
    get_exit_status()
}

#[wasm_bindgen]
pub fn wasm_get_register_value(register: &str) -> Option<i64> {
    get_register_value(register)
}

#[wasm_bindgen]
pub fn wasm_get_register_names() -> JsValue {
    serde_wasm_bindgen::to_value(&get_register_names()).unwrap()
}

#[wasm_bindgen]
pub fn wasm_get_register_values() -> Vec<i64> {
    get_register_values()
}

#[wasm_bindgen]
pub fn wasm_get_memory_bounds() -> JsValue {
    serde_wasm_bindgen::to_value(&get_memory_bounds()).unwrap()
}

#[wasm_bindgen]
pub fn wasm_get_memory_slice(address: usize, length: usize) -> Result<Vec<i64>, String> {
    get_memory_slice(address, length)
}

#[wasm_bindgen]
pub fn wasm_get_word_size() -> usize {
    get_word_size()
}

#[wasm_bindgen]
pub fn wasm_receive_input(data: &str) {
    receive_input(data);
}

#[wasm_bindgen(start)]
pub fn wasm_initialize_backend() {
    register_instructions();
}
