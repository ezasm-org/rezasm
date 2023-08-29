use crate::error::EzasmError;
use crate::instructions::targets::Target;
use crate::simulation::simulator::Simulator;
use crate::util::raw_data::RawData;

pub trait Output: Target {
    fn set(&mut self, simulator: &mut Simulator, data: RawData) -> Result<(), EzasmError>;
}
