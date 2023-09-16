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

lazy_static! {
    pub static ref ADD: Instruction =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 + value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref SUB: Instruction =
        instruction!(sub, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 - value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref MUL: Instruction =
        instruction!(mul, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 * value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref DIV: Instruction =
        instruction!(div, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value2 == 0 {
                return Err(SimulatorError::DivideByZeroError);
            } else {
                let k = value1 / value2;
                return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
            }
        });
}

pub fn register_instructions() {
    register_instruction(&ADD);
    register_instruction(&SUB);
    register_instruction(&MUL);
    register_instruction(&DIV);
}
