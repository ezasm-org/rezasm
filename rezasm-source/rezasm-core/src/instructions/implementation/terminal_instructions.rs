use std::io;

use lazy_static::lazy_static;

use crate::{instructions::{instruction::Instruction, targets::input_output_target::InputOutputTarget, instruction_registry::register_instruction}, instruction, util::{raw_data::RawData, error::{SimulatorError, IoError, ParserError}}};
use crate::instructions::targets::output_target::Output;


lazy_static! {
    pub static ref READI: Instruction =
        instruction!(readi, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(IoError::from)?;
            let k: i64 = buf.trim().parse().map_err(ParserError::from)?;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(IoError::from)?;
            let k: f64 = buf.trim().parse().map_err(ParserError::from)?;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(IoError::from)?;
            let k: char = buf.trim().parse().map_err(ParserError::from)?;
            return output.set(simulator, RawData::from_char(k));
        });
}

pub fn register_instructions() {
    register_instruction(&READI);
    register_instruction(&READF);
    register_instruction(&READC);
}