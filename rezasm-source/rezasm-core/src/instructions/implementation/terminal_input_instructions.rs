use crate::simulation::transform::transformable::Transformable;
use crate::simulation::transform::transformation::Transformation;
use crate::util::error::IoError;
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

            let integer = scanner.next_i64()?.ok_or(IoError::ReadError)?;
            let data = RawData::from_int(integer, simulator.get_word_size());

            let transformation = Transformable::InputOutputTransformable(output)
                .create_transformation(simulator, data)?;
            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());
            let float = scanner.next_f64()?.ok_or(IoError::ReadError)?;

            let data = RawData::from_float(float, simulator.get_word_size());
            let transformation = Transformable::InputOutputTransformable(output)
                .create_transformation(simulator, data)?;
            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());
            let char = scanner.next_char()?.ok_or(IoError::ReadError)?;

            let data = RawData::from_int(char as i64, simulator.get_word_size());
            let transformation = Transformable::InputOutputTransformable(output)
                .create_transformation(simulator, data)?;
            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READS: Instruction = instruction!(
        reads,
        |simulator: Simulator, input1: InputOutputTarget, input2: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let max_size = input2.get(simulator)?.int_value();
            let mut s = vec![0; max_size as usize];
            let word_size = simulator.get_word_size().clone();
            simulator.get_reader_mut().read_exact(&mut s).unwrap();
            let s = String::from_utf8(s).unwrap();
            for b in s.bytes() {
                simulator
                    .get_memory_mut()
                    .write(address as usize, &RawData::from_int(b as i64, &word_size))?;
                address += simulator.get_memory().word_size().value() as i64;
            }
            simulator.get_memory_mut().write(
                address as usize,
                &RawData::from_int('\0' as i64, &word_size),
            )?;
            Ok(TransformationSequence::new_empty())
        }
    );

    pub static ref READS_UNSIZED: Instruction =
        instruction!(reads, |simulator: Simulator, input1: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let bytes = scanner.next_raw()?.ok_or(IoError::ReadError)?;

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();
            bytes.into_iter().enumerate()
                .map(|(offset, byte)| (address + offset * word_size, byte))
                .map(|(address, byte)| Transformation::new(output, output.value, byte));
            // FIX: need correct way to get output and output value for Transformation

            Ok(TransformationSequence::new_empty())
        });

    pub static ref READLN: Instruction = instruction!(
        readln,
        |simulator: Simulator, input1: InputOutputTarget, input2: InputOutputTarget| {
            todo!();
        }
    );

    pub static ref READLN_UNSIZED: Instruction =
        instruction!(readln, |simulator: Simulator, input1: InputOutputTarget| {
            todo!();
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
