#![allow(dead_code)]
#![allow(unused_variables)]

extern crate lazy_static;

use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::{test_instruction, Instruction};
use crate::instructions::instruction_field::{Subclass, SubclassFactory};
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::{EZNumber, text_to_number, tokenize_line};
use crate::parser::line::Line;
use crate::simulation::memory::Memory;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::simulation::simulator::Simulator;
use crate::util::raw_data::RawData;
use crate::util::word_size::DEFAULT_WORD_SIZE;

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
    test_subclasses();
}

fn test_tokenize_line() {
    assert_eq!(
        std::format!(
            "{:?}",
            tokenize_line(&String::from("add $t0 1 2 # this - is = a # comment"))
        ),
        "[\"add\", \"$t0\", \"1\", \"2\"]"
    );
    println!("Line tokenize works");
}

fn test_text_to_number() {
    assert_eq!(match text_to_number(String::from("0x0010.8000")).unwrap() {
        EZNumber::Integer(_) => f64::INFINITY,
        EZNumber::Float(x) => x,
    }, 16.5);
    println!("Hex-Float-String -> Float-Value works");
}

fn test_memory() {
    let mut memory: Memory = Memory::new();
    let data = RawData::from_int(100, &DEFAULT_WORD_SIZE);
    memory.write(memory.current_heap_pointer(), &data).unwrap();
    assert_eq!(memory
        .read(memory.current_heap_pointer())
        .unwrap()
        .int_value(),
        100
    );
    println!("Memory works");
}

fn test_registry() {
    let mut registry: Registry = Registry::new(&DEFAULT_WORD_SIZE);
    registry
        .get_register_mut(&String::from(registry::T0))
        .unwrap()
        .set_data(RawData::from(255i32));
    assert_eq!(registry
       .get_register(&String::from(registry::T0))
       .unwrap()
       .get_data()
       .int_value(), 255);
    println!("Registry works");
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
        Ok(_) => println!("Instruction macros work"),
        Err(x) => println!("{:?}", x),
    };
}

pub fn test_subclasses() {
    let input_subclasses = SubclassFactory::<InputTarget>::subclasses();
    let input_output_subclasses = SubclassFactory::<InputOutputTarget>::subclasses();
    let input_output_typeid = input_output_subclasses.get(0).unwrap();
    assert!(input_subclasses.contains(input_output_typeid));
    println!("Subclasses work")
}
