use std::any::TypeId;
use std::iter::zip;
use std::marker::PhantomData;
use crate::util::error::EzasmError;

use crate::instructions::instruction::Instruction;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::argument_type::ArgumentType;
use crate::simulation::simulator::Simulator;

#[derive(Debug)]
pub struct InstructionField {
    argument_types: Vec<TypeId>,
    instructions: Vec<Instruction>,
    name: String
}

impl InstructionField {
    pub fn new(argument_types: Vec<TypeId>, instructions: Vec<Instruction>, name: String) -> InstructionField {
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
                return Some(instruction)
            }
        }
        None
    }

    pub fn call_instruction_function(&self, simulator: &mut Simulator, instruction_attempt: &Vec<ArgumentType>) -> Result<(), EzasmError> {
        match self.get_instruction(instruction_attempt) {
            None => Err(EzasmError::InvalidArguments),
            Some(instruction) => (*instruction.get_function())(simulator, instruction.get_types(), instruction_attempt)
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
        vec![TypeId::of::<InputTarget>(), TypeId::of::<InputOutputTarget>()]
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

#[macro_export]
macro_rules! instruction_field {
    ($name:ident, |$simulator_name:ident: $simulator_type:ty, $($names:ident: $types:ty),*| -> ($return_type:ty) $func:tt) =>
    ({
        let mut types_list: Vec<TypeId> = Vec::new();
        let mut subtypes_list: Vec<Vec<TypeId>> = Vec::new();

        #[allow(unused_imports)]
        #[allow(unused_variables)]
        #[allow(unused_assignments)]

        use std::any::TypeId;
        use ezasm_core::util::error::EzasmError;
        use ezasm_core::instructions::instruction_field::{Subclass, SubclassFactory};
        use ezasm_core::instructions::instruction::{Instruction, matches_argument_types};
        use ezasm_core::instructions::targets::input_target::Input;
        use ezasm_core::instructions::targets::output_target::Output;
        use ezasm_core::instructions::argument_type::ArgumentType;
        use ezasm_core::instructions::argument_type::Downcast;
        use ezasm_core::instructions::targets::input_output_target::InputOutputTarget;
        use ezasm_core::instructions::targets::input_target::InputTarget;
        use ezasm_core::instructions::instruction_field::InstructionField;
        use ezasm_core::simulation::simulator::Simulator;

        $(types_list.push(TypeId::of::<$types>());)*
        $(subtypes_list.push(SubclassFactory::<$types>::subclasses());)*

        let mut all_possible_lists: Vec<Vec<TypeId>> = vec![Vec::new()];
        for type_element_list in subtypes_list.iter() {
            let initial_lists_state = all_possible_lists.clone();
            for (type_element_index, type_element) in type_element_list.iter().enumerate() {
                if type_element_index > 0 {
                    // Append a copy of the initial state, each with the next type appended to them
                    for list in initial_lists_state.iter() {
                        let mut list = list.clone();
                        list.push(type_element.clone());
                        all_possible_lists.push(list);
                    }
                } else {
                    // Only happens for the first element
                    for list in all_possible_lists.iter_mut() {
                        list.push(type_element.clone());
                    }
                }
            }
        }

        let mut instruction_field_vec: Vec<Instruction> = Vec::new();
        for permutation in all_possible_lists {
            fn $name($simulator_name: $simulator_type, types: &Vec<TypeId>, arguments: &Vec<ArgumentType>) -> Result<$return_type, EzasmError> {
                if matches_argument_types(&types, &arguments) {
                    let mut _counter: usize = 0;
                    $(
                        #[allow(unused_mut)]
                        let mut $names: $types = arguments[_counter].downcast::<$types>().unwrap().clone();
                        _counter = _counter + 1;
                    )*
                    $func;
                    Ok(())
                } else {
                    return Err(EzasmError::InvalidArguments)
                }
            }
            instruction_field_vec.push(Instruction::new(permutation, $name));
        }

        InstructionField::new(types_list, instruction_field_vec, std::stringify!($name).to_string())
    });
}
