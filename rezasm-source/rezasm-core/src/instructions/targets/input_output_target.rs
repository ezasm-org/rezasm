use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::output_target::Output;
use crate::simulation::registry;
use crate::simulation::simulator::Simulator;
use crate::util::error::{ParserError, SimulatorError};
use crate::util::raw_data::RawData;

pub trait InputOutput: Input + Output {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputOutputTarget {
    DereferenceInputOutput(usize, i64),
    RegisterInputOutput(usize),
}

impl InputOutput for InputOutputTarget {}

impl Input for InputOutputTarget {
    fn get(&self, simulator: &Simulator) -> Result<RawData, SimulatorError> {
        let data = self.register_data(simulator);
        match self {
            InputOutputTarget::DereferenceInputOutput(_, offset) => {
                let address = match data {
                    Ok(x) => x.int_value() + offset,
                    Err(error) => return Err(error.into()),
                };

                if address < 0 {
                    return Err(SimulatorError::ReadNegativeAddressError(address));
                }

                simulator.get_memory().read(address as usize)
            }
            InputOutputTarget::RegisterInputOutput(_) => Ok(data?),
        }
    }
}

impl Output for InputOutputTarget {
    fn set(&self, simulator: &mut Simulator, data: RawData) -> Result<(), SimulatorError> {
        match self {
            InputOutputTarget::DereferenceInputOutput(r, offset) => {
                let address = self.register_data(simulator)?.int_value() + offset.clone();

                if address < 0 {
                    return Err(SimulatorError::WriteNegativeAddressError(address));
                }

                simulator.get_memory_mut().write(address as usize, &data)
            }
            InputOutputTarget::RegisterInputOutput(r) => Ok(simulator
                .get_registers_mut()
                .get_register_by_number_mut(r.clone())?
                .set_data(data)),
        }
    }
}

impl InputOutputTarget {
    fn register_data(&self, simulator: &Simulator) -> Result<RawData, ParserError> {
        let register = match self {
            InputOutputTarget::DereferenceInputOutput(r, _) => {
                simulator.get_registers().get_register_by_number(r.clone())
            }
            InputOutputTarget::RegisterInputOutput(r) => {
                simulator.get_registers().get_register_by_number(r.clone())
            }
        };
        match register {
            Ok(r) => Ok(r.get_data().clone()),
            Err(error) => Err(error),
        }
    }

    pub fn new_dereference_offset(
        register: usize,
        offset: i64,
    ) -> Result<InputOutputTarget, ParserError> {
        if registry::is_valid_register_number(register) {
            Ok(InputOutputTarget::DereferenceInputOutput(register, offset))
        } else {
            Err(ParserError::InvalidRegisterNumberError(register))
        }
    }

    pub fn new_register(register: &usize) -> Result<InputOutputTarget, ParserError> {
        Ok(InputOutputTarget::RegisterInputOutput(register.clone()))
    }
}
