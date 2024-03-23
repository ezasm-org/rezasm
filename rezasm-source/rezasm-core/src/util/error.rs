use scanner_rust::ScannerError;
use std::char::ParseCharError;
use std::num::{ParseFloatError, ParseIntError};
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EzasmError {
    #[error("{0}")]
    ParserError(#[from] ParserError),

    #[error("{0}")]
    SimulatorError(#[from] SimulatorError),

    #[error("internal error: {0}")]
    InternalError(#[from] InternalError),

    #[error("{0}")]
    IoError(#[from] IoError),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("internal error: {0}")]
    InternalError(#[from] InternalError),

    #[error("invalid given instruction `{0}`")]
    InvalidInstructionError(String),

    #[error("instruction `{0}` cannot accept argument `{1}` at index {2}")]
    InvalidArgumentsError(String, String, usize),

    #[error("instruction `{0}` does not accept {1} arguments")]
    InvalidArgumentsCountError(String, usize),

    #[error("invalid register number `{0}`")]
    InvalidRegisterNumberError(usize),

    #[error("invalid register name `{0}`")]
    InvalidRegisterNameError(String),

    #[error("invalid character immediate `{0}`")]
    CharacterImmediateError(String),

    #[error("invalid dereference `{0}`")]
    DereferenceError(String),

    #[error("invalid label definition `{0}`")]
    LabelDefinitionError(String),

    #[error("invalid numeric immediate `{0}`")]
    NumericalImmediateError(String),

    #[error("invalid string immediate `{0}`")]
    StringImmediateError(String),

    #[error("unrecognized register `{0}`")]
    UnknownRegisterError(String),

    #[error("unrecognized token `{0}`")]
    UnknownTokenError(String),

    #[error("the zero register is not mutable")]
    ImmutableZeroRegisterError,
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("improper usage of try_into")]
    MismatchedTryIntoError,

    #[error("improper usage of get_input_output_target")]
    GetInputOutputTargetError,

    #[error("null op on transformation sequence")]
    NullOpError,
}

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("{0}")]
    ParserError(#[from] ParserError),

    #[error("internal error: {0}")]
    InternalError(#[from] InternalError),

    #[error("{0}")]
    IoError(#[from] IoError),

    #[error("attempted read to address `{0}` which is negative")]
    ReadNegativeAddressError(i64),

    #[error("attempted read to address `{0}` outside of memory")]
    ReadOutOfBoundsError(usize),

    #[error("attempted write to address `{0}` which is negative")]
    WriteNegativeAddressError(i64),

    #[error("attempted write to address `{0}` outside of memory")]
    WriteOutOfBoundsError(usize),

    #[error("attempted write to address `{0}` in read-only memory")]
    WriteToReadOnlyError(usize),

    #[error("invalid given memory size `{0}`")]
    InvalidMemorySizeError(usize),

    #[error("invalid word size `{0}`")]
    InvalidWordSizeError(usize),

    #[error("invalid heap pointer `{0}`")]
    InvalidHeapPointerError(usize),

    #[error("invalid program counter `{0}`")]
    InvalidProgramCounterError(i64),

    #[error("invalid line number `{0}`")]
    InvalidLineNumber(i64),

    #[error("invalid file identifier `{0}`")]
    InvalidFileIdentifier(i64),

    #[error("string immediate `{0}` does not exist")]
    StringImmediateDoesNotExistError(String),

    #[error("string immediate `{0}` could not be allocated because there is not enough memory in the string region")]
    StringRegionOutOfMemoryError(String),

    #[error("label `{0}` does not exist")]
    NonExistentLabelError(String),

    #[error("label `{0}` is already in use")]
    LabelInUseError(String),

    #[error("attempted to divide by zero")]
    DivideByZeroError,

    #[error("attempted to convert NaN value to an integer")]
    NaNConversionError,

    #[error("could not read type {0}")]
    ReadError(String),
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("{0}")]
    StdIoError(#[from] std::io::Error),

    #[error("{0}")]
    ScannerError(#[from] scanner_rust::ScannerError),

    #[error("could not open file `{0}`")]
    CouldNotOpenFileError(String),

    #[error("path `{0}` is not a file")]
    PathIsNotFileError(String),

    #[error("file `{0}` does not exist")]
    FileDoesNotExistError(String),

    #[error("attempted to seek out of bounds in file")]
    OutOfBoundsError,

    #[error("some bytes are not UTF-8 in the input file")]
    UnsupportedEncodingError,

    #[error("read operation failed")]
    ReadError,

    #[error("write operation failed")]
    WriteError,

    #[error("the given directory doesn't exist")]
    DirectoryError,
}

impl From<ParseFloatError> for ParserError {
    fn from(error: ParseFloatError) -> Self {
        ParserError::NumericalImmediateError(error.to_string())
    }
}

impl From<ParseIntError> for ParserError {
    fn from(error: ParseIntError) -> Self {
        ParserError::NumericalImmediateError(error.to_string())
    }
}

impl From<ParseCharError> for ParserError {
    fn from(error: ParseCharError) -> Self {
        ParserError::StringImmediateError(error.to_string())
    }
}

impl From<ScannerError> for SimulatorError {
    fn from(err: ScannerError) -> Self {
        SimulatorError::IoError(IoError::ScannerError(err))
    }
}

pub fn handle_error(error: EzasmError) -> ! {
    println!("{}", error);
    process::exit(1);
}
