use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;

lazy_static! {
    pub static ref PRINTS: Instruction =
    instruction!(prints, |simulator: Simulator,
                            output: InputOutputTarget,
                            input1: InputTarget| {
    });
    pub static ref PRINTS: Instruction =
    instruction!(prints, |simulator: Simulator,
                            output: InputOutputTarget,
                            input1: InputTarget
                            input2: InputTarget| {
    });
    pub static ref PRINTI: Instruction =
        instruction!(printi, |simulator: Simulator,
                                output: InputOutputTarget,
                                input1: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
        });
    pub static ref PRINTF: Instruction =
        instruction!(printf, |simulator: Simulator,
                                output: InputOutputTarget,
                                input1: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
        });
    pub static ref PRINTC: Instruction =
    instruction!(printc, |simulator: Simulator,
                            output: InputOutputTarget,
                            input1: InputTarget| {
    });
}