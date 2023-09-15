use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_field::InstructionField;
use crate::util::error::ParserError;

pub struct InstructionRegistry {
    instructions: HashMap<String, Vec<&'static InstructionField>>,
}

impl InstructionRegistry {
    fn new() -> InstructionRegistry {
        InstructionRegistry {
            instructions: HashMap::new(),
        }
    }

    fn register_instruction(&mut self, instruction: &'static InstructionField) {
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

    fn get_instruction(
        &self,
        name: &String,
        args: &Vec<ArgumentType>,
    ) -> Result<&'static Instruction, ParserError> {
        match self.instructions.get(name) {
            None => Err(ParserError::InvalidInstructionError(name.to_string())),
            Some(group) => {
                for attempt in group.iter() {
                    match attempt.get_instruction(args) {
                        None => {}
                        Some(instruction) => return Ok(instruction),
                    }
                }
                Err(ParserError::InvalidArgumentsError(name.to_string()))
            }
        }
    }
}

lazy_static! {
    static ref INSTRUCTIONS: Mutex<InstructionRegistry> = Mutex::new(InstructionRegistry::new());
}

pub fn register_instruction(instruction: &'static InstructionField) {
    INSTRUCTIONS
        .lock()
        .unwrap()
        .register_instruction(instruction);
}

pub fn get_instruction(
    name: &String,
    args: &Vec<ArgumentType>,
) -> Result<&'static Instruction, ParserError> {
    INSTRUCTIONS.lock().unwrap().get_instruction(name, args)
}

pub fn is_instruction_name_registered(instruction: &String) -> bool {
    INSTRUCTIONS
        .lock()
        .unwrap()
        .instructions
        .contains_key(instruction)
}