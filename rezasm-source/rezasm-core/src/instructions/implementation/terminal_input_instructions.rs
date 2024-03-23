use crate::instructions::targets::input_target::InputTarget;
use crate::simulation::reader::ReaderBox;
use crate::simulation::transform::transformable::Transformable;
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

    /// Definition of the `readi` instruction, used to read an integer
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
        |simulator: Simulator, input1: InputOutputTarget, input2: InputTarget| {
            let len = input2.get(simulator)?.int_value() as usize;
            if len == 1 {
                return Ok(TransformationSequence::new_empty());
            }
            let mut bytes = vec![0u8; len - 1];
            let read_count = read_to_sized(simulator.get_reader_mut(), &mut bytes, |c| {
                c.is_ascii_whitespace()
            })
                .map_err(IoError::from)?;

            if read_count == 0 {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let mut words = pad_bytes(&bytes[0..read_count]);
            words.append(&mut vec![0u8; 4]);

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(address)
                .create_transformation(simulator, RawData::new(&words))?;

            Ok(TransformationSequence::new_single(transformation))
        }
    );

    pub static ref READS_UNSIZED: Instruction =
        instruction!(reads, |simulator: Simulator, input1: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(input) = scanner.next()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let mut words = pad_bytes(input.as_bytes());
            words.append(&mut vec![0u8; 4]);

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(address)
                .create_transformation(simulator, RawData::new(&words))?;

            Ok(TransformationSequence::new_single(transformation))
        });

    pub static ref READLN: Instruction = instruction!(
        readln,
        |simulator: Simulator, input1: InputOutputTarget, input2: InputTarget| {
            let len = input2.get(simulator)?.int_value() as usize;
            if len == 1 {
                return Ok(TransformationSequence::new_empty());
            }
            let mut bytes = vec![0u8; len - 1];
            let read_count = read_to_sized(simulator.get_reader_mut(), &mut bytes, |c| {
                *c == '\n' as u8
            })
                .map_err(IoError::from)?;

            if read_count == 0 {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let mut words = pad_bytes(&bytes[0..read_count]);
            words.append(&mut vec![0u8; 4]);

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(address)
                .create_transformation(simulator, RawData::new(&words))?;

            Ok(TransformationSequence::new_single(transformation))
        }
    );

    pub static ref READLN_UNSIZED: Instruction =
        instruction!(readln, |simulator: Simulator, input1: InputOutputTarget| {
            let mut scanner = ScannerAscii::new(simulator.get_reader_mut());

            let Some(input) = scanner.next_line()? else {
                return Ok(TransformationSequence::new_nullop(simulator)?);
            };

            let mut words = pad_bytes(input.as_bytes());
            words.append(&mut vec![0u8; 4]);

            let address = input1.get(simulator)?.int_value() as usize;
            let word_size = simulator.get_word_size().value();

            let transformation = Transformable::MemoryTransformable(address)
                .create_transformation(simulator, RawData::new(&words))?;

            Ok(TransformationSequence::new_single(transformation))
        });
}

fn pad_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .map(|byte| vec![0u8, 0u8, 0u8, *byte])
        .flatten()
        .collect()
}

/// Uses a boxed custom reader to read until whitespace or a size is reached.
///
/// # Arguments
///
/// * `reader` - the boxed reader used to get input.
/// * `target` - the buffer to which to read.
///
/// # Returns
///
/// * `()` - if the read works.
/// * `io::Error` - if the read fails for some reason.
fn read_to_sized(
    reader: &mut ReaderBox,
    target: &mut [u8],
    terminator: fn(&u8) -> bool,
) -> std::io::Result<usize> {
    let mut buf = [0u8];
    let len = target.len();
    for idx in 0usize..len {
        if reader.read(&mut buf)? == 0usize {
            buf[0] = 0u8;
        }

        if terminator(&buf[0]) || buf[0] == 0u8 {
            return Ok(idx);
        }

        target[idx] = buf[0];
    }

    Ok(len)
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
