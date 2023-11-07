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
        instruction!(printf, |simulator: Simulator, input: InputTarget| {
            let value1 = input.get(&simulator)?.int_value();
            simulator.get_writer().write_char(&(value1 as u8 as char));
            Ok(())
        });
}

pub fn register_instructions() {}
