extern crate rezasm_core;
extern crate rezasm_macro;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use rezasm_web_core::util::commands::{
    get_exit_status, get_register_value, is_completed, load, reset, run, step, stop,
};

#[wasm_bindgen]
pub fn wasm_stop() {
    stop();
}

#[wasm_bindgen]
pub fn wasm_reset() {
    reset();
}

#[wasm_bindgen]
pub fn wasm_load(lines: &str) {
    load(lines);
}

#[wasm_bindgen]
pub fn wasm_run() {
    run();
}

#[wasm_bindgen]
pub fn wasm_step() {
    step();
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
