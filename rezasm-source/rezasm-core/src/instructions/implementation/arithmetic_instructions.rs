use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::simulation::register::Register;
use crate::simulation::registry;
use crate::simulation::transform::transformable::Transformable;
use crate::simulation::transform::transformation::Transformation;
use crate::simulation::transform::transformation_sequence::TransformationSequence;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

lazy_static! {
    pub static ref ADD: Instruction =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 + value2;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SUB: Instruction =
        instruction!(sub, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 - value2;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref MUL: Instruction =
        instruction!(mul, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let result: i128 = value1 as i128 * value2 as i128;

            let word_size = simulator.get_word_size().clone();

            let hi: &mut Register = simulator
                .get_registers_mut()
                .get_register_mut(&registry::HI.to_string())?;
            let full: i128 = (value1 as i128) * (value2 as i128);

            hi.set_data(RawData::from_int(
                match word_size {
                    WordSize::Four => (i128::abs(full) >> 32) as i64,
                    WordSize::Eight => (i128::abs(full) >> 64) as i64,
                },
                &word_size,
            ));

            let lo: &mut Register = simulator
                .get_registers_mut()
                .get_register_mut(&registry::LO.to_string())?;
            lo.set_data(RawData::from_int(full as i64, &word_size));

            let k = value1 * value2;

            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref DIV: Instruction =
        instruction!(div, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value2 == 0 {
                return Err(SimulatorError::DivideByZeroError);
            } else {
                let k = value1 / value2;
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    RawData::from_int(k, simulator.get_word_size()),
                );
                return Ok(TransformationSequence::new_single(transformation));
            }
        });
    pub static ref AND: Instruction =
        instruction!(and, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 & value2;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref OR: Instruction =
        instruction!(or, |simulator: Simulator,
                          output: InputOutputTarget,
                          input1: InputTarget,
                          input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 | value2;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref XOR: Instruction =
        instruction!(xor, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 ^ value2;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref NOT: Instruction =
        instruction!(not, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let k = !value1;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref MOD: Instruction =
        instruction!(_mod, |simulator: Simulator,
                            output: InputOutputTarget,
                            input1: InputTarget,
                            input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value2 == 0 {
                return Err(SimulatorError::DivideByZeroError);
            } else {
                let k = value1 % value2;
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    RawData::from_int(k, simulator.get_word_size()),
                );
                return Ok(TransformationSequence::new_single(transformation));
            }
        });
    pub static ref SLL: Instruction =
        instruction!(sll, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value = input1.get(&simulator)?.int_value();
            let shift = input2.get(&simulator)?.int_value() as u64;
            let word_size = simulator.get_word_size();

            let k = if shift >= (word_size.value() as u64 * 8) {
                0
            } else {
                value << shift
            };

            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SRL: Instruction =
        instruction!(srl, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value = input1.get(&simulator)?.int_value();
            let shift = input2.get(&simulator)?.int_value() as u64;
            let word_size = simulator.get_word_size();

            let k = if shift >= (word_size.value() as u64 * 8) {
                0
            } else {
                match word_size {
                    WordSize::Four => (value as u32 >> shift) as i64,
                    WordSize::Eight => (value as u64 >> shift) as i64,
                }
            };

            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref SRA: Instruction =
        instruction!(sra, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value = input1.get(&simulator)?.int_value();
            let shift = input2.get(&simulator)?.int_value() as u64;
            let word_size = simulator.get_word_size();

            let k = if shift >= (word_size.value() as u64 * 8) {
                value >> 63
            } else {
                value >> shift
            };

            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref INC: Instruction =
        instruction!(inc, |simulator: Simulator, output: InputOutputTarget| {
            let value = output.get(&simulator)?.int_value();
            let k = value + 1;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
    pub static ref DEC: Instruction =
        instruction!(dec, |simulator: Simulator, output: InputOutputTarget| {
            let value = output.get(&simulator)?.int_value();
            let k = value - 1;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                RawData::from_int(k, simulator.get_word_size()),
            );
            return Ok(TransformationSequence::new_single(transformation));
        });
}

pub fn register_instructions() {
    register_instruction(&ADD);
    register_instruction(&SUB);
    register_instruction(&MUL);
    register_instruction(&DIV);
    register_instruction(&AND);
    register_instruction(&OR);
    register_instruction(&XOR);
    register_instruction(&MOD);
    register_instruction(&NOT);
    register_instruction(&SLL);
    register_instruction(&SRL);
    register_instruction(&SRA);
    register_instruction(&INC);
    register_instruction(&DEC);
}
