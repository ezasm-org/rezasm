use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;

lazy_static! {
    pub static ref IMPORT: Instruction =
        instruction!(import, |simulator: Simulator,
                            input: InputTarget| {
            let path = input.get(&simulator)?.string_value();
            simulator.import_lines_from_file(path)
        });
}

pub fn register_instructions() {
    register_instruction(&IMPORT);
}