mod common;

use crate::common::writer::TestWriter;
use common::reader::TestReader;
use rezasm_core::instructions::implementation::arithmetic_instructions::ADD;
use rezasm_core::instructions::implementation::register_instructions;
use rezasm_core::parser::lexer::{
    parse_line, parse_lines, text_to_number, tokenize_line, EZNumber,
};
use rezasm_core::parser::line::Line;
use rezasm_core::simulation::memory::Memory;
use rezasm_core::simulation::reader::DummyReader;
use rezasm_core::simulation::reader_cell::ReaderCell;
use rezasm_core::simulation::registry::Registry;
use rezasm_core::simulation::registry::{self, get_register_number};
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::simulation::writer::DummyWriter;
use rezasm_core::util::error::ParserError;
use rezasm_core::util::io::RezasmFileReader;
use rezasm_core::util::raw_data::RawData;
use rezasm_core::util::word_size::DEFAULT_WORD_SIZE;
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn test_tokenize_line() {
    assert_eq!(
        std::format!(
            "{:?}",
            tokenize_line(&String::from("add $t0 1 2 # this - is = a # comment"))
        ),
        "[\"add\", \"$t0\", \"1\", \"2\"]"
    );
}

#[test]
fn test_text_to_number() {
    assert_eq!(
        match text_to_number(String::from("0x0010.8000")).unwrap() {
            EZNumber::Integer(_) => f64::INFINITY,
            EZNumber::Float(x) => x,
        },
        16.5
    );
}

#[test]
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
}

#[test]
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

    let k = registry.get_register_by_number_mut(get_register_number(&"$0".to_string()).unwrap());

    assert!(k.is_err_and(|e| e.to_string() == ParserError::ImmutableZeroRegisterError.to_string()));
}

#[test]
pub fn test_macro() {
    register_instructions();
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

    let foo = &ADD;

    match foo.call_function(&mut simulator, &args) {
        Ok(seq) => {
            seq.apply(&mut simulator).unwrap();
            assert_eq!(
                simulator
                    .get_registers()
                    .get_register(&registry::T0.to_string())
                    .unwrap()
                    .get_data()
                    .int_value(),
                121
            );
        }
        Err(_) => {
            assert!(false);
        }
    }
}

#[test]
pub fn test_simulator_instruction() {
    register_instructions();
    let mut simulator: Simulator = Simulator::new();

    let line = parse_line(&"add $t0 $t0 1".to_string(), simulator.get_word_size())
        .unwrap()
        .unwrap();
    let _ = simulator.add_line(line, "".to_string());
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
}

#[test]
pub fn test_print_instructions() {
    register_instructions();
    let writer = Box::new(TestWriter::new());
    let reader = ReaderCell::new(DummyReader::new());
    let mut simulator: Simulator = Simulator::new_custom_reader_writer(reader, writer);
    let program = "
        move $s2 \"Print Instructions Work!\\n\"
        add $s1 '\\n' 0
        add $t0 1 2
        printi $t0
        printc $s1
        addf $t1 1.5 0
        printf $t1
        printc $s1
        prints $s2";
    let lines = parse_lines(&program.to_string(), simulator.get_word_size()).unwrap();
    simulator.add_lines(lines, "".to_string()).unwrap();
    while !simulator.is_done() {
        simulator.run_line_from_pc().unwrap();
    }
    let writer = simulator.get_writer();
    let output = writer
        .deref()
        .as_any()
        .downcast_ref::<TestWriter>()
        .unwrap()
        .get_data();
    assert_eq!(output.as_str(), "3\n1.5\nPrint Instructions Work!\n");
}

#[test]
pub fn test_read_instructions() { // Test assumes all other instructions work properly
    register_instructions();
    let buffer = "10\n10.5\na\nHello"; //doesn't cover everything, should be close enough
    let program = "
        readi $t0
        readf $t1
        alloc $s0 $t0
        readc $t2
        reads $s0
        printi $t0
        printc '\\n'
        printf $t1
        printc '\\n'
        printc $t2
        printc '\\n'
        prints $s0".to_string();

    let mut reader = ReaderCell::new(TestReader::new());
    let _ = reader.write(buffer.as_bytes()).unwrap();
    let writer = Box::new(TestWriter::new());
    let mut simulator: Simulator = Simulator::new_custom_reader_writer(reader, writer);
    let lines = parse_lines(&program, simulator.get_word_size()).unwrap();
    simulator.add_lines(lines, "".to_string()).unwrap();
    while !simulator.is_done() {
        simulator.run_line_from_pc().unwrap();
    }
    let writer = simulator.get_writer();
    let output = writer
        .deref()
        .as_any()
        .downcast_ref::<TestWriter>()
        .unwrap()
        .get_data();
    assert_eq!(output.as_str(), "10\n10.5\na\nHello");
}

#[test]
pub fn test_simulator_labels() {
    register_instructions();
    let mut simulator: Simulator = Simulator::new();
    let program = "
        add $t0 0 0
        add $t1 0 1
        fib:
        add $t2 $t0 $t1
        add $t0 0 $t1
        add $t1 0 $t2
        add $pc 0 fib";
    let lines = parse_lines(&program.to_string(), simulator.get_word_size()).unwrap();
    match simulator.add_lines(lines, "".to_string()) {
        Ok(_) => {}
        Err(_) => assert!(false),
    }

    for _ in 0..50 {
        match simulator.run_line_from_pc() {
            Ok(_) => {}
            Err(_) => assert!(false),
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
}

#[test]
pub fn test_io() {
    register_instructions();

    // Read into rezasm file
    let file_path = workspace_root().join("example").join("arithmetic_fib.ez");
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
    let file_path = workspace_root().join("example").join("arith_modified.ez");
    let path: String = file_path.to_string_lossy().into();
    let mut writer = rezasmfile.into_writer(Some(path)).unwrap();
    let write_str = "\n# This comment was generated by io tests!";
    writer.extend_from_slice(write_str.as_bytes());
    writer.flush().unwrap();
}
