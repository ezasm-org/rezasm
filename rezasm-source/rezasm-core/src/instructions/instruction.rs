use std::any::TypeId;
use std::fmt::{Debug, Formatter};

use crate::instructions::argument_type::ArgumentType;
use crate::simulation::simulator::Simulator;
use crate::util::error::SimulatorError;

pub type TInstructionFunction =
    fn(&mut Simulator, &Vec<TypeId>, &Vec<ArgumentType>) -> Result<(), SimulatorError>;

#[derive(Clone)]
pub struct Instruction {
    name: String,
    types: Vec<TypeId>,
    function: TInstructionFunction,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::format!("{:?}", self.types).as_str())
    }
}

impl Instruction {
    pub fn new(name: String, types: Vec<TypeId>, function: TInstructionFunction) -> Self {
        Instruction {
            name,
            types,
            function,
        }
    }
    pub fn get_types(&self) -> &Vec<TypeId> {
        &self.types
    }

    pub fn get_function(&self) -> &TInstructionFunction {
        &self.function
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn call_function(
        &self,
        simulator: &mut Simulator,
        arguments: &Vec<ArgumentType>,
    ) -> Result<(), SimulatorError> {
        (self.function)(simulator, &self.types, arguments)
    }
}

#[macro_export]
macro_rules! instruction {
    ($name:ident, |$simulator_name:ident: Simulator, $($names:ident: $types:ty),*| $func:tt) =>
    ({
        let mut v: Vec<std::any::TypeId> = Vec::new();
        $(v.push(std::any::TypeId::of::<&mut $types>());)*
        fn $name($simulator_name: &mut crate::simulation::simulator::Simulator, types: &Vec<std::any::TypeId>, arguments: &Vec<crate::instructions::argument_type::ArgumentType>) -> Result<(), crate::util::error::SimulatorError> {
            let mut _counter: usize = 0;
            $(
                #[allow(unused_mut)]
                let mut $names: $types = match arguments[_counter].clone().try_into() {
                    Ok(value) => value,
                    Err(error) => return Err(error.into()),
                };
                _counter = _counter + 1;
            )*
            $func
        }
        let mut instruction_name = std::stringify!($name);
        instruction_name = instruction_name.trim_start_matches('_');
        Instruction::new( instruction_name.to_string(), v, $name )
    });
}
pub use instruction;
