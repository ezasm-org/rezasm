#![allow(dead_code)]
#![allow(unused_variables)]

extern crate lazy_static;

extern crate ezasm_core;
extern crate ezasm_macro;

use ezasm_core::instructions::argument_type::ArgumentType;
use ezasm_core::instructions::instruction_field::{InstructionField, Subclass, SubclassFactory};
use ezasm_core::instructions::targets::input_output_target::InputOutputTarget;
use ezasm_core::instructions::targets::input_target::InputTarget;
use ezasm_core::instructions::implementation::arithmetic_instructions;
use ezasm_core::parser::lexer::{EZNumber, text_to_number, tokenize_line};
use ezasm_core::parser::line::Line;
use ezasm_core::simulation::memory::Memory;
use ezasm_core::simulation::registry;
use ezasm_core::simulation::registry::Registry;
use ezasm_core::simulation::simulator::Simulator;
use ezasm_core::util::raw_data::RawData;
use ezasm_core::util::word_size::DEFAULT_WORD_SIZE;

fn main() {
    test_tokenize_line();
    test_text_to_number();
    test_memory();
    test_registry();
    test_subclasses();
    test_instruction_field_macro();
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

pub fn test_subclasses() {
    let input_subclasses = SubclassFactory::<InputTarget>::subclasses();
    let input_output_subclasses = SubclassFactory::<InputOutputTarget>::subclasses();
    let input_output_typeid = input_output_subclasses.get(0).unwrap();
    assert!(input_subclasses.contains(input_output_typeid));
    println!("Subclasses work")
}

pub fn test_instruction_field_macro() {
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

    let targets: Vec<ArgumentType> = args
        .iter()
        .map(|k| simulator.get_target(k).unwrap())
        .collect();

    match arithmetic_instructions::add.call_instruction_function(&mut simulator, &targets) {
        Ok(_) => {
            assert_eq!(simulator.get_registers().get_register(&registry::T0.to_string()).unwrap().get_data().int_value(), 121);
            println!("Instruction Fields work")
        },
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}
