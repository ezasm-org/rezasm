use std::io;

use lazy_static::lazy_static;

use crate::{instructions::{instruction::Instruction, targets::{input_output_target::InputOutputTarget, input_target::Input}, instruction_registry::register_instruction}, instruction, util::{raw_data::RawData, error::{IoError, ParserError}}};
use crate::instructions::targets::output_target::Output;


lazy_static! {
    pub static ref READI: Instruction =
        instruction!(readi, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let k = simulator.terminal_stream().unwrap().next_i64().map_err(IoError::from)?.unwrap();
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let k = simulator.terminal_stream().unwrap().next_f64().map_err(IoError::from)?.unwrap();
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let k = simulator.terminal_stream().unwrap().next_char().map_err(IoError::from)?.unwrap();
            return output.set(simulator, RawData::from_char(k));
        });
    pub static ref READS: Instruction =
        instruction!(reads, |simulator: Simulator,
                            input1: InputOutputTarget,
                            input2: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let max_size = input2.get(simulator)?.int_value();
            let s = simulator.terminal_stream().unwrap().next_bytes(max_size as usize).map_err(IoError::from)?.unwrap();
            let s = String::from_utf8(s).unwrap();
            for c in s.chars() {
                simulator.get_memory_mut().write(address as usize, &RawData::from_char(c))?;
                address += simulator.get_memory().word_size().value() as i64;
            }
            simulator.get_memory_mut().write(address as usize, &RawData::from_char('\0'))?;
            Ok(())
        });
    pub static ref READS2: Instruction =
        instruction!(reads, |simulator: Simulator,
                            input1: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let s = simulator.terminal_stream().unwrap().next().map_err(IoError::from)?.unwrap();
            for c in s.chars() {
                simulator.get_memory_mut().write(address as usize, &RawData::from_char(c))?;
                address += simulator.get_memory().word_size().value() as i64;
            }
            simulator.get_memory_mut().write(address as usize, &RawData::from_char('\0'))?;
            Ok(())
        });
    pub static ref READLN: Instruction =
        instruction!(readln, |simulator: Simulator,
                            input1: InputOutputTarget,
                            input2: InputOutputTarget| {
            /*
            For some reason this scanner crate has *type*_until() for every type instead of just taking
            a closure that returns a bool for comparing??? So there isn't really a simple way to get both
            use next_bytes() and read_line().

            let mut address = input1.get(simulator)?.int_value();
            let max_size = input2.get(simulator)?.int_value();
            let s = simulator.terminal_stream().unwrap().next_bytes(max_size as usize).map_err(IoError::from)?.unwrap();
            let s = String::from_utf8(s).unwrap();
            for c in s.chars() {
                simulator.get_memory_mut().write(address as usize, &RawData::from_char(c))?;
                address += simulator.get_memory().word_size().value() as i64;
            }
            simulator.get_memory_mut().write(address as usize, &RawData::from_char('\0'))?;
            */
            Ok(())
        });
    pub static ref READLN2: Instruction =
        instruction!(readln, |simulator: Simulator,
                            input1: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let s = simulator.terminal_stream().unwrap().next_line().map_err(IoError::from)?.unwrap();
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
    register_instruction(&READS2);
    register_instruction(&READLN);
    register_instruction(&READLN2);
}