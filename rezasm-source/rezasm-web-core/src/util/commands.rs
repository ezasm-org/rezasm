use lazy_static::lazy_static;
use rezasm_core::parser::lexer;
use rezasm_core::simulation::registry;
use rezasm_core::simulation::simulator::Simulator;

use crate::util::runtime::Runtime;

use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, RwLockWriteGuard};

lazy_static! {
    static ref SIMULATOR: Arc<RwLock<Simulator>> = Arc::new(RwLock::new(Simulator::new()));
    static ref SHOULD_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref RUNTIME: Arc<RwLock<Runtime>> =
        Arc::new(RwLock::new(Runtime::new(SHOULD_STOP.clone())));
}

pub fn get_simulator() -> RwLockWriteGuard<'static, Simulator> {
    SIMULATOR.write().unwrap()
}

pub fn set_simulator(simulator: Simulator) {
    *SIMULATOR.write().unwrap() = simulator;
}

pub fn get_runtime() -> RwLockWriteGuard<'static, Runtime> {
    RUNTIME.write().unwrap()
}

pub fn stop() {
    SHOULD_STOP.store(true, Ordering::SeqCst);
    get_runtime().abort();
}

pub fn reset() {
    stop();
    get_simulator().reset();
}

pub fn load(lines: &str) -> Result<(), String> {
    let mut simulator = get_simulator();

    for line_string in lines
        .lines()
        .map(|string| string.to_string())
        .collect::<Vec<String>>()
    {
        let line_parse = lexer::parse_line(&line_string.to_string(), simulator.get_word_size());

        match line_parse {
            None => { /* no-op */ }
            Some(x) => match x {
                Ok(line) => match simulator.add_line(line) {
                    Ok(_) => {}
                    Err(error) => return Err(format!("Error parsing program: {}", error)),
                },
                Err(error) => return Err(format!("Error parsing program: {}", error)),
            },
        };
    }
    Ok(())
}

pub fn step() -> Result<(), String> {
    match get_simulator().run_line_from_pc() {
        Ok(_) => {}
        Err(error) => return Err(format!("Program error: {}", error)),
    };

    let simulator = get_simulator();

    if simulator.is_error() {
        Err(format!(
            "Invalid PC: {}",
            simulator.get_registers().get_pc().get_data().int_value()
        ))
    } else {
        Ok(())
    }
}

pub fn is_completed() -> bool {
    get_simulator().is_done() || get_simulator().is_error()
}

pub fn get_exit_status() -> i64 {
    get_simulator()
        .get_registers()
        .get_register(&registry::R0.to_string())
        .unwrap()
        .get_data()
        .int_value()
}

pub fn get_register_value(register: &str) -> Option<i64> {
    match get_simulator()
        .get_registers()
        .get_register(&register.to_string())
    {
        Ok(x) => Some(x.get_data().int_value()),
        Err(_) => None,
    }
}

pub fn initialize_runtime(runtime: tokio::runtime::Runtime) {
    *RUNTIME.write().unwrap() = Runtime::from_rt(SHOULD_STOP.clone(), runtime);
}
