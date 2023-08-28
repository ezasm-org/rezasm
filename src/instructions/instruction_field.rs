use std::any::TypeId;
use std::marker::PhantomData;

use crate::error::EzasmError;
use crate::instructions::instruction::Instruction;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::instruction::matches_argument_types;
use crate::instructions::argument_type::ArgumentType;
use crate::instructions::argument_type::Downcast;
use crate::simulation::simulator::Simulator;

#[derive(Debug)]
pub struct InstructionField {
    argument_types: Vec<TypeId>,
    instructions: Vec<Instruction>,
    name: String
}

impl InstructionField {
    fn new(argument_types: Vec<TypeId>, instructions: Vec<Instruction>, name: String) -> InstructionField {
        InstructionField {
            argument_types,
            instructions,
            name,
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

macro_rules! instruction_field {
    ($name:ident, |$simulator_name:ident: $simulator_type:ty, $($names:ident: $types:ty),*| -> ($return_type:ty) $func:tt) =>
    ({
        let mut types_list: Vec<TypeId> = Vec::new();
        let mut subtypes_list: Vec<Vec<TypeId>> = Vec::new();
        use crate::instructions::instruction_field::{Subclass, SubclassFactory};
        $(types_list.push(TypeId::of::<&mut $types>());)*
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
            instruction_field_vec.push(Instruction::new(permutation, Box::new(function)));
        }

        InstructionField::new(types_list, instruction_field_vec, std::stringify!($name).to_string())
    });
}

pub fn test_instruction_field_macro() -> InstructionField {
    instruction_field!(add, |simulator: &mut Simulator, x: InputOutputTarget, y: InputTarget, z: InputTarget| -> (()) {

    })
}
