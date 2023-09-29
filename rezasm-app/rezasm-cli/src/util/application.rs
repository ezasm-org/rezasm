use rezasm_core::parser::lexer;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::SimulatorError;
use rezasm_core::util::io::RezAsmFile;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

pub struct Application {
    simulator: Simulator,
    code_file: RezAsmFile,
    input_file: BufReader<File>,
    output_file: BufWriter<File>,
}

impl Application {
    pub fn new(
        simulator: Simulator,
        code_file: RezAsmFile,
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

    pub fn run_until_completion(mut self) -> Result<(), SimulatorError> {
        let lines = self
            .code_file
            .lines().map_err(SimulatorError::from)?;
        for line in lines {
            match lexer::parse_line(&line, self.simulator.get_word_size()) {
                Some(line_result) => match line_result {
                    Ok(line) => self.simulator.add_line(line)?,
                    Err(error) => return Err(error.into()),
                },
                None => {}
            };
        }

        while !self.simulator.is_done() {
            self.simulator.run_line_from_pc()?
        }

        Ok(())
    }
}
