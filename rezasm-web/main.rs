// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate rezasm_core;
extern crate rezasm_macro;
extern crate rezasm_app;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;
use rezasm_app::instructions::implementation::arithmetic_instructions::register_instructions;
use rezasm_core::parser::lexer;
use rezasm_core::simulation::registry;
use rezasm_core::simulation::simulator::Simulator;

lazy_static! {
    static ref SIMULATOR: Arc<RwLock<Simulator>> = Arc::new(RwLock::new(Simulator::new()));
}

pub fn get_simulator<'a>() -> RwLockWriteGuard<'a, Simulator> {
    SIMULATOR.write().unwrap()
}

pub fn set_simulator(simulator: Simulator) {
    *SIMULATOR.write().unwrap() = simulator;
}

#[tauri::command]
fn run(line: &str) -> String {
    let mut simulator = get_simulator();

    for line_string in line
        .lines()
        .map(|string| string.to_string())
        .collect::<Vec<String>>() {
        let line_parse = lexer::parse_line(&line_string.to_string(), simulator.get_word_size());

        match line_parse {
            None => { /* no-op */ },
            Some(x) => match x {
                Ok(line ) => {
                    match simulator.add_line(line) {
                        Ok(_) => {}
                        Err(error) => return format!("{:?}", error),
                    }
                },
                Err(error) => return format!("{:?}", error),
            }
        };
    }
    while !simulator.is_done() || simulator.is_error() {
        match simulator.run_line_from_pc() {
            Ok(_) => {}
            Err(error) => return format!("{:?}", error),
        }
    }
    let return_code: i64 = simulator.get_registers().get_register(&registry::R0.to_string()).unwrap().get_data().int_value();
    format!("Program completed with exit code {}", return_code)
}

#[tauri::command]
fn step() -> String {
    "".to_string()
}

fn main() {
    register_instructions();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
