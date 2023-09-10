use crate::util::runtime::Runtime;
use crate::util::serial_result::SerialResult;
use std::ops::Deref;

use lazy_static::lazy_static;
use rezasm_core::parser::lexer;
use rezasm_core::simulation::registry;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::EzasmError;

use std::string::ToString;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

type CallbackFnStr = fn(&str);
type CallbackFnI64 = fn(i64);

fn _temp_str(_: &str) {}
fn _temp_i64(_: i64) {}

lazy_static! {
    static ref SIMULATOR: Arc<RwLock<Simulator>> = Arc::new(RwLock::new(Simulator::new()));
    static ref RUNTIME: Arc<RwLock<Runtime>> = Arc::new(RwLock::new(Runtime::new()));
    static ref SIGNAL_ERROR: Arc<RwLock<CallbackFnStr>> = Arc::new(RwLock::new(_temp_str));
    static ref SIGNAL_PROGRAM_COMPLETION: Arc<RwLock<CallbackFnI64>> =
        Arc::new(RwLock::new(_temp_i64));
}

pub fn get_simulator() -> RwLockWriteGuard<'static, Simulator> {
    SIMULATOR.write().unwrap()
}

pub fn set_simulator(simulator: Simulator) {
    *SIMULATOR.write().unwrap() = simulator;
}

pub fn register_callbacks(signal_error: CallbackFnStr, signal_program_completion: CallbackFnI64) {
    *SIGNAL_ERROR.write().unwrap() = signal_error;
    *SIGNAL_PROGRAM_COMPLETION.write().unwrap() = signal_program_completion;
}

pub fn stop() {
    RUNTIME.write().unwrap().abort();
}

pub fn reset() {
    stop();
    get_simulator().reset();
}

pub fn load(lines: &str) -> SerialResult<(), String> {
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
                    Err(error) => {
                        return SerialResult::Err(format!("Error parsing program: {}", error))
                    }
                },
                Err(error) => {
                    return SerialResult::Err(format!("Error parsing program: {}", error))
                }
            },
        };
    }
    SerialResult::Ok(())
}

pub fn run() {
    RUNTIME.write().unwrap().call(async {
        {
            let mut simulator = get_simulator();
            while !simulator.is_done() && !simulator.is_error() {
                if RUNTIME.read().unwrap().deref().force_stop {
                    break;
                }
                match simulator.run_line_from_pc() {
                    Ok(_) => {}
                    Err(error) => {
                        signal_error(format!("Program error: {}", error).as_str());
                        return Ok::<(), EzasmError>(());
                    }
                }
            }
        }

        let simulator = get_simulator();
        if simulator.is_error() {
            signal_error(
                format!(
                    "Invalid PC: {}",
                    simulator.get_registers().get_pc().get_data().int_value()
                )
                .as_str(),
            );
        } else if simulator.is_done() {
            signal_program_completion(
                simulator
                    .get_registers()
                    .get_register(&registry::R0.to_string())
                    .unwrap()
                    .get_data()
                    .int_value(),
            );
        } else {
            signal_error("Program terminated forcefully");
        }
        Ok(())
    });
}

pub fn step() {
    RUNTIME.write().unwrap().call(async {
        {
            let mut simulator = get_simulator();
            match simulator.run_line_from_pc() {
                Ok(_) => {}
                Err(error) => {
                    signal_error(format!("Program error: {}", error).as_str());
                    return Ok::<(), EzasmError>(());
                }
            }
        }

        let simulator = get_simulator();
        if simulator.is_error() {
            signal_error(
                format!(
                    "Invalid PC: {}",
                    simulator.get_registers().get_pc().get_data().int_value()
                )
                .as_str(),
            );
        } else if simulator.is_done() {
            signal_program_completion(
                simulator
                    .get_registers()
                    .get_register(&registry::R0.to_string())
                    .unwrap()
                    .get_data()
                    .int_value(),
            );
        } else {
            signal_error("Program terminated forcefully");
        }
        Ok(())
    });
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

fn signal_error(error: &str) {
    SIGNAL_ERROR.read().unwrap()(error);
}

fn signal_program_completion(exit_status: i64) {
    SIGNAL_PROGRAM_COMPLETION.read().unwrap()(exit_status);
}

pub fn initialize_globals(runtime: tokio::runtime::Runtime) {
    *RUNTIME.write().unwrap() = Runtime::from_rt(runtime);
}
