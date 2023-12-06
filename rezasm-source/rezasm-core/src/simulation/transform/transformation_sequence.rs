use crate::{simulation::simulator::Simulator, util::error::SimulatorError};

use super::transformation::Transformation;

#[derive(Debug)]
pub struct TransformationSequence {
    transformations: Vec<Transformation>,
}

impl TransformationSequence {
    pub fn new_single(transformation: Transformation) -> TransformationSequence {
        TransformationSequence { transformations: vec![transformation] }
    }

    pub fn new(transformations: Vec<Transformation>) -> TransformationSequence {
        TransformationSequence { transformations }
    }

    pub fn new_empty() -> TransformationSequence {
        TransformationSequence {
            transformations: vec![],
        }
    }

    pub fn concatenate(&mut self, other: TransformationSequence) {
        self.transformations.extend(other.transformations.clone())
    }

    pub fn invert(&mut self) -> TransformationSequence {
        TransformationSequence {
            transformations: self
                .transformations
                .clone()
                .iter()
                .rev()
                .map(|transformation| -> Transformation { transformation.clone().invert() })
                .collect(),
        }
    }

    pub fn apply(&mut self, simulator: &mut Simulator) -> Result<(), SimulatorError> {
        for transformation in &mut self.transformations {
            transformation.apply(simulator)?
        }
        Ok(())
    }
}
