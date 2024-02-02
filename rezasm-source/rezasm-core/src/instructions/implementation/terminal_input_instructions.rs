use crate::instructions::targets::output_target::Output;
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

lazy_static! {
    pub static ref READI: Instruction =
        instruction!(readi, |simulator: Simulator, output: InputOutputTarget| {
            let mut buf = [0u8; 8]; // breaks on input of 123456789, or any other big integer
            simulator.get_reader_mut().read(&mut buf).unwrap();
            let integer_string = String::from_utf8_lossy(&buf)
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>();
            let integer = match integer_string.parse::<i64>() {
                Ok(i) => RawData::from_int(i, simulator.get_word_size()),
                Err(e) => {
                    match e.kind() {
                        std::num::IntErrorKind::Empty => {
                            let nullop = Transformable::NullOpTransformable.create_transformation(simulator, RawData::from_int(-1, simulator.get_word_size()))?;
                            return Ok(TransformationSequence::new_single(nullop));
                        },
                        _ => return Err(crate::util::error::SimulatorError::IoError(crate::util::error::IoError::ReadError))
                    }
                }
            };
            let transformation = Transformable::InputOutputTransformable(output).create_transformation(simulator, integer)?;
            Ok(TransformationSequence::new_single(transformation))
        });
    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, output: InputOutputTarget| {
            let mut buf = [0u8; 8];
            simulator.get_reader_mut().read(&mut buf).unwrap();
            let float_string = String::from_utf8_lossy(&buf)
                .chars()
                .filter(|c| c.is_numeric() || *c == '.')
                .collect::<String>();
            let float = float_string.parse::<f64>().unwrap();
            output.set(
                simulator,
                RawData::from_float(float, simulator.get_word_size()),
            );
            Ok(TransformationSequence::new_empty())
        });
    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, output: InputOutputTarget| {
            let mut buf = [0u8; 4];
            simulator.get_reader_mut().read(&mut buf).unwrap();
            let char = String::from_utf8_lossy(&buf).chars().next().unwrap();
            output.set(
                simulator,
                RawData::from_int(char as i64, simulator.get_word_size()),
            );
            Ok(TransformationSequence::new_empty())
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
            todo!();
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

pub fn register_instructions() {
    register_instruction(&READI);
    register_instruction(&READF);
    register_instruction(&READC);
    register_instruction(&READS);
    register_instruction(&READS_UNSIZED);
    register_instruction(&READLN);
    register_instruction(&READLN_UNSIZED);
}
