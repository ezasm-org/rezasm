use crate::error::EzasmError;
use crate::instructions::argument_type::ArgumentType;
use crate::instructions::argument_type::Downcast;
use crate::instructions::targets::input_output_target::{InputOutput, InputOutputTarget};
use crate::instructions::targets::input_target::{Input, InputTarget};
use crate::instructions::targets::output_target::Output;
use crate::instructions::targets::Target;
use crate::parser::line::Line;
use crate::simulation::simulator::Simulator;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;

pub type TInstructionFunction =
    dyn FnMut(&mut Simulator, &Vec<TypeId>, &Vec<ArgumentType>) -> Result<(), EzasmError>;

pub struct Instruction {
    types: Vec<TypeId>,
    function: Box<TInstructionFunction>,
}

impl Instruction {
    pub fn get_types(&self) -> &Vec<TypeId> {
        &self.types
    }

    pub fn get_function(&mut self) -> &mut Box<TInstructionFunction> {
        &mut self.function
    }
}

macro_rules! instruction {
    ($name:ident, |$simulator_name:ident: $simulator_type:ty, $($names:ident: $types:ty),*| -> ($return_type:ty) $func:tt) =>
    ({
        let mut v: Vec<TypeId> = Vec::new();
        $(v.push(TypeId::of::<&mut $types>());)*
        let function = |$simulator_name: $simulator_type, types: &Vec<TypeId>, arguments: &Vec<ArgumentType>| -> Result<$return_type, EzasmError> {
            if matches_argument_types(&types, &arguments) {
                let mut counter: usize = 0;
                $(
                    let mut $names: $types = arguments[counter].downcast::<$types>().unwrap().clone();
                    counter = counter + 1;
                )*
                $func;
                Ok(())
            } else {
                return Err(EzasmError::InvalidArguments)
            }
        };

        Instruction {
            types: v,
            function: Box::new(function)
        }
    });
}

fn matches_argument_types(target: &Vec<TypeId>, attempt: &Vec<ArgumentType>) -> bool {
    if target.len() != attempt.len() {
        false
    } else {
        for (t, a) in zip(target, attempt) {
            if t != &a.get_mut_type_id() {
                return false;
            }
        }
        true
    }
}

pub fn test_instruction() -> Instruction {
    instruction!(foo, |simulator: &mut Simulator,
                       x: InputOutputTarget,
                       y: InputTarget,
                       z: InputTarget|
     -> (()) {
        let k =
            (y.get(&simulator).unwrap().int_value() + z.get(simulator).unwrap().int_value()).into();
        let _ = x.set(simulator, k);
        println!("Addition result: {}", x.get(simulator).unwrap().int_value());
    })
}
