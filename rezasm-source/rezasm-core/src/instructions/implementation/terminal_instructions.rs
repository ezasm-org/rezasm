use std::io;

use lazy_static::lazy_static;

use crate::{instructions::{instruction::Instruction, targets::{input_output_target::InputOutputTarget, input_target::Input}, instruction_registry::register_instruction}, instruction, util::{raw_data::RawData, error::{IoError, ParserError}}};
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
    pub static ref READS: Instruction =
        instruction!(reads, |simulator: Simulator,
                            input1: InputOutputTarget,
                            input2: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let max_size = input2.get(simulator)?.int_value();
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).map_err(IoError::from)?;
            let mut s: String = buf.trim().parse().unwrap();
            s.truncate(max_size as usize);
            for c in s.chars() {
                simulator.get_memory_mut().write(address as usize, &RawData::from_char(c))?;
                address += simulator.get_memory().word_size().value() as i64;
            }
            simulator.get_memory_mut().write(address as usize, &RawData::from_char('\0'))?;
            Ok(())
        });

}

pub fn register_instructions() {
    register_instruction(&READI);
    register_instruction(&READF);
    register_instruction(&READC);
    register_instruction(&READS);
}