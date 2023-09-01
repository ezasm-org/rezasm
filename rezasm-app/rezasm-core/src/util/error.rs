use std::num::{ParseFloatError, ParseIntError};
use std::process;

#[derive(Debug)]
pub enum EzasmError {
    ParserError,
    SimualtorError,
    InvalidArgumentsError,
    InvalidWordSizeError(usize),
    InvalidMemorySizeError(usize),
    InvalidInstructionError(String),
    CouldNotOpenFileError(String),
    PathIsNotFileError(String),
    FileDoesNotExistError(String),
    ReadOutOfBoundsError(usize),
    WriteOutOfBoundsError(usize),
    WriteToReadOnlyError(usize),
    InvalidProgramCounterError(i64),
    NonExistentLabelError(String),
    LabelInUseError(String),
}

impl From<ParseFloatError> for EzasmError {
    fn from(_err: ParseFloatError) -> Self {
        EzasmError::ParserError
    }
}

impl From<ParseIntError> for EzasmError {
    fn from(_err: ParseIntError) -> Self {
        EzasmError::ParserError
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
    process::exit(1);
}