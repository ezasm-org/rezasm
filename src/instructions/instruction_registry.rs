use ezasm_core::instructions::argument_type::ArgumentType;
use ezasm_core::instructions::instruction::Instruction;
use ezasm_core::instructions::instruction_field::InstructionField;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct InstructionRegistry {
    instructions: HashMap<String, Vec<InstructionField>>,
}

impl InstructionRegistry {
    pub fn new() -> InstructionRegistry {
        InstructionRegistry {
            instructions: HashMap::new(),
        }
    }

    pub fn register_instruction(&mut self, instruction: InstructionField) {
        match self.instructions.get_mut(instruction.name()) {
            None => self
                .instructions
                .insert(instruction.name().to_string(), vec![instruction]),
            Some(x) => {
                x.push(instruction);
                None
            }
        };
    }

    pub fn get_instruction(&self, name: &String, args: &Vec<ArgumentType>) -> Option<&Instruction> {
        match self.instructions.get(name) {
            None => None,
            Some(group) => {
                for attempt in group.iter() {
                    match attempt.get_instruction(args) {
                        None => {}
                        Some(instruction) => return Some(instruction),
                    }
                }
                None
            }
        }
    }
}

lazy_static! {
    pub static ref INSTRUCTIONS: InstructionRegistry = InstructionRegistry::new();
}
