use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::parse_line;
use crate::parser::line::Line;
use crate::util::error::{ParserError, SimulatorError};
use crate::util::io::RezasmFileReader;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

lazy_static! {
    pub static ref IMPORT: Instruction =
        instruction!(import, |simulator: Simulator, input: InputTarget| {
            let address = input.get(&simulator)?.int_value() as usize;
            let file_path = simulator.get_memory().get_string(address)?;
            let mut relative_location = simulator.get_program().main_file();
            let relative_location_split = relative_location =
                match relative_location.rsplit_once('/') {
                    Some((first, _)) => format!("{}/", first),
                    None => String::new(),
                };
            let file = RezasmFileReader::new(&format!("{}{}", relative_location, file_path))?;
            let line_results: Vec<Option<Result<Line, ParserError>>> = file
                .lines()?
                .iter()
                .map(|line| parse_line(line, simulator.get_word_size()))
                .collect();
            let mut lines = Vec::new();
            for line in line_results {
                match line {
                    Some(l) => lines.push(l?),
                    None => {}
                }
            }
            simulator.add_lines(lines, file_path)
        });
}

pub fn register_instructions() {
    register_instruction(&IMPORT);
}
