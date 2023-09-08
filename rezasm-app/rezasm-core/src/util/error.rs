use std::num::{ParseFloatError, ParseIntError};
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EzasmError {
    #[error("error parsing the given code")]
    ParserError(ParserError),

    #[error("error running the given code")]
    SimulatorError,

    #[error("arguments do not match instructions")]
    InvalidArgumentsError,

    #[error("invalid given memory size `{0}`")]
    InvalidMemorySizeError(usize),

    #[error("invalid given instruction `{0}`")]
    InvalidInstructionError(String),

    #[error("invalid register number `{0}`")]
    InvalidRegisterNumberError(usize),

    #[error("invalid register name `{0}`")]
    InvalidRegisterNameError(String),

    #[error("invalid word size `{0}`")]
    InvalidWordSizeError(usize),

    #[error("could not open file `{0}`")]
    CouldNotOpenFileError(String),

    #[error("path `{0}` is not a file")]
    PathIsNotFileError(String),

    #[error("file `{0}` does not exist")]
    FileDoesNotExistError(String),

    #[error("attempted read to address `{0}` outside of memory")]
    ReadOutOfBoundsError(usize),

    #[error("attempted read to address `{0}` which is negative")]
    ReadNegativeAddressError(i64),

    #[error("attempted write to address `{0}` outside of memory")]
    WriteOutOfBoundsError(usize),

    #[error("attempted write to address `{0}` which is negative")]
    WriteNegativeAddressError(i64),

    #[error("attempted write to address `{0}` in read-only memory")]
    WriteToReadOnlyError(usize),

    #[error("invalid program counter `{0}`")]
    InvalidProgramCounterError(i64),

    #[error("label `{0}` does not exist")]
    NonExistentLabelError(String),

    #[error("label `{0}` is already in use")]
    LabelInUseError(String),

    #[error("action timed out")]
    TimeoutError(),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("invalid character immediate `{0}`")]
    CharacterImmediateError(String),

    #[error("invalid numeric immediate `{0}`")]
    NumericImmediateError(String),

    #[error("invalid string immediate `{0}`")]
    StringImmediateError(String),

    #[error("invalid dereference `{0}`")]
    DereferenceError(String),

    #[error("unknown register `{0}`")]
    UnknownRegisterError(String),

    #[error("unknown token `{0}`")]
    UnknownTokenError(String),
}

impl From<ParserError> for EzasmError {
    fn from(error: ParserError) -> Self {
        EzasmError::ParserError(error)
    }
}

impl From<ParseFloatError> for ParserError {
    fn from(error: ParseFloatError) -> Self {
        ParserError::NumericImmediateError(error.to_string())
    }
}

impl From<ParseIntError> for ParserError {
    fn from(error: ParseIntError) -> Self {
        ParserError::NumericImmediateError(error.to_string())
    }
}

pub fn handle_error(error: EzasmError) -> ! {
    println!("{}", error);
    match error {
        EzasmError::ParserError(_) => {}
        EzasmError::SimulatorError => {}
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
        EzasmError::ReadNegativeAddressError(_) => {}
        EzasmError::WriteOutOfBoundsError(_) => {}
        EzasmError::WriteNegativeAddressError(_) => {}
        EzasmError::WriteToReadOnlyError(_) => {}
        EzasmError::InvalidProgramCounterError(_) => {}
        EzasmError::NonExistentLabelError(_) => {}
        EzasmError::LabelInUseError(_) => {}
        EzasmError::TimeoutError() => {}
    }
    process::exit(1);
}
