#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate lazy_static;

use crate::error::EzasmError;
use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::{test_instruction, Instruction, TInstructionFunction};
use crate::instructions::targets::input_output_target::{InputOutput, InputOutputTarget};
use crate::instructions::targets::input_target::{Input, InputTarget};
use crate::instructions::targets::output_target::Output;
use crate::instructions::targets::*;
use crate::parser::lexer::{text_to_number, tokenize_line};
use crate::parser::line::Line;
use crate::simulation::memory;
use crate::simulation::memory::Memory;
use crate::simulation::register::Register;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::simulation::simulator::Simulator;
use crate::util::raw_data::RawData;
use crate::util::word_size::{WordSize, DEFAULT_WORD_SIZE};
use std::any::TypeId;

mod error;
mod instructions;
mod parser;
mod simulation;
mod util;

fn main() {
    test_tokenize_line();
    test_text_to_number();
    test_memory();
    test_registry();
    test_instruction_macro();
}

fn test_tokenize_line() {
    println!(
        "{:?}",
        tokenize_line(&String::from("add $t0 1 2 # this - is = a # comment"))
    );
    // Should be ["add", "$t0", "1" "2"]
}

fn test_text_to_number() {
    println!("{:?}", text_to_number(String::from("0x0010.8000")).unwrap());
    // Should be Float(16.5)
}

fn test_memory() {
    let mut memory: Memory = Memory::new();
    let data = RawData::from_int(100, &DEFAULT_WORD_SIZE);
    memory.write(memory.current_heap_pointer(), &data).unwrap();
    println!(
        "{:?}",
        memory
            .read(memory.current_heap_pointer())
            .unwrap()
            .int_value()
    );
    // Should be 100
}

fn test_registry() {
    let mut registry: Registry = Registry::new(&DEFAULT_WORD_SIZE);
    registry
        .get_register_mut(&String::from(registry::T0))
        .unwrap()
        .set_data(RawData::from(255i32));
    println!(
        "{:?}",
        registry
            .get_register(&String::from(registry::T0))
            .unwrap()
            .get_data()
            .int_value()
    );
    // Should be 255
}

fn test_instruction_macro() {
    let mut simulator: Simulator = Simulator::new();

    let line: Line = Line::new(
        &String::from("add"),
        ["$T0".to_string(), "100".to_string(), "21".to_string()].to_vec(),
    )
    .unwrap();
    let args = match line {
        Line::Instruction(_, args) => args,
        _ => Vec::new(),
    };

    let mut targets: Vec<ArgumentType> = args
        .iter()
        .map(|k| simulator.get_target(k).unwrap())
        .collect();
    let mut instruction: Instruction = test_instruction();
    let instruction_types = instruction.get_types().clone();
    let instruction_function = instruction.get_function();
    match (*instruction_function)(&mut simulator, &instruction_types, &mut targets) {
        Ok(_) => println!("Success"),
        Err(x) => println!("{:?}", x),
    };
}
