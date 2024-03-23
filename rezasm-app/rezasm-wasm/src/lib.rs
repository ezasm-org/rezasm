mod wasm_writer;

extern crate rezasm_core;
extern crate rezasm_web_core;
extern crate serde_wasm_bindgen;
extern crate wasm_bindgen;

use crate::wasm_writer::WasmWriter;
use rezasm_core::instructions::implementation::register_instructions;
use rezasm_web_core::{
    get_exit_status, get_memory_bounds, get_memory_slice, get_register_names, get_register_value,
    get_register_values, get_simulator_mut, get_word_size, initialize_simulator, is_completed, load,
    reset, step, stop
};
use wasm_bindgen::prelude::*;
use rezasm_core::simulation::reader::DummyReader;

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
    let mut simulator = get_simulator_mut();
    let reader = simulator.get_reader_mut();
    // TODO make wasm_reader and expand buffer for it, then use it here
    let _ = reader.as_any_mut().downcast_mut::<DummyReader>().unwrap();
    let _ = data;
}

#[wasm_bindgen(start)]
pub fn wasm_initialize_backend() {
    register_instructions();
    initialize_simulator(Some(Box::new(DummyReader::new())), Some(Box::new(WasmWriter::new())));
}
