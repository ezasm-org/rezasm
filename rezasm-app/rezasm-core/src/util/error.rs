use std::num::{ParseFloatError, ParseIntError};
use std::process;

#[derive(Debug)]
pub enum EzasmError {
    ParserError,
    SimualtorError,
    InvalidArgumentsError,
    InvalidMemorySizeError(usize),
    InvalidInstructionError(String),
    InvalidRegisterNumberError(usize),
    InvalidRegisterNameError(String),
    InvalidWordSizeError(usize),
    CouldNotOpenFileError(String),
    PathIsNotFileError(String),
    FileDoesNotExistError(String),
    ReadOutOfBoundsError(usize),
    WriteOutOfBoundsError(usize),
    WriteToReadOnlyError(usize),
    InvalidProgramCounterError(i64),
    NonExistentLabelError(String),
    LabelInUseError(String),
    TimeoutError(),
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
        EzasmError::InvalidMemorySizeError(_) => {}
        EzasmError::InvalidInstructionError(_) => {}
        EzasmError::InvalidRegisterNumberError(_) => {}
        EzasmError::InvalidRegisterNameError(_) => {}
        EzasmError::InvalidWordSizeError(_) => {}
        EzasmError::CouldNotOpenFileError(_) => {}
        EzasmError::PathIsNotFileError(_) => {}
        EzasmError::FileDoesNotExistError(_) => {}
        EzasmError::ReadOutOfBoundsError(_) => {}
        EzasmError::WriteOutOfBoundsError(_) => {}
        EzasmError::WriteToReadOnlyError(_) => {}
        EzasmError::InvalidProgramCounterError(_) => {}
        EzasmError::NonExistentLabelError(_) => {}
        EzasmError::LabelInUseError(_) => {}
        EzasmError::TimeoutError() => {}
    }
    process::exit(1);
}
