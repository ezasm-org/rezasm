use crate::{simulation::simulator::Simulator, util::{error::SimulatorError, raw_data::RawData}};

use super::{transformable::Transformable, transformation::Transformation};

#[derive(Debug, Clone)]
pub struct TransformationSequence {
    transformations: Vec<Transformation>,
}

impl TransformationSequence {
    pub fn new_single(transformation: Transformation) -> TransformationSequence {
        TransformationSequence {
            transformations: vec![transformation],
        }
    }

    pub fn new(transformations: Vec<Transformation>) -> TransformationSequence {
        TransformationSequence { transformations }
    }

    pub fn new_empty() -> TransformationSequence {
        TransformationSequence {
            transformations: vec![],
        }
    }

    pub fn new_nullop(simulator: &Simulator) -> Result<TransformationSequence, SimulatorError> {
        let word_size = simulator.get_word_size();
        let data = RawData::empty_data(word_size);
        let transformation = Transformable::NullOpTransformable.create_transformation(simulator, data)?;
        Ok(TransformationSequence::new_single(transformation))
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

    pub fn contains_nullop(&self) -> bool {
        for t in &self.transformations {
            if t.is_nullop() {
                return true;
            }
        }
        false
    }


    pub fn apply(&self, simulator: &mut Simulator) -> Result<(), SimulatorError> {
        for transformation in &self.transformations {
            transformation.apply(simulator)?
        }
        Ok(())
    }
}
