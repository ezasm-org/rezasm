use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::simulation::simulator::Writer;
use crate::util::error::IoError;
use crate::util::error::SimulatorError;

lazy_static! {
    pub static ref PRINTI: Instruction =
        instruction!(printi, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.int_value();
            let output = format!("{}", value1);
            write(simulator.get_writer(), &output)?;
            Ok(())
        });
    pub static ref PRINTF: Instruction =
        instruction!(printf, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.float_value();
            let output = format!("{}", value1);
            write(simulator.get_writer(), &output)?;
            Ok(())
        });
    pub static ref PRINTC: Instruction =
        instruction!(printc, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.int_value();
            let output = format!("{}", value1 as u8 as char);
            write(simulator.get_writer(), &output)?;
            Ok(())
        });
    pub static ref PRINTS_SIZED: Instruction =
        instruction!(prints, |simulator: Simulator,
                              input: InputTarget,
                              input2: InputTarget| {
            let address = input.get(&simulator)?.int_value();
            let size = input2.get(&simulator)?.int_value();
            let output = simulator
                .get_memory()
                .get_string_sized(address as usize, size as usize)?;
            write(simulator.get_writer(), &output)?;
            Ok(())
        });
    pub static ref PRINTS: Instruction =
        instruction!(prints, |simulator: Simulator, input: InputTarget| {
            let address = input.get(&simulator)?.int_value();
            let output = simulator.get_memory().get_string(address as usize)?;
            write(simulator.get_writer(), &output)?;
            Ok(())
        });
}

pub fn write(writer: &Mutex<Box<dyn Writer>>, data: &String) -> Result<(), IoError> {
    writer
        .lock()
        .unwrap()
        .write(&data.as_bytes())
        .map_err(|t| IoError::StdIoError(t))?;
    writer
        .lock()
        .unwrap()
        .flush()
        .map_err(|t| IoError::StdIoError(t))?;
    Ok(())
}

pub fn register_instructions() {
    register_instruction(&PRINTI);
    register_instruction(&PRINTF);
    register_instruction(&PRINTC);
    register_instruction(&PRINTS);
    register_instruction(&PRINTS_SIZED);
}
