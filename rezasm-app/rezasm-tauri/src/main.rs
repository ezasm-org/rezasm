// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tauri_writer;

extern crate lazy_static;
extern crate tauri;

use lazy_static::lazy_static;
use rezasm_core::instructions::implementation::register_instructions;
use rezasm_web_core::{
    get_exit_status, get_memory_bounds, get_memory_slice, get_register_names, get_register_value,
    get_register_values, get_word_size, is_completed, load, receive_input, register_writer, reset,
    step, stop,
};
use tauri::{Manager, Window};

use crate::tauri_writer::TauriWriter;
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
fn tauri_step() -> Result<(), String> {
    step()
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

#[tauri::command]
fn tauri_get_register_names() -> Vec<String> {
    get_register_names()
}

#[tauri::command]
fn tauri_get_register_values() -> Vec<i64> {
    get_register_values()
}

#[tauri::command]
fn tauri_get_memory_bounds() -> (usize, usize, usize) {
    get_memory_bounds()
}

#[tauri::command]
fn tauri_get_memory_slice(address: usize, length: usize) -> Result<Vec<i64>, String> {
    get_memory_slice(address, length)
}

#[tauri::command]
fn tauri_get_word_size() -> usize {
    get_word_size()
}

#[tauri::command]
fn tauri_receive_input(data: &str) {
    receive_input(data);
}

fn main() {
    register_instructions();
    register_writer(Box::new(TauriWriter::new()));

    tauri::Builder::default()
        .setup(|app| Ok(set_window(app.get_window(WINDOW_NAME).unwrap())))
        .invoke_handler(tauri::generate_handler![
            tauri_load,
            tauri_reset,
            tauri_step,
            tauri_stop,
            tauri_is_completed,
            tauri_get_exit_status,
            tauri_get_register_value,
            tauri_get_register_names,
            tauri_get_register_values,
            tauri_get_memory_bounds,
            tauri_get_memory_slice,
            tauri_get_word_size,
            tauri_receive_input,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
