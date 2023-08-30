use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum EzasmError {
    ParserError,
    SimualtorError,
    InvalidArguments,
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
