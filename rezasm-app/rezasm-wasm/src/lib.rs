extern crate rezasm_core;
extern crate rezasm_macro;
extern crate rezasm_web_core;
extern crate wasm_bindgen;

use rezasm_instructions::register_instructions;
use rezasm_web_core::{get_exit_status, get_register_value, is_completed, load, reset, step, stop};
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

#[wasm_bindgen(start)]
pub fn wasm_initialize_backend() {
    register_instructions();
}
