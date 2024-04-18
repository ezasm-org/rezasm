use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::simulation::transform::transformable::Transformable;
use crate::simulation::transform::transformation::Transformation;
use crate::simulation::transform::transformation_sequence::TransformationSequence;
use crate::util::raw_data::RawData;

lazy_static! {
    pub static ref SEQ: Instruction =
        instruction!(seq, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = match value1 == value2 {
                true => 1,
                false => 0,
            };
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SNE: Instruction =
        instruction!(sne, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = match value1 != value2 {
                true => 1,
                false => 0,
            };
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SLT: Instruction =
        instruction!(slt, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = match value1 < value2 {
                true => 1,
                false => 0,
            };
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SLE: Instruction =
        instruction!(sle, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = match value1 <= value2 {
                true => 1,
                false => 0,
            };
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SGT: Instruction =
        instruction!(sgt, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = match value1 > value2 {
                true => 1,
                false => 0,
            };
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SGE: Instruction =
        instruction!(sge, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = match value1 >= value2 {
                true => 1,
                false => 0,
            };
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
}

pub fn register_instructions() {
    register_instruction(&SEQ);
    register_instruction(&SNE);
    register_instruction(&SLT);
    register_instruction(&SLE);
    register_instruction(&SGT);
    register_instruction(&SGE);
}
