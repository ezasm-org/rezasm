use rezasm_core::parser::lexer;
use rezasm_core::parser::line::Line;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::EzasmError;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

pub struct Application {
    simulator: Simulator,
    code_file: BufReader<File>,
    input_file: BufReader<File>,
    output_file: BufWriter<File>,
}

impl Application {
    pub fn new(
        simulator: Simulator,
        code_file: BufReader<File>,
        input_file: BufReader<File>,
        output_file: BufWriter<File>,
    ) -> Application {
        Application {
            simulator,
            code_file,
            input_file,
            output_file,
        }
    }

    pub fn run_until_completion(mut self) -> Result<(), EzasmError> {
        let lines = self.code_file
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
        for line in lines {
            match lexer::parse_line(&line, self.simulator.get_word_size()) {
                Some(line_result) => match line_result {
                    Ok(line) => self.simulator.add_line(line)?,
                    Err(error) => return Err(error),
                },
                None => {}
            };
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

pub fn handle_error(error: EzasmError) -> ! {
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
