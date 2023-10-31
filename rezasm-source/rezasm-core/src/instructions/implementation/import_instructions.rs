use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;
use crate::instructions::targets::input_target::InputTarget;

lazy_static! {
    pub static ref IMPORT: Instruction =
        instruction!(import, |simulator: Simulator,
                            input: InputTarget| {
            if let InputTarget::StringInput(path) = input {
                simulator.import_lines_from_file(path)
            } else {
                panic!("String input error too lazy to handle error rn lol");
            }
        });
}

pub fn register_instructions() {
    register_instruction(&IMPORT);
}