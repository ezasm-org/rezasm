use crate::instructions::targets::Target;

pub trait InputTarget: Target {
    fn get(&self) -> Vec<u8>;
}

impl<T: InputTarget> Target for T {}

pub enum InputTargets {
    ImmediateInput(Vec<u8>),
    LabelReferenceInput(String),
    StringInput(String),
}

impl InputTarget for InputTargets {
    fn get(&self) -> Vec<u8> {
        //TODO implement proper getters for the string variants
        match self {
            InputTargets::ImmediateInput(v) => v.clone(),
            InputTargets::LabelReferenceInput(s) => Vec::new(),
            InputTargets::StringInput(s) => Vec::new(),
        }
    }
}
