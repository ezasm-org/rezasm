extern crate rezasm_core;
extern crate rezasm_macro;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use rezasm_web_core::util::commands::{
    get_exit_status, get_register_value, is_completed, load, register_callbacks, reset, run, step,
    stop,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = errorCallback)]
    fn error_callback(error: &str);

    #[wasm_bindgen(js_namespace = window, js_name = programCompletionCallback)]
    fn program_completion_callback(error: &str);
}

fn signal_error(error: &str) {
    error_callback(error);
}

fn signal_program_completion(exit_status: i64) {
    program_completion_callback(format!("{}", exit_status).as_str());
}

#[wasm_bindgen]
pub fn wasm_initialize_backend() {
    register_callbacks(signal_error, signal_program_completion);
}

#[wasm_bindgen]
pub fn wasm_stop() {
    stop()
}

#[wasm_bindgen]
pub fn wasm_reset() {
    reset()
}

#[wasm_bindgen]
pub fn wasm_load(lines: &str) -> Result<(), String> {
    load(lines)
}

#[wasm_bindgen]
pub fn wasm_run() {
    run()
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
