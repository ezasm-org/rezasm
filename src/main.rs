#![allow(dead_code)]
#![allow(unused_variables)]

mod instructions;
mod util;

extern crate ezasm_core;
extern crate ezasm_macro;
extern crate lazy_static;

use ezasm_core::instructions::argument_type::ArgumentType;
use ezasm_core::instructions::instruction_field::{Subclass, SubclassFactory};
use ezasm_core::instructions::targets::input_output_target::InputOutputTarget;
use ezasm_core::instructions::targets::input_target::InputTarget;
use ezasm_core::parser::lexer::{text_to_number, tokenize_line, EZNumber, parse_line};
use ezasm_core::parser::line::Line;
use ezasm_core::simulation::memory::Memory;
use ezasm_core::simulation::registry;
use ezasm_core::simulation::registry::Registry;
use ezasm_core::simulation::simulator::Simulator;
use ezasm_core::util::raw_data::RawData;
use ezasm_core::util::word_size::DEFAULT_WORD_SIZE;
use ezasm_macro::instruction;

use crate::instructions::implementation::arithmetic_instructions::register_instructions;
use crate::util::cli;

fn main() {
    let args = cli::get_args();

    register_instructions();
    test_tokenize_line();
    test_text_to_number();
    test_memory();
    test_registry();
    test_subclasses();
    test_proc_macro();
    test_simulator_instruction();
    test_simulator_labels();
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
    assert_eq!(
        match text_to_number(String::from("0x0010.8000")).unwrap() {
            EZNumber::Integer(_) => f64::INFINITY,
            EZNumber::Float(x) => x,
        },
        16.5
    );
    println!("Hex-Float-String -> Float-Value works");
}

fn test_memory() {
    let mut memory: Memory = Memory::new();
    let data = RawData::from_int(100, &DEFAULT_WORD_SIZE);
    memory.write(memory.current_heap_pointer(), &data).unwrap();
    assert_eq!(
        memory
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
    assert_eq!(
        registry
            .get_register(&String::from(registry::T0))
            .unwrap()
            .get_data()
            .int_value(),
        255
    );
    println!("Registry works");
}

pub fn test_subclasses() {
    let input_subclasses = SubclassFactory::<InputTarget>::subclasses();
    let input_output_subclasses = SubclassFactory::<InputOutputTarget>::subclasses();
    let input_output_typeid = input_output_subclasses.get(0).unwrap();
    assert!(input_subclasses.contains(input_output_typeid));
    println!("Subclasses work")
}

pub fn test_proc_macro() {
    let mut simulator: Simulator = Simulator::new();

    let line: Line = Line::new(
        &String::from("add"),
        ["$T0".to_string(), "$T0".to_string(), "121".to_string()].to_vec(),
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

    let foo = instruction!(foo, |simulator: Simulator,
                                 x: InputOutputTarget,
                                 y: InputTarget,
                                 z: InputTarget| {
        let sum = y.get(simulator)?.int_value() + z.get(simulator)?.int_value();
        let _ = x.set(simulator, RawData::from(sum));
        Ok(())
    });

    match foo.call_instruction_function(&mut simulator, &targets) {
        Ok(_) => {
            assert_eq!(
                simulator
                    .get_registers()
                    .get_register(&registry::T0.to_string())
                    .unwrap()
                    .get_data()
                    .int_value(),
                121
            );
            println!("Instruction Fields work")
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

pub fn test_simulator_instruction() {
    let mut simulator: Simulator = Simulator::new();

    let line = parse_line(&"add $t0 $t0 1".to_string(), 0).unwrap().unwrap();
    let _ = simulator.add_line(line);
    let _ = simulator.run_line_from_pc();

    assert_eq!(simulator.get_registers().get_register(&registry::T0.to_string()).unwrap().get_data().int_value(), 1i64);

    println!("Instruction Registry and Simulator work");
}

pub fn test_simulator_labels() {
    let mut simulator: Simulator = Simulator::new();

    let line1 = parse_line(&"add $t0 0 0".to_string(), 0).unwrap().unwrap();
    let line2 = parse_line(&"add $t1 0 1".to_string(), 1).unwrap().unwrap();
    let line3 = parse_line(&"fib:".to_string(), 2).unwrap().unwrap();
    let line4 = parse_line(&"add $t2 $t0 $t1".to_string(), 3).unwrap().unwrap();
    let line5 = parse_line(&"add $t0 0 $t1".to_string(), 4).unwrap().unwrap();
    let line6 = parse_line(&"add $t1 0 $t2".to_string(), 5).unwrap().unwrap();
    let line7 = parse_line(&"add $pc 0 fib".to_string(), 6).unwrap().unwrap();

    match simulator.add_lines(vec![line1, line2, line3, line4, line5, line6, line7]) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }

    for i in 0..50 {
        match simulator.run_line_from_pc() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
    }

    assert_eq!(simulator.get_registers().get_register(&registry::T1.to_string()).unwrap().get_data().int_value(), 233i64);
    println!("Labels worked (fibonacci test)")
}
