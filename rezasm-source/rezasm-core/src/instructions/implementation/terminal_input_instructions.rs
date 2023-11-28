use std::io;

use lazy_static::lazy_static;

use crate::{instructions::{instruction::Instruction, targets::{input_output_target::InputOutputTarget, input_target::Input}, instruction_registry::register_instruction}, instruction, util::{raw_data::RawData, error::{IoError, ParserError}}};
use crate::instructions::targets::output_target::Output;


lazy_static! {
    pub static ref READI: Instruction =
        instruction!(readi, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let mut buf = [0u8; 8];
            simulator.get_reader_mut().read(&mut buf).unwrap();
            println!("{:?}", buf);
            let k = i64::from_le_bytes(buf);
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref READF: Instruction =
        instruction!(readf, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let mut buf = [0u8; 8];
            simulator.get_reader_mut().read(&mut buf).unwrap();
            let k = f64::from_be_bytes(buf);
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref READC: Instruction =
        instruction!(readc, |simulator: Simulator, 
                            output: InputOutputTarget| {
            let mut buf = [0u8; 4];
            simulator.get_reader_mut().read(&mut buf).unwrap();
            let u = u32::from_be_bytes(buf);
            let k = char::from_u32(u).ok_or(IoError::ReadError)?;
            return output.set(simulator, RawData::from_char(k));
        });
    pub static ref READS: Instruction =
        instruction!(reads, |simulator: Simulator,
                            input1: InputOutputTarget,
                            input2: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let max_size = input2.get(simulator)?.int_value();
            let mut s =  vec![0; max_size as usize];
            simulator.get_reader_mut().read_exact(&mut s).unwrap();
            let s = String::from_utf8(s).unwrap();
            for c in s.chars() {
                simulator.get_memory_mut().write(address as usize, &RawData::from_char(c))?;
                address += simulator.get_memory().word_size().value() as i64;
            }
            simulator.get_memory_mut().write(address as usize, &RawData::from_char('\0'))?;
            Ok(())
        });
    pub static ref READS_SIZED: Instruction =
        instruction!(reads, |simulator: Simulator,
                            input1: InputOutputTarget| {
            let mut address = input1.get(simulator)?.int_value();
            let mut s = String::new();
            simulator.get_reader_mut().read_to_string(&mut s).unwrap();
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
            todo!();
        });
    pub static ref READLN_SIZED: Instruction =
        instruction!(readln, |simulator: Simulator,
                            input1: InputOutputTarget| {
            todo!();
        });

}

pub fn register_instructions() {
    register_instruction(&READI);
    register_instruction(&READF);
    register_instruction(&READC);
    register_instruction(&READS);
    register_instruction(&READS_SIZED);
    register_instruction(&READLN);
    register_instruction(&READLN_SIZED);
}