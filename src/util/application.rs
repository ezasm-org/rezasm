use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use rezasm_core::parser::lexer;
use rezasm_core::parser::line::Line;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::EzasmError;

pub struct Application {
    simulator: Simulator,
    code_file: BufReader<File>,
    input_file: BufReader<File>,
    output_file: BufWriter<File>,
}

impl Application {
    pub fn new(simulator: Simulator, code_file: BufReader<File>, input_file: BufReader<File>, output_file: BufWriter<File>) -> Application {
        Application {
            simulator,
            code_file,
            input_file,
            output_file
        }
    }

    pub fn run_until_completion(mut self) -> Result<(), EzasmError> {
        let mut buffer_string = String::new();
        let mut count: i64 = 0;
        while let Ok(bytes_read) = self.code_file.read_line(&mut buffer_string) {
            if bytes_read == 0 {
                break;
            }

            // Remove newline character
            let _ = buffer_string.pop();

            match lexer::parse_line(&buffer_string, count) {
                Some(line_result) => match line_result {
                    Ok(line) => {
                        self.simulator.add_line(line)?;
                        count += 1;
                    },
                    Err(error) => return Err(error),
                }
                None => {}
            };


            buffer_string.clear();
        }

        while !self.simulator.is_done() {
            self.simulator.run_line_from_pc()?
        }

        Ok(())
    }

    fn run_one_line(&mut self, line: Line) -> Result<(), EzasmError> {
        self.simulator.add_line(line)?;
        self.simulator.run_line_from_pc()
    }

}

pub fn handle_error(error: EzasmError) ->! {
    println!("{:?}", error);
    match error {
        EzasmError::ParserError => {}
        EzasmError::SimualtorError => {}
        EzasmError::InvalidArgumentsError => {}
        EzasmError::InvalidWordSizeError(_) => {}
        EzasmError::InvalidMemorySizeError(_) => {}
        EzasmError::InvalidInstructionError(_) => {}
        EzasmError::CouldNotOpenFileError(_) => {}
        EzasmError::PathIsNotFileError(_) => {}
        EzasmError::FileDoesNotExistError(_) => {}
        EzasmError::ReadOutOfBoundsError(_) => {}
        EzasmError::WriteOutOfBoundsError(_) => {}
        EzasmError::WriteToReadOnlyError(_) => {}
        EzasmError::InvalidProgramCounterError(_) => {}
        EzasmError::NonExistentLabelError(_) => {}
        EzasmError::LabelInUseError(_) => {}
    }
    panic!();
}
