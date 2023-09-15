use lazy_static::lazy_static;
use rezasm_core::instructions::instruction_field::InstructionField;
use rezasm_core::instructions::instruction_registry::register_instruction;
use rezasm_core::util::error::{EzasmError, SimulatorError};
use rezasm_macro::instruction;

lazy_static! {
    pub static ref ADD: InstructionField =
        instruction!(add, |simulator: Simulator,
                     output: InputOutputTarget,
                     input1: InputTarget,
                     input2: InputTarget| {
                         let value1 = input1.get(&simulator)?.int_value();
                         let value2 = input2.get(&simulator)?.int_value();
                         let k = value1 + value2;
                         output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
                     });
    pub static ref SUB: InstructionField =
        instruction!(sub, |simulator: Simulator,
                     output: InputOutputTarget,
                     input1: InputTarget,
                     input2: InputTarget| {
                         let value1 = input1.get(&simulator)?.int_value();
                         let value2 = input2.get(&simulator)?.int_value();
                         let k = value1 - value2;
                         output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
                     });
    pub static ref MUL: InstructionField =
        instruction!(mul, |simulator: Simulator,
                     output: InputOutputTarget,
                     input1: InputTarget,
                     input2: InputTarget| {
                         let value1 = input1.get(&simulator)?.int_value();
                         let value2 = input2.get(&simulator)?.int_value();
                         let k = value1 * value2;
                         output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
                     });
    pub static ref DIV: InstructionField =
        instruction!(div, |simulator: Simulator,
                     output: InputOutputTarget,
                     input1: InputTarget,
                     input2: InputTarget| {
                         let value1 = input1.get(&simulator)?.int_value();
                         let value2 = input2.get(&simulator)?.int_value();
                         if value2 == 0 {
                             Err(SimulatorError::DivideByZeroError)
                         } else {
                             let k = value1 / value2;
                             output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
                         }
                     });
}

pub fn register_instructions() {
    register_instruction(&ADD);
    register_instruction(&SUB);
    register_instruction(&MUL);
    register_instruction(&DIV);
}
