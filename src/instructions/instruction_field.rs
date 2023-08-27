use std::any::TypeId;
use std::marker::PhantomData;
use crate::instructions::instruction::Instruction;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;

pub struct InstructionField{
    argument_count: usize,
    argument_types: Vec<TypeId>,
    instructions: Vec<Instruction>,
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
        let mut v: Vec<TypeId> = Vec::new();
        $(v.push(TypeId::of::<&mut $types>());)*
        for ;
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
