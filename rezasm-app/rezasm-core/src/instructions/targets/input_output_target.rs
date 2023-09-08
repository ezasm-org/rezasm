use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::output_target::Output;
use crate::simulation::registry;
use crate::simulation::simulator::Simulator;
use crate::util::error::EzasmError;
use crate::util::error::EzasmError::InvalidRegisterNumberError;
use crate::util::raw_data::RawData;

pub trait InputOutput: Input + Output {}

#[derive(Debug, Clone)]
pub enum InputOutputTarget {
    DereferenceInputOutput(usize, i64),
    RegisterInputOutput(usize),
}

impl InputOutput for InputOutputTarget {}

impl Input for InputOutputTarget {
    fn get(&self, simulator: &Simulator) -> Result<RawData, EzasmError> {
        let data = self.register_data(simulator);
        match self {
            InputOutputTarget::DereferenceInputOutput(_, offset) => {
                let address = match data {
                    Ok(x) => x.int_value() + offset,
                    Err(error) => return Err(error),
                };

                if address < 0 {
                    return Err(EzasmError::ReadNegativeAddressError(address))
                }

                simulator.get_memory().read(address as usize)
            },
            InputOutputTarget::RegisterInputOutput(_) => data,
        }
    }
}

impl Output for InputOutputTarget {
    fn set(&self, simulator: &mut Simulator, data: RawData) -> Result<(), EzasmError> {
        match self {
            InputOutputTarget::DereferenceInputOutput(r, offset) => {
                let address = match self.register_data(simulator) {
                    Ok(d) => d.int_value() + offset.clone(),
                    Err(e) => return Err(e),
                };

                if address < 0 {
                    return Err(EzasmError::WriteNegativeAddressError(address))
                }

                simulator.get_memory_mut().write(address as usize, &data)
            }
            InputOutputTarget::RegisterInputOutput(r) => simulator
                .get_registers_mut()
                .get_register_by_number_mut(r.clone())
                .map(|r| r.set_data(data)),
        }
    }
}

impl InputOutputTarget {
    fn register_data(&self, simulator: &Simulator) -> Result<RawData, EzasmError> {
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
    ) -> Result<InputOutputTarget, EzasmError> {
        if registry::is_valid_register_number(register) {
            Ok(InputOutputTarget::DereferenceInputOutput(register, offset))
        } else {
            Err(InvalidRegisterNumberError(register))
        }
    }

    pub fn new_register(register: &usize) -> Result<InputOutputTarget, EzasmError> {
        Ok(InputOutputTarget::RegisterInputOutput(register.clone()))
    }
}
