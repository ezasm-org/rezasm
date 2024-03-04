use crate::simulation::simulator::Simulator;
use crate::simulation::transform::transformable::{self, Transformable};
use crate::simulation::transform::transformation::{self, Transformation};
use crate::util::error::IoError;
use crate::util::word_size::{self, WordSize};
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
            let word_size = simulator.get_word_size();

            let (transformable, data) = match scanner.next_i64()? {
                Some(int) => (Transformable::InputOutputTransformable(output), RawData::from_int(int, word_size)),
                None => (Transformable::NullOpTransformable, RawData::empty_data(word_size)),
            };

            let transformation = transformable.create_transformation(simulator, data)?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());
            let word_size = simulator.get_word_size();

            let (transformable, data) = match scanner.next_f64()? {
                Some(float) => (Transformable::InputOutputTransformable(output), RawData::from_float(float, word_size)),
                None => (Transformable::NullOpTransformable, RawData::empty_data(word_size)),
            };

            let transformation = transformable.create_transformation(simulator, data)?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, output: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());
            let word_size = simulator.get_word_size();

            // FIX: for some reason the type system hates this line
            // FIX: the word size here should be one, but I don't know what function to use for this
            let (transformable, data): (Transformable, RawData) = match scanner.next_char()? {
                Some(char) => (Transformable::InputOutputTransformable(output), RawData::(char as i64, word_size)),
                None => (Transformable::NullOpTransformable, RawData::empty_data(word_size)),
            };

            let transformation = transformable.create_transformation(simulator, data)?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READS: Instruction = instruction!(
        reads,
        |simulator: Simulator, input1: InputOutputTarget, input2: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());
            let word_size = simulator.get_word_size();

            // Get all relevant address stuff as integers for math
            let word_bytes = word_size.value();
            let target = input1.get(simulator)?.int_value() as usize;
            let len = input2.get(simulator)?.int_value() as usize;

            let string = scanner.next_bytes(len)?;

            // Return NullOp if input is whitespace
            let Some(string) = string else {
                return TransformationSequence::new_nullop(simulator);
            };

            let transformation_vec = string.iter().enumerate()
                // FIX: This skips all invalid transformations, which may not be the desired behavior
                .filter_map(|(offset, _)| {
                    let transformable = Transformable::MemoryTransformable(target + offset * word_bytes);
                    let data = RawData::new(&[string[offset]]);
                    Some(transformable.create_transformation(simulator, data).ok()?)
                })
                .collect::<Vec<_>>();

            Ok(TransformationSequence::new(transformation_vec))
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
