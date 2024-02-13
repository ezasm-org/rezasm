use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::output_target::Output;
use crate::simulation::simulator::Simulator;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;

use super::transformation::Transformation;

#[derive(Copy, Clone, Debug)]
pub enum ReadType {
    Integer,
    Float,
    Character,
    SizedString,
    UnsizedString,
    Line
}

/// InputReadTransformable is primarily for signalling the simulator to enter AWAITING
#[derive(Copy, Debug)]
pub enum Transformable {
    FileReadTransformable(i64),
    HeapPointerTransformable,
    MemoryTransformable(usize),
    InputOutputTransformable(InputOutputTarget),
    TerminalReadTransformable{target: InputOutputTarget, read_type: ReadType, is_done: bool},
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
            _ => Ok(RawData::new(&[])),
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
            Transformable::FileReadTransformable(cursor) => todo!(), //must be todo until read instructions are done
            _ => Ok(())
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
            Self::TerminalReadTransformable { target, read_type, is_done: false} => true,
            _ => false,
        }
    }

    pub fn complete_read(&mut self) {
        match self {
            Self::TerminalReadTransformable { target, read_type, is_done: true} => {},
            Self::TerminalReadTransformable { target, read_type, ref mut is_done} => *is_done = true,
            _ => {}
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
            Transformable::TerminalReadTransformable{target, read_type, is_done} => Transformable::TerminalReadTransformable{target: target.clone(), read_type: read_type.clone(), is_done: is_done.clone()},
        }
    }
}
