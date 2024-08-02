#![allow(dead_code)]
#![allow(unused_variables)]

extern crate scanner_rust;
extern crate thiserror;

#[macro_use]
pub mod instructions;

pub mod parser;

pub mod simulation;

pub mod util;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use super::*;
    use crate::instructions::implementation::register_instructions;
    use crate::simulation::reader::DummyReader;
    use crate::simulation::reader_cell::ReaderCell;
    use parser::lexer::parse_lines;
    use simulation::simulator::Simulator;
    use test_utils::workspace_root;
    use util::word_size::WordSize;

    #[test]
    fn test_fibo() {
        register_instructions();
        let word_size = WordSize::Eight;
        let file_name = "MainFile".to_string();

        let mut simulator = Simulator::new_custom(
            &word_size,
            1024,
            ReaderCell::new(DummyReader::new()),
            Box::new(io::stdout()),
        );

        let workspace = workspace_root()
            .to_str()
            .expect("workspace_root to string failed")
            .to_string();

        let path = format!("{workspace}/example/fibonacci_jump.ez");

        let code = fs::read_to_string(&path).expect(format!("File {}: read failed", path).as_str());
        let lines = parse_lines(&code, &word_size).expect("Lexing failed");
        lines.into_iter().for_each(|line| {
            simulator
                .add_line(line, file_name.clone())
                .expect("Failed to add line")
        });

        while !simulator.is_done() {
            simulator.run_line_from_pc().expect("Line panicked");
        }

        assert_eq!(
            simulator
                .get_registers_mut()
                .get_register("A1")
                .expect("Register access error")
                .get_data()
                .int_value(),
            89i64
        );
    }
}
