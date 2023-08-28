use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::iter::zip;

use crate::error::EzasmError;
use crate::instructions::argument_type::ArgumentType;
use crate::simulation::simulator::Simulator;

pub type TInstructionFunction =
    dyn FnMut(&mut Simulator, &Vec<TypeId>, &Vec<ArgumentType>) -> Result<(), EzasmError>;

pub struct Instruction {
    types: Vec<TypeId>,
    function: Box<TInstructionFunction>,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::format!("{:?}", self.types).as_str())
    }
}

impl Instruction {

    pub fn new(types: Vec<TypeId>, function: Box<TInstructionFunction>) -> Self {
        Instruction {
            types,
            function
        }
    }
    pub fn get_types(&self) -> &Vec<TypeId> {
        &self.types
    }

    pub fn get_function(&self) -> Box<TInstructionFunction> {
        unsafe {
            (((&self.function) as *const Box<TInstructionFunction>) as *mut Box<TInstructionFunction>).clone().read()
        }
    }
}

pub fn matches_argument_types(target: &Vec<TypeId>, attempt: &Vec<ArgumentType>) -> bool {
    if target.len() != attempt.len() {
        false
    } else {
        for (t, a) in zip(target, attempt) {
            if t != &a.get_mut_type_id() && t != &a.get_ref_type_id() && t != &a.get_type_id()   {
                return false;
            }
        }
        true
    }
}
