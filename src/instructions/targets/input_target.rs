use crate::{instructions::targets::Target, util::raw_data::{self, RawData}};
use crate::error::EzasmError;
use crate::simulation::simulator::Simulator;
use crate::util::word_size::WordSize;

pub trait Input: Target {
    fn get(&self, simulator: Simulator) -> Result<RawData, EzasmError>;
}

impl<T: Input> Target for T {}

pub enum InputTarget {
    ImmediateInput(RawData),
    LabelReferenceInput(String),
    StringInput(String),
}

impl InputTarget {
    fn new_immediate(data: RawData) -> InputTarget {
        Self::ImmediateInput(data)
    }
    
    fn new_label_reference(data: &String) -> InputTarget {
        Self::LabelReferenceInput(data.clone())
    }

    fn new_string(data: &String) -> InputTarget {
        Self::StringInput(data.clone())
    }
}

impl Input for InputTarget {
    fn get(&self, simulator: Simulator) -> Result<RawData, EzasmError> {
        match self {
            InputTarget::ImmediateInput(x) => Ok(x.clone()),
            InputTarget::LabelReferenceInput(s) => simulator.get_label_line_number(s)
                                                                .map(|x| {RawData::from_int(x.clone(), simulator.get_word_size())}),
            InputTarget::StringInput(s) => simulator.get_memory().get_string_immediate_address(s)
                                                                         .map(|x| {x.clone()}),
        }
    }
}
