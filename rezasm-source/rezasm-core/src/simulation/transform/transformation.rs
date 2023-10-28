use crate::{
    simulation::simulator::Simulator,
    util::{error::SimulatorError, raw_data::RawData},
};

use super::transformable::Transformable;

pub struct Transformation {
    output: Transformable,
    from: RawData,
    to: RawData,
}

impl Transformation {
    pub fn invert(&mut self) -> Transformation {
        Transformation {
            output: self.output.clone(),
            from: self.to.clone(),
            to: self.from.clone(),
        }
    }

    pub fn apply(&mut self, simulator: &mut Simulator) -> Result<(), SimulatorError> {
        self.output.set(self.to.clone(), simulator)
    }
}

impl Clone for Transformation {
    fn clone(&self) -> Transformation {
        Transformation {
            output: self.output.clone(),
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}
