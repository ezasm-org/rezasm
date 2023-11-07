use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

lazy_static! {
    pub static ref PRINTI: Instruction =
        instruction!(printi, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.int_value();
            simulator.get_writer().write_integer(&value1);
            Ok(())
        });

    pub static ref PRINTF: Instruction =
        instruction!(printf, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.float_value();
            simulator.get_writer().write_float(&value1);
            Ok(())
        });

    pub static ref PRINTC: Instruction =
        instruction!(printc, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.int_value();
            simulator.get_writer().write_char(&(value1 as u8 as char));
            Ok(())
        });

    pub static ref PRINTS_SIZED: Instruction = 
        instruction!(prints, |simulator: Simulator, input: InputTarget, input2: InputTarget| {
        let address = input.get(&simulator)?.int_value();
        let size = input2.get(&simulator)?.int_value();
        let output = simulator.get_memory().get_string_sized(address as usize, size as usize)?;
        simulator.get_writer().write_string(&output);
        Ok(())
        });

    pub static ref PRINTS: Instruction = 
        instruction!(prints, |simulator: Simulator, input: InputTarget| {
        let address = input.get(&simulator)?.int_value();
        let output = simulator.get_memory().get_string(address as usize)?;
        simulator.get_writer().write_string(&output);
        Ok(())
        });
}

pub fn register_instructions() {
    register_instruction(&PRINTI);
    register_instruction(&PRINTF);
    register_instruction(&PRINTC);
    register_instruction(&PRINTS);
    register_instruction(&PRINTS_SIZED);
}
