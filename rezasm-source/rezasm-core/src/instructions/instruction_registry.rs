use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::instructions::instruction::Instruction;
use crate::util::error::ParserError;

pub struct InstructionRegistry {
    instructions: HashMap<String, Vec<&'static Instruction>>,
}

impl InstructionRegistry {
    fn new() -> InstructionRegistry {
        InstructionRegistry {
            instructions: HashMap::new(),
        }
    }

    fn register_instruction(&mut self, instruction: &'static Instruction) {
        match self.instructions.get_mut(instruction.get_name()) {
            None => self
                .instructions
                .insert(instruction.get_name().to_string(), vec![instruction]),
            Some(x) => {
                x.push(instruction);
                None
            }
        };
    }

    fn get_instruction(
        &self,
        name: &str,
        argc: usize,
    ) -> Result<&'static Instruction, ParserError> {
        match self.instructions.get(name) {
            None => Err(ParserError::InvalidInstructionError(name.to_string())),
            Some(group) => {
                for attempt in group.iter() {
                    if attempt.get_types().len() == argc {
                        return Ok(attempt);
                    }
                }
                Err(ParserError::InvalidArgumentsCountError(
                    name.to_string(),
                    argc,
                ))
            }
        }
    }
}

lazy_static! {
    static ref INSTRUCTIONS: Mutex<InstructionRegistry> = Mutex::new(InstructionRegistry::new());
}

/// Registers an instruction with the static INSTRUCTIONS
///
/// # Arguments
///
/// * instruction: the static-lifetime instruction to register
pub fn register_instruction(instruction: &'static Instruction) {
    INSTRUCTIONS
        .lock()
        .unwrap()
        .register_instruction(instruction);
}

pub fn get_instruction(name: &str, argc: usize) -> Result<&'static Instruction, ParserError> {
    INSTRUCTIONS.lock().unwrap().get_instruction(name, argc)
}

pub fn is_instruction_name_registered(instruction: &str) -> bool {
    INSTRUCTIONS
        .lock()
        .unwrap()
        .instructions
        .contains_key(instruction)
}
