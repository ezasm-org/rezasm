use crate::simulation::transform::transformable::Transformable;
use crate::{
    instruction,
    instructions::{
        instruction::Instruction,
        instruction_registry::register_instruction,
        targets::{input_output_target::InputOutputTarget, input_target::Input},
    },
    simulation::transform::transformation_sequence::TransformationSequence,
    util::raw_data::RawData,
};
use lazy_static::lazy_static;
use scanner_rust::ScannerAscii;

lazy_static! {

    /// Definition of the `readi` intruction, used to read an integer
    pub static ref READI: Instruction =
        instruction!(readi, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(num) = scanner.next_i64()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let word_size = simulator.get_word_size();

            let data = RawData::from_int(num, word_size);
            let transformation = Transformable::InputOutputTransformable(output)
                .create_transformation(simulator, data)?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(num) = scanner.next_f64()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let word_size = simulator.get_word_size();

            let data = RawData::from_float(num, word_size);
            let transformation = Transformable::InputOutputTransformable(output).create_transformation(simulator, data)?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(ch) = scanner.next_char()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let word_size = simulator.get_word_size();

            let data = RawData::from_int(ch as i64, word_size);
            let transformation = Transformable::InputOutputTransformable(output).create_transformation(simulator, data)?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READS: Instruction = instruction!(
        reads,
        |simulator: Simulator, input1: InputOutputTarget, input2: InputOutputTarget| {

            let len = input2.get(simulator)?.int_value() as usize;
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(mut bytes) = scanner.next_bytes(len)? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };
            bytes.push(b'\0');

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(bytes.len())
                .create_transformation(simulator, RawData::new(&bytes))?;

            Ok(TransformationSequence::new_single(transformation))
        }
    );

    pub static ref READS_UNSIZED: Instruction =
        instruction!(reads, |simulator: Simulator, input1: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(mut bytes) = scanner.next()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };
            bytes.push('\0');

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(bytes.len())
                .create_transformation(simulator, RawData::new(bytes.as_bytes()))?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READLN: Instruction = instruction!(
        readln,
        |simulator: Simulator, input1: InputOutputTarget, input2: InputOutputTarget| {
            let len = input2.get(simulator)?.int_value() as usize;
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(bytes) = scanner.next_line()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(bytes.len())
                .create_transformation(simulator, RawData::new(&bytes.as_bytes()[0..len]))?;

            Ok(TransformationSequence::new_single(transformation))
        }
    );

    pub static ref READLN_UNSIZED: Instruction =
        instruction!(readln, |simulator: Simulator, input1: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(bytes) = scanner.next_line()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(bytes.len())
                .create_transformation(simulator, RawData::new(bytes.as_bytes()))?;

            Ok(TransformationSequence::new_single(transformation))
        });
}

/// Registers the instructions found in this file
pub fn register_instructions() {
    register_instruction(&READI);
    register_instruction(&READF);
    register_instruction(&READC);
    register_instruction(&READS);
    register_instruction(&READS_UNSIZED);
    register_instruction(&READLN);
    register_instruction(&READLN_UNSIZED);
}
