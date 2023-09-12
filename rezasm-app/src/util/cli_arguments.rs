use crate::util::application::Application;
use crate::util::cli::Arguments;
use rezasm_core::simulation::simulator::Simulator;
use rezasm_core::util::error::EzasmError;
use rezasm_core::util::word_size::WordSize;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::os::fd::{AsRawFd, FromRawFd};
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
                Err(_) => return Err(EzasmError::CouldNotOpenFileError(path_string.to_string())),
            }
        } else {
            return Err(EzasmError::PathIsNotFileError(path_string.to_string()));
        }
    } else {
        return Err(EzasmError::FileDoesNotExistError(path_string.to_string()));
    };
    Ok(file)
}

pub fn handle_arguments(arguments: Arguments) -> Result<Application, EzasmError> {
    let word_size = match &arguments.get_word_size() {
        4 => WordSize::Four,
        8 => WordSize::Eight,
        _ => return Err(EzasmError::InvalidWordSizeError(arguments.get_word_size())),
    };

    let memory_size = match arguments.get_memory_size() {
        0 => return Err(EzasmError::InvalidMemorySizeError(0)),
        x => x,
    };

    let simulator: Simulator = Simulator::new_custom(&word_size, memory_size);

    let code_file: BufReader<File> =
        BufReader::new(get_file_from_path(&arguments.get_code_file())?);

    let input_file: BufReader<File> = match arguments.get_input_file() {
        Some(input_file_string) => BufReader::new(get_file_from_path(&input_file_string)?),
        None => {
            let fd = io::stdin().as_raw_fd();
            unsafe { BufReader::new(File::from_raw_fd(fd)) }
        }
    };

    let output_file: BufWriter<File> = match arguments.get_output_file() {
        Some(input_file_string) => BufWriter::new(get_file_from_path(&input_file_string)?),
        None => {
            let fd = io::stdin().as_raw_fd();
            unsafe { BufWriter::new(File::from_raw_fd(fd)) }
        }
    };

    Ok(Application::new(
        simulator,
        code_file,
        input_file,
        output_file,
    ))
}
