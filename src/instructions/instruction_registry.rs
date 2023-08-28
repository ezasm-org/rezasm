use std::collections::HashMap;
use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_field::InstructionField;

struct InstructionRegistry {
    instructions: HashMap<String, Vec<InstructionField>>,
}

impl InstructionRegistry {
    pub fn register_instruction(&mut self, instruction: InstructionField) {
        match self.instructions.get_mut(instruction.name()) {
            None => self.instructions.insert(instruction.name().to_string(), vec![instruction]),
            Some(x) => {
                x.push(instruction);
                None
            },
        };
    }

    pub fn get_instruction(&self, name: &String, args: &Vec<ArgumentType>) -> Option<&Instruction> {
        match self.instructions.get(name) {
            None => None,
            Some(group) => {
                for attempt in group.iter() {
                    match attempt.get_instruction(args) {
                        None => {},
                        Some(instruction) => return Some(instruction)
                    }
                }
                None
            }
        }
    }
}
