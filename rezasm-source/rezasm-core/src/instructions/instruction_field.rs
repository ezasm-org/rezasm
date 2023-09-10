use std::any::TypeId;
use std::iter::zip;
use std::marker::PhantomData;

use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::Instruction;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::simulation::simulator::Simulator;
use crate::util::error::SimulatorError;

#[derive(Debug)]
pub struct InstructionField {
    argument_types: Vec<TypeId>,
    instructions: Vec<Instruction>,
    name: String,
}

impl InstructionField {
    pub fn new(
        argument_types: Vec<TypeId>,
        instructions: Vec<Instruction>,
        name: String,
    ) -> InstructionField {
        InstructionField {
            name,
            argument_types,
            instructions,
        }
    }

    pub fn len(&self) -> usize {
        self.argument_types.len()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn argument_types(&self) -> &Vec<TypeId> {
        &self.argument_types
    }

    pub fn get_instruction(&self, instruction_attempt: &Vec<ArgumentType>) -> Option<&Instruction> {
        if self.len() != instruction_attempt.len() {
            return None;
        }

        for instruction in self.instructions.iter() {
            if match_any_argument_types(instruction.get_types(), instruction_attempt) {
                return Some(instruction);
            }
        }
        None
    }

    pub fn call_instruction_function(
        &self,
        simulator: &mut Simulator,
        instruction_attempt: &Vec<ArgumentType>,
    ) -> Result<(), SimulatorError> {
        match self.get_instruction(instruction_attempt) {
            None => Err(SimulatorError::InvalidInstructionError(
                self.name.to_string(),
            )),
            Some(instruction) => (*instruction.get_function())(
                simulator,
                instruction.get_types(),
                instruction_attempt,
            ),
        }
    }
}

pub trait Subclass<T> {
    fn subclasses() -> Vec<TypeId>;
}

pub struct SubclassFactory<T> {
    phantom: PhantomData<T>,
}

impl Subclass<InputTarget> for SubclassFactory<InputTarget> {
    fn subclasses() -> Vec<TypeId> {
        vec![
            TypeId::of::<InputTarget>(),
            TypeId::of::<InputOutputTarget>(),
        ]
    }
}

impl Subclass<InputOutputTarget> for SubclassFactory<InputOutputTarget> {
    fn subclasses() -> Vec<TypeId> {
        vec![TypeId::of::<InputOutputTarget>()]
    }
}

fn match_any_argument_types(target: &Vec<TypeId>, attempt: &Vec<ArgumentType>) -> bool {
    if target.len() != attempt.len() {
        false
    } else {
        for (t, a) in zip(target, attempt) {
            if t != &a.get_mut_type_id() && t != &a.get_type_id() && t != &a.get_ref_type_id() {
                return false;
            }
        }
        true
    }
}
