// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate lazy_static;
extern crate tauri;
extern crate tokio;

use lazy_static::lazy_static;
use rezasm_instructions::register_instructions;
use rezasm_web_core::util::commands::{
    get_exit_status, get_register_value, get_runtime, initialize_runtime, is_completed, load,
    reset, step, stop,
};
use tauri::{Manager, Window};
use tokio::runtime;

use std::sync::{Arc, RwLock};

lazy_static! {
    static ref WINDOW: Arc<RwLock<Option<Window>>> = Arc::new(RwLock::new(None));
}

pub const WINDOW_NAME: &'static str = "main";

pub fn get_window() -> Window {
    WINDOW
        .write()
        .unwrap()
        .as_ref()
        .unwrap()
        .get_window(WINDOW_NAME)
        .unwrap()
}

pub fn set_window(window: Window) {
    let _ = WINDOW.write().unwrap().replace(window);
}

#[tauri::command]
fn tauri_stop() {
    stop()
}

#[tauri::command]
fn tauri_reset() {
    reset()
}

#[tauri::command]
fn tauri_load(lines: &str) -> Result<(), String> {
    load(lines)
}

#[tauri::command()]
fn tauri_step() {
    get_runtime().call(async { step() });
}

#[tauri::command]
fn tauri_is_completed() -> bool {
    is_completed()
}

#[tauri::command]
fn tauri_get_exit_status() -> i64 {
    get_exit_status()
}

#[tauri::command]
fn tauri_get_register_value(register: &str) -> Option<i64> {
    get_register_value(register)
}

fn main() {
    let rt = runtime::Builder::new_multi_thread().build().unwrap();

    rt.block_on(async {
        tauri::async_runtime::set(runtime::Handle::current());
    });

    register_instructions();
    initialize_runtime(rt);

    tauri::Builder::default()
        .setup(|app| Ok(set_window(app.get_window(WINDOW_NAME).unwrap())))
        .invoke_handler(tauri::generate_handler![
            tauri_load,
            tauri_reset,
            tauri_step,
            tauri_stop,
            tauri_is_completed,
            tauri_get_exit_status,
            tauri_get_register_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
