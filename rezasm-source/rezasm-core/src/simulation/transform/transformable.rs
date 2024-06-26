use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::output_target::Output;
use crate::simulation::simulator::Simulator;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;

use super::transformation::Transformation;

/// NullOpTransformable is primarily for signalling the simulator to enter AWAITING
#[derive(Copy, Debug)]
pub enum Transformable {
    FileReadTransformable(i64),
    HeapPointerTransformable,
    MemoryTransformable(usize),
    InputOutputTransformable(InputOutputTarget),
    NullOpTransformable,
}

impl Transformable {
    pub fn get(&self, simulator: &Simulator) -> Result<RawData, SimulatorError> {
        match self {
            Transformable::InputOutputTransformable(input_output) => input_output.get(simulator),
            Transformable::HeapPointerTransformable => Ok(RawData::from_int(
                simulator.get_memory().current_heap_pointer() as i64,
                simulator.get_word_size(),
            )),
            Transformable::MemoryTransformable(address) => {
                simulator.get_memory().read(address.clone())
            }
            Transformable::FileReadTransformable(cursor) => {
                Ok(RawData::from_int(cursor.clone(), simulator.get_word_size()))
            }
            Transformable::NullOpTransformable => {
                Ok(RawData::empty_data(simulator.get_word_size()))
            }
        }
    }

    pub fn set(&self, data: RawData, simulator: &mut Simulator) -> Result<(), SimulatorError> {
        match self {
            Transformable::InputOutputTransformable(input_output) => {
                input_output.set(simulator, data)
            }
            Transformable::HeapPointerTransformable => simulator
                .get_memory_mut()
                .set_heap_pointer(data.int_value() as usize),
            Transformable::MemoryTransformable(address) => {
                simulator.get_memory_mut().write(address.clone(), &data)
            }
            Transformable::FileReadTransformable(cursor) => todo!(),
            Transformable::NullOpTransformable => Ok(()),
        }
    }

    pub fn create_transformation(
        &self,
        simulator: &Simulator,
        output: RawData,
    ) -> Result<Transformation, SimulatorError> {
        Ok(Transformation::new(
            self.clone(),
            self.get(simulator)?,
            output,
        ))
    }

    pub fn is_nullop(&self) -> bool {
        match self {
            Transformable::NullOpTransformable => true,
            _ => false,
        }
    }
}

impl Clone for Transformable {
    fn clone(&self) -> Transformable {
        match self {
            Transformable::MemoryTransformable(address) => {
                Transformable::MemoryTransformable(address.clone())
            }
            Transformable::HeapPointerTransformable => Transformable::HeapPointerTransformable,
            Transformable::InputOutputTransformable(input_output) => {
                Transformable::InputOutputTransformable(input_output.clone())
            }
            Transformable::FileReadTransformable(cursor) => {
                Transformable::FileReadTransformable(cursor.clone())
            }
            Transformable::NullOpTransformable => Transformable::NullOpTransformable,
        }
    }
}
