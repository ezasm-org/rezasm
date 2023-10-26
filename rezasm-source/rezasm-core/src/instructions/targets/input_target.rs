use std::any::Any;

use crate::simulation::registry;
use crate::simulation::simulator::Simulator;
use crate::util::error::{InternalError, ParserError, SimulatorError};
use crate::{instructions::targets::Target, util::raw_data::RawData};

pub trait Input: Target {
    fn get(&self, simulator: &Simulator) -> Result<RawData, SimulatorError>;
}

impl<T: Input> Target for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub enum InputTarget {
    ImmediateInput(RawData),
    RegisterInput(usize),
    DereferenceInput(usize, i64),
    LabelReferenceInput(String),
    StringInput(String),
}

impl InputTarget {
    pub fn new_immediate(data: RawData) -> InputTarget {
        Self::ImmediateInput(data)
    }

    pub fn new_label_reference(data: &String) -> InputTarget {
        Self::LabelReferenceInput(data.clone())
    }

    pub fn new_string(data: &String) -> InputTarget {
        Self::StringInput(data.clone())
    }

    pub fn new_dereference_offset(
        register: usize,
        offset: i64,
    ) -> Result<InputTarget, ParserError> {
        if registry::is_valid_register_number(register) {
            Ok(InputTarget::DereferenceInput(register, offset))
        } else {
            Err(ParserError::InvalidRegisterNumberError(register))
        }
    }

    pub fn new_register(register: &usize) -> Result<InputTarget, ParserError> {
        Ok(InputTarget::RegisterInput(register.clone()))
    }

    fn register_data(&self, simulator: &Simulator) -> Result<RawData, ParserError> {
        let register = match self {
            InputTarget::DereferenceInput(r, _) => {
                simulator.get_registers().get_register_by_number(r.clone())
            }
            InputTarget::RegisterInput(r) => {
                simulator.get_registers().get_register_by_number(r.clone())
            }
            _ => Err(InternalError::GetInputOutputTargetError.into()),
        };

        match register {
            Ok(r) => Ok(r.get_data().clone()),
            Err(error) => Err(error),
        }
    }
}

impl Input for InputTarget {
    fn get(&self, simulator: &Simulator) -> Result<RawData, SimulatorError> {
        match self {
            InputTarget::ImmediateInput(x) => Ok(x.clone()),
            InputTarget::LabelReferenceInput(s) => {
                let line = simulator.get_label_line_number(s)?;
                Ok(RawData::from_int(line, simulator.get_word_size()))
            }
            InputTarget::StringInput(s) => simulator
                .get_memory()
                .get_string_immediate_address(s)
                .map(|x| x.clone()),
            InputTarget::DereferenceInput(r, offset) => {
                let data = self.register_data(simulator);
                let address = match data {
                    Ok(x) => x.int_value() + offset,
                    Err(error) => return Err(error.into()),
                };

                if address < 0 {
                    return Err(SimulatorError::ReadNegativeAddressError(address));
                }

                simulator.get_memory().read(address as usize)
            }
            InputTarget::RegisterInput(r) => {
                let data = self.register_data(simulator);
                Ok(data?)
            }
        }
    }
}
