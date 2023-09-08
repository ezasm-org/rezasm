use crate::instructions::targets::Target;
use crate::simulation::simulator::Simulator;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;

pub trait Output: Target {
    fn set(&self, simulator: &mut Simulator, data: RawData) -> Result<(), SimulatorError>;
}
