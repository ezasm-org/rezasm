// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

extern crate tokio;
extern crate rezasm_core;
extern crate rezasm_macro;
extern crate rezasm_app;

use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use lazy_static::lazy_static;
use tokio::runtime;
use rezasm_app::instructions::implementation::arithmetic_instructions::register_instructions;
use rezasm_core::parser::lexer;
use rezasm_core::simulation::registry;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::EzasmError;
use crate::util::serial_result::SerialResult;
use crate::util::runtime::Runtime;

lazy_static! {
    static ref SIMULATOR: Arc<RwLock<Simulator>> = Arc::new(RwLock::new(Simulator::new()));
    static ref RUNTIME: Arc<RwLock<Runtime>> = Arc::new(RwLock::new(Runtime::new()));
}

pub fn get_simulator<'a>() -> RwLockWriteGuard<'a, Simulator> {
    SIMULATOR.write().unwrap()
}

pub fn set_simulator(simulator: Simulator) {
    *SIMULATOR.write().unwrap() = simulator;
}

#[tauri::command]
fn stop() {
    RUNTIME.write().unwrap().abort();
}

#[tauri::command]
fn reset() {
    get_simulator().reset();
}

#[tauri::command]
fn load(lines: &str) -> SerialResult<(), String> {
    let mut simulator = get_simulator();

    for line_string in lines
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
                        Err(error) => return SerialResult::Err(format!("{:?}", error)),
                    }
                },
                Err(error) => return SerialResult::Err(format!("{:?}", error)),
            }
        };
    }
    SerialResult::Ok(())
}

#[tauri::command(async)]
async fn run() {
    tauri::async_runtime::spawn(async {
        // let mut counter: usize = 0;
        let mut simulator = get_simulator();
        while !simulator.is_done() && !simulator.is_error() && !RUNTIME.read().unwrap().deref().force_stop {
            match simulator.run_line_from_pc() {
                Ok(_) => { /*counter += 1*/ },
                Err(error) => return Err(error),
            }
            // if counter % 1000 == 0 {
            //     println!("{}", counter / 1000);
            // }
        }

        if simulator.is_error() {
            Err(EzasmError::InvalidProgramCounterError(simulator.get_registers().get_pc().get_data().int_value()))
        } else {
            Ok(())
        }
    });

}

#[tauri::command(async)]
async fn step() {
    tauri::async_runtime::spawn(async {
        match get_simulator().run_line_from_pc() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    });
}

#[tauri::command]
fn is_completed() -> bool {
    get_simulator().is_done() || get_simulator().is_error()
}

#[tauri::command]
fn get_exit_status() -> i64 {
    get_simulator().get_registers().get_register(&registry::R0.to_string()).unwrap().get_data().int_value()
}

#[tauri::command]
fn get_register_value(register: &str) -> Option<i64> {
    match get_simulator().get_registers().get_register(&register.to_string()) {
        Ok(x) => Some(x.get_data().int_value()),
        Err(_) => None,
    }
}


fn main() {
    register_instructions();

    let rt = runtime::Builder::new_multi_thread().build().unwrap();

    rt.block_on(async {
        tauri::async_runtime::set(runtime::Handle::current());
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load, reset, run, step, stop, is_completed, get_exit_status, get_register_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
