use crate::util::application::Application;
use crate::util::cli::Arguments;
use crate::util::cli_io::{InputSource, OutputSink};
use rezasm_core::simulation::reader_cell::ReaderCell;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::{EzasmError, IoError, SimulatorError};
use rezasm_core::util::io::{RezasmFileReader, RezasmFileWriter};
use rezasm_core::util::word_size::WordSize;
use std::fs::File;
use std::path::Path;

fn get_file_from_path(path_string: &String) -> Result<File, EzasmError> {
    let path = if path_string.starts_with('~') {
        todo!();
    } else {
        Path::new(path_string).to_path_buf()
    };

    let file: File = if path.exists() {
        if path.is_file() {
            match File::open(path) {
                Ok(file) => file,
                Err(_) => return Err(IoError::CouldNotOpenFileError(path_string.to_string()))?,
            }
        } else {
            return Err(IoError::PathIsNotFileError(path_string.to_string()))?;
        }
    } else {
        return Err(IoError::FileDoesNotExistError(path_string.to_string()))?;
    };
    Ok(file)
}

pub fn handle_arguments(arguments: Arguments) -> Result<Application, EzasmError> {
    let word_size = match &arguments.get_word_size() {
        4 => WordSize::Four,
        8 => WordSize::Eight,
        _ => {
            return Err(SimulatorError::InvalidWordSizeError(
                arguments.get_word_size(),
            ))?
        }
    };

    let memory_size = match arguments.get_memory_size() {
        0 => return Err(SimulatorError::InvalidMemorySizeError(0))?,
        x => x,
    };

    let code_file = RezasmFileReader::new(arguments.get_code_file())?;

    let input_file: InputSource = match arguments.get_input_file() {
        Some(input_file_string) => InputSource::new_file(RezasmFileReader::new(input_file_string)?),
        None => InputSource::new_console(),
    };

    let output_file: OutputSink = match arguments.get_output_file() {
        Some(output_file_string) => {
            OutputSink::new_file(RezasmFileWriter::new(output_file_string)?)
        }
        None => OutputSink::new_console(),
    };

    let simulator: Simulator = Simulator::new_custom(
        &word_size,
        memory_size,
        ReaderCell::new(input_file),
        Box::new(output_file),
    );

    Ok(Application::new(simulator, code_file))
}
