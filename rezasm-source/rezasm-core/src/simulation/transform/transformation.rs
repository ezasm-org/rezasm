use crate::{
    simulation::simulator::Simulator,
    util::{error::SimulatorError, raw_data::RawData},
};

use super::transformable::Transformable;

#[derive(Debug)]
pub struct Transformation {
    output: Transformable,
    from: RawData,
    to: RawData,
}

impl Transformation {
    pub fn get_to(&self) -> &RawData {
        &self.to
    }

    pub fn get_from(&self) -> &RawData {
        &self.from
    }

    pub fn new(output: Transformable, from: RawData, to: RawData) -> Transformation {
        Transformation { output, from, to }
    }
    pub fn invert(&mut self) -> Transformation {
        Transformation {
            output: self.output.clone(),
            from: self.to.clone(),
            to: self.from.clone(),
        }
    }

    pub fn apply(&self, simulator: &mut Simulator) -> Result<(), SimulatorError> {
        self.output.set(self.to.clone(), simulator)
    }

    pub fn is_nullop(&self) -> bool {
        self.output.is_nullop()
    }

    pub fn get_output(&self) -> Transformable {
        self.output
    }

    pub fn get_output_mut(&mut self) -> &mut Transformable {
        &mut self.output
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
