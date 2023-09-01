// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate rezasm_core;
extern crate rezasm_macro;
extern crate rezasm_app;

use rezasm_app::instructions::implementation::arithmetic_instructions::register_instructions;
use rezasm_core::instructions::instruction_registry::{is_instruction_name_registered};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!\nInstructions exist today? {}", name, is_instruction_name_registered(&"add".to_string()))
}

fn main() {
    register_instructions();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
