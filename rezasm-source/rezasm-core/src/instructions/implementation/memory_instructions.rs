use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::simulation::registry;
use crate::simulation::simulator::Simulator;
use crate::simulation::transform::transformable::Transformable;
use crate::simulation::transform::transformation::Transformation;
use crate::simulation::transform::transformation_sequence::TransformationSequence;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;

fn consecutive_push(simulator: &Simulator, input: InputTarget, times: i64) -> Result<TransformationSequence, SimulatorError> {
    let offset = times * simulator.get_word_size().value() as i64;
    let sp_target = InputOutputTarget::new_register(&registry::SP_NUMBER)?;
    let sp = sp_target.get(simulator)?.int_value() - simulator.get_word_size().value() as i64 - offset;
    let sp_transformable = Transformable::InputOutputTransformable(sp_target);

    let t1 = sp_transformable.create_transformation(simulator, RawData::from_int(sp, simulator.get_word_size()))?;

    let memory_transformer = Transformable::MemoryTransformable(t1.get_to().int_value() as usize);
    let t2 = Transformation::new(
        memory_transformer,
        memory_transformer.get(simulator)?,
        input.get(simulator)?
        );
    Ok(TransformationSequence::new(vec![t1, t2]))
}

fn consecutive_pop(simulator: &Simulator, output: InputOutputTarget, times: i64) -> Result<TransformationSequence, SimulatorError> {
    let offset = times * simulator.get_word_size().value() as i64;
    let sp_target = InputOutputTarget::new_register(&registry::SP_NUMBER)?;
    let io = Transformable::InputOutputTransformable(output);
    let sp = sp_target.get(simulator)?.int_value() - simulator.get_word_size().value() as i64 + offset;
    let t1 = io.create_transformation(simulator, RawData::from_int(simulator.get_memory().read(sp_target.get(simulator)?.int_value() as usize)?.int_value() + offset, simulator.get_word_size()))?;
    let t2 = Transformable::InputOutputTransformable(sp_target).create_transformation(simulator, RawData::from_int(sp_target.get(simulator)?.int_value() + simulator.get_word_size().value() as i64 + offset, simulator.get_word_size()))?;
    Ok(TransformationSequence::new(vec![t1, t2]))
}

lazy_static! {
    pub static ref PUSH: Instruction =
        instruction!(push, |simulator: Simulator, input: InputTarget| {
            consecutive_push(simulator, input, 0)
        });
    pub static ref POP: Instruction =
        instruction!(pop, |simulator: Simulator, output: InputOutputTarget| {
            consecutive_pop(simulator, output, 0)
        });
    pub static ref LOAD: Instruction =
        instruction!(load, |simulator: Simulator,
                            output: InputOutputTarget,
                            input: InputTarget| {
            let memory = simulator.get_memory();
            let word = memory.read(input.get(simulator)?.int_value() as usize)?;
            let out_transformable = Transformable::InputOutputTransformable(output);
            Ok(TransformationSequence::new_single(out_transformable.create_transformation(simulator, input.get(simulator)?)?))
        });
    pub static ref STORE: Instruction =
        instruction!(store, |simulator: Simulator,
                             input1: InputTarget,
                             input2: InputTarget| {
            let address = input2.get(simulator)?.int_value() as usize;
            let data = input1.get(simulator)?;
            let memory = simulator.get_memory_mut();
            memory.write(address, &data)
        });
    pub static ref ALLOC: Instruction =
        instruction!(alloc, |simulator: Simulator,
                             output: InputOutputTarget,
                             input: InputTarget| {
            let offset = input.get(simulator)?.int_value() as usize;
            let memory = simulator.get_memory_mut();
            let heap_pointer = memory.current_heap_pointer();
            memory.set_heap_pointer(heap_pointer + offset)?;
            let bytes = RawData::from_int(heap_pointer as i64, simulator.get_word_size());
            output.set(simulator, bytes)
        });
    pub static ref MOVE: Instruction =
        instruction!(_move, |simulator: Simulator,
                             output: InputOutputTarget,
                             input: InputTarget| {
            output.set(simulator, input.get(simulator)?)
        });
}

pub fn register_instructions() {
    register_instruction(&PUSH);
    register_instruction(&POP);
    register_instruction(&LOAD);
    register_instruction(&STORE);
    register_instruction(&ALLOC);
    register_instruction(&MOVE);
}
