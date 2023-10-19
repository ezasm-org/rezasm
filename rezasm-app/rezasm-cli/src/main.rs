#![allow(dead_code)]
#![allow(unused_variables)]

mod util;

extern crate lazy_static;
extern crate rezasm_core;

use std::io::Write;

use rezasm_core::instructions::implementation::register_instructions;
use rezasm_core::parser::lexer::{parse_line, text_to_number, tokenize_line, EZNumber};
use rezasm_core::parser::line::Line;
use rezasm_core::simulation::memory::Memory;
use rezasm_core::simulation::registry;
use rezasm_core::simulation::registry::Registry;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::handle_error;
use rezasm_core::util::raw_data::RawData;
use rezasm_core::util::word_size::DEFAULT_WORD_SIZE;

use crate::util::application::Application;
use crate::util::cli;
use crate::util::cli::Arguments;
use crate::util::cli_arguments::handle_arguments;

fn main() {
    register_instructions();
    test_tokenize_line();
    test_text_to_number();
    test_memory();
    test_registry();
    test_macro();
    test_simulator_instruction();
    test_simulator_labels();
    test_io();

    let args: Arguments = cli::get_args();
    let application: Application = match handle_arguments(args) {
        Ok(app) => app,
        Err(error) => handle_error(error),
    };

    match application.run_until_completion() {
        Ok(_) => {}
        Err(error) => handle_error(error.into()),
    };
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

pub fn test_macro() {
    let mut simulator: Simulator = Simulator::new();

    let line: Line = Line::new(
        &String::from("add"),
        ["$T0".to_string(), "$T0".to_string(), "121".to_string()].to_vec(),
        simulator.get_word_size(),
    )
    .unwrap();

    let args = match line {
        Line::Instruction(_, args) => args,
        _ => Vec::new(),
    };

    let foo = &rezasm_core::instructions::implementation::arithmetic_instructions::ADD;

    match foo.call_function(&mut simulator, &args) {
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

    let line = parse_line(&"add $t0 $t0 1".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let _ = simulator.add_line(line);
    let _ = simulator.run_line_from_pc();

    assert_eq!(
        simulator
            .get_registers()
            .get_register(&registry::T0.to_string())
            .unwrap()
            .get_data()
            .int_value(),
        1i64
    );

    println!("Instruction Registry and Simulator work");
}

pub fn test_simulator_labels() {
    let mut simulator: Simulator = Simulator::new();

    let line1 = parse_line(&"add $t0 0 0".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let line2 = parse_line(&"add $t1 0 1".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let line3 = parse_line(&"fib:".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let line4 = parse_line(&"add $t2 $t0 $t1".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let line5 = parse_line(&"add $t0 0 $t1".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let line6 = parse_line(&"add $t1 0 $t2".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let line7 = parse_line(&"add $pc 0 fib".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();

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

    assert_eq!(
        simulator
            .get_registers()
            .get_register(&registry::T1.to_string())
            .unwrap()
            .get_data()
            .int_value(),
        233i64
    );
    println!("Labels worked (fibonacci test)")
}

pub fn test_io() {
    use rezasm_core::util::io::RezasmFileReader;
    use std::fs;
    use std::path::PathBuf;

    // Read into rezasm file
    let file_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/example/arithmatic_fib.ez"
    ));
    let mut rezasmfile = RezasmFileReader::new(file_path.clone()).expect("failed to read file");

    // Reads with fs
    let reg_bytes = fs::read(file_path).unwrap();
    let reg_read: Vec<String> = String::from_utf8(reg_bytes.clone())
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    // Compare
    assert_eq!(rezasmfile.lines().expect("failed to get lines"), reg_read);

    assert_eq!(32, rezasmfile.seek_absolute_byte(1).unwrap());
    assert_eq!(101, rezasmfile.seek_absolute_byte(3).unwrap());
    assert_eq!(32, rezasmfile.seek_relative_byte(-2).unwrap());
    assert_eq!(35, rezasmfile.peek_absolute_byte(0).unwrap());

    rezasmfile.seek_start();

    let mut bytes = vec![];
    while let Some(byte) = rezasmfile.next() {
        bytes.push(byte);
    }

    // Compare the bytes.
    assert_eq!(reg_bytes, bytes);

    // Cursor should now be at the end. Attempt to peek 0.
    assert_eq!(35, rezasmfile.peek_absolute_byte(0).unwrap());

    // Check validity (should be invalid).
    assert_eq!(false, rezasmfile.is_cursor_valid());

    // Convert to a writer.
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/example/arith_modified.ez");
    let mut writer = rezasmfile.into_writer(Some(path)).unwrap();
    let write_str = "\n# This comment was generated by io test!";
    writer.extend_from_slice(write_str.as_bytes());
    writer.flush().unwrap();

    println!("Io worked");
}
