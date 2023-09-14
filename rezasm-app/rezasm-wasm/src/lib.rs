extern crate js_sys;
extern crate lazy_static;
extern crate rezasm_core;
extern crate rezasm_macro;
extern crate rezasm_web_core;
extern crate tokio;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

use lazy_static::lazy_static;
use rezasm_instructions::register_instructions;
use rezasm_web_core::util::commands::{
    get_exit_status, get_register_value, get_simulator, is_completed, load, register_callbacks,
    reset, run, step, stop,
};
use wasm_bindgen::prelude::*;

use js_sys::Promise;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use wasm_bindgen_futures::future_to_promise;

lazy_static! {
    static ref RESULT: Arc<RwLock<Option<Result<i64, String>>>> = Arc::new(RwLock::new(None));
    static ref IS_RUNNING: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
}

#[wasm_bindgen]
extern "C" {
    fn eval(command: &str);
}

fn get_result() -> RwLockReadGuard<'static, Option<Result<i64, String>>> {
    RESULT.read().unwrap()
}

fn set_result(result: Option<Result<i64, String>>) {
    *RESULT.write().unwrap() = result;
}

fn set_is_running() {
    *IS_RUNNING.write().unwrap() = true;
}

fn reset_is_running() {
    *IS_RUNNING.write().unwrap() = false;
}

fn get_is_running() -> bool {
    *IS_RUNNING.read().unwrap()
}

fn signal_error(error: &str) {
    set_result(Some(Err(error.to_string())));
    reset_is_running();
}

fn signal_program_completion(exit_code: i64) {
    set_result(Some(Ok(exit_code)));
    reset_is_running();
}

fn signal_termination() {
    set_result(None);
    reset_is_running();
}

fn handle_program_completion() {
    let result: &Option<Result<i64, String>> = &*get_result();
    let _ = match result {
        None => js_sys::eval("window.errorCallback(\"Program terminated forcefully\")"),
        Some(r) => match r {
            Ok(exit_code) => {
                js_sys::eval(format!("window.programCompletionCallback({})", exit_code).as_str())
            }
            Err(error) => js_sys::eval(format!("window.errorCallback(\"{}\")", error).as_str()),
        },
    };
}

#[wasm_bindgen]
pub fn wasm_stop() {
    reset_is_running();
    stop();
}

#[wasm_bindgen]
pub fn wasm_reset() {
    reset_is_running();
    reset();
}

#[wasm_bindgen]
pub fn wasm_load(lines: &str) -> Result<(), String> {
    load(lines)
}

#[wasm_bindgen]
pub fn wasm_completion_callback() {
    if get_is_running() {
        let window = web_sys::window().expect("could not get window");
        let _ = window
            .set_timeout_with_str_and_timeout_and_unused_0("window.wasm_completion_callback()", 50);
    } else if get_simulator().is_done() || get_simulator().is_error() {
        handle_program_completion();
    }
}

#[wasm_bindgen]
pub fn wasm_run() -> Promise {
    set_is_running();
    future_to_promise(async {
        run();
        wasm_completion_callback();
        Ok(JsValue::from(0))
    })
}

#[wasm_bindgen]
pub fn wasm_step() -> Promise {
    future_to_promise(async {
        step();
        wasm_completion_callback();
        Ok(JsValue::from(0))
    })
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
    register_callbacks(signal_error, signal_program_completion, signal_termination);
}
