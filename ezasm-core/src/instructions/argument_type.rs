use std::any::TypeId;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::{Input, InputTarget};
use crate::instructions::targets::Target;

#[derive(Debug)]
pub enum ArgumentType {
    InputOutput(InputOutputTarget),
    Input(InputTarget),
}

pub trait Downcast {
    fn downcast<T: Input>(&self) -> Option<&T>;
}

impl Downcast for ArgumentType {
    fn downcast<T: Input>(&self) -> Option<&T> {
        match self {
            ArgumentType::InputOutput(x) => x.as_any().downcast_ref::<T>(),
            ArgumentType::Input(x) => x.as_any().downcast_ref::<T>(),
        }
    }
}

impl ArgumentType {
    pub fn is_input_output(&self) -> bool {
        match self {
            ArgumentType::InputOutput(_) => true,
            ArgumentType::Input(_) => false,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            ArgumentType::InputOutput(_) => false,
            ArgumentType::Input(_) => true,
        }
    }

    pub fn get_input(&self) -> Option<Box<&InputTarget>> {
        match self {
            ArgumentType::InputOutput(_) => return None,
            ArgumentType::Input(x) => Some(Box::new(x)),
        }
    }

    pub fn get_input_output(&self) -> Option<Box<&InputOutputTarget>> {
        match self {
            ArgumentType::InputOutput(x) => Some(Box::new(x)),
            ArgumentType::Input(_) => return None,
        }
    }

    pub fn into_input(self) -> Option<Box<InputTarget>> {
        match self {
            ArgumentType::InputOutput(_) => return None,
            ArgumentType::Input(x) => Some(Box::new(x)),
        }
    }

    pub fn into_input_output(self) -> Option<Box<InputOutputTarget>> {
        match self {
            ArgumentType::InputOutput(x) => Some(Box::new(x)),
            ArgumentType::Input(_) => return None,
        }
    }

    pub fn get_mut_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<&mut InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<&mut InputTarget>(),
        }
    }

    pub fn get_ref_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<&InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<&InputTarget>(),
        }
    }

    pub fn get_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<InputTarget>(),
        }
    }

    pub fn get_target(&self) -> Box<dyn Target> {
        match self {
            ArgumentType::Input(t) => Box::new(t.clone()),
            ArgumentType::InputOutput(t) => Box::new(t.clone()),
        }
    }
}
