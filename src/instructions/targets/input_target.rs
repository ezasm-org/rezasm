use crate::{instructions::targets::Target, util::raw_data::{self, RawData}, simulator::memory::WordSize};

pub trait InputTarget: Target {
    fn get(&self) -> &RawData;
}

impl<T: InputTarget> Target for T {}

pub enum InputTargets {
    ImmediateInput(RawData),
    LabelReferenceInput(String),
    StringInput(String),
}

impl InputTargets {
    fn new_immediate(data: RawData) -> InputTargets {
        Self::ImmediateInput(data)
    }
    
    fn new_label_reference(data: &String) -> InputTargets {
        Self::LabelReferenceInput(data.clone())
    }

    fn new_string(data: &String) -> InputTargets {
        Self::StringInput(data.clone())
    }
}

impl InputTarget for InputTargets {
    fn get(&self) -> &RawData {
        //TODO implement proper getters for the string variants
        match self {
            InputTargets::ImmediateInput(v) => v,
            InputTargets::LabelReferenceInput(s) => &RawData::empty_data(&WordSize::Four),
            InputTargets::StringInput(s) => &RawData::empty_data(&WordSize::Four),
        }
    }
}
