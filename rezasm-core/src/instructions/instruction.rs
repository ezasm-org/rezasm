use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::iter::zip;

use crate::instructions::argument_type::ArgumentType;
use crate::simulation::simulator::Simulator;
use crate::util::error::EzasmError;

pub type TInstructionFunction =
    fn(&mut Simulator, &Vec<TypeId>, &Vec<ArgumentType>) -> Result<(), EzasmError>;

#[derive(Clone)]
pub struct Instruction {
    types: Vec<TypeId>,
    function: TInstructionFunction,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::format!("{:?}", self.types).as_str())
    }
}

impl Instruction {
    pub fn new(types: Vec<TypeId>, function: TInstructionFunction) -> Self {
        Instruction { types, function }
    }
    pub fn get_types(&self) -> &Vec<TypeId> {
        &self.types
    }

    pub fn get_function(&self) -> &TInstructionFunction {
        &self.function
    }
}

pub fn matches_argument_types(target: &Vec<TypeId>, attempt: &Vec<ArgumentType>) -> bool {
    if target.len() != attempt.len() {
        false
    } else {
        for (t, a) in zip(target, attempt) {
            if t != &a.get_mut_type_id() && t != &a.get_ref_type_id() && t != &a.get_type_id() {
                return false;
            }
        }
        true
    }
}
