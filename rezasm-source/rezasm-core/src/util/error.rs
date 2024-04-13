use std::num::{ParseFloatError, ParseIntError};
use std::process;
use thiserror::Error;

/// Represents all errors that can occur in Rezasm's business layer.
///
/// This includes errors from the scanner, parser, input-output, and "internal" errors. These
/// errors are generally caused by the user's input assembly code. They are not fatal, but
/// should be displayed to the user to allow them to correct their input.
#[derive(Error, Debug)]
pub enum EzasmError {
    /// Represents errors that occur during the parsing of EzASM instructions.
    ///
    /// Refer to the `ParserError` enum for more information.
    #[error(transparent)]
    ParserError(#[from] ParserError),

    /// Represents errors that occur during the simulator's execution.
    ///
    /// Refer to the `SimulatorError` enum for more information.
    #[error(transparent)]
    SimulatorError(#[from] SimulatorError),

    /// Represents errors that cannot be handled, but should not immediately panic.
    ///
    /// Refer to the `InternalError` enum for more information.
    #[error(transparent)]
    InternalError(#[from] InternalError),

    /// Represents the errors that can occur during various types of I/O operations.
    ///
    /// Refer to the `IoError` enum for more information.
    #[error(transparent)]
    IoError(#[from] IoError),
}

/// Represents errors that occur during the parsing of EzASM instructions.
#[derive(Error, Debug)]
pub enum ParserError {
    /// Represents errors that cannot be handled, but should not immediately panic.
    ///
    /// Refer to the `InternalError` enum for more information.
    #[error(transparent)]
    InternalError(#[from] InternalError),

    /// Error that occurs when an instruction name is invalid.
    ///
    /// This error occurs when the instruction name is not recognized by the parser. This can be
    /// caused by a typo in the instruction name, or by an instruction that does not exist.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing invalid instruction name.
    #[error("invalid given instruction `{0}`")]
    InvalidInstructionError(String),

    /// Error that occurs when an instruction cannot accept a provided argument.
    ///
    /// This error occurs when an instruction cannot accept the provided argument at the given
    /// index. This can be caused by an argument that is not valid for the instruction, or by an
    /// argument that is not formatted correctly.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the instruction name.
    /// * `1` - `String` containing the invalid argument.
    /// * `2` - `usize` containing the index of the invalid argument.
    #[error("instruction `{0}` cannot accept argument `{1}` at index {2}")]
    InvalidArgumentsError(String, String, usize),

    /// Error that occurs when an instruction can't accept the provided number of arguments.
    ///
    /// This error occurs when an instruction cannot accept as many or as few arguments as are
    /// provided to it. This can be caused by an incorrect number of arguments, or by an argument
    /// that is not formatted correctly (e.g. a multi-word string literal might not be in quotes).
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the instruction name.
    /// * `1` - `usize` containing the number of arguments provided.
    #[error("instruction `{0}` does not accept {1} arguments")]
    InvalidArgumentsCountError(String, usize),

    /// Error that occurs when a provided register number doesn't exist.
    ///
    /// This error occurs when a provided register number does not represent a valid register. This
    /// can be caused by a register number that is out of bounds, or by a register number that is
    /// not formatted correctly.
    ///
    /// # Arguments:
    ///
    /// * `0` - `usize` containing the invalid register number.
    #[error("invalid register number `{0}`")]
    InvalidRegisterNumberError(usize),

    /// Error that occurs when a provided register name doesn't exist.
    ///
    /// This error occurs when a provided register name does not represent a valid register. This
    /// can be caused by a register name that is not recognized by the parser, or by a register name
    /// that is not formatted correctly.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the invalid register name.
    #[error("invalid register name `{0}`")]
    InvalidRegisterNameError(String),

    /// Error that occurs when a provided immediate value for a character is invalid.
    ///
    /// This error occurs when a provided immediate value for a character is not valid. This can be
    /// caused by a character literal that is not formatted correctly, or by a character that is not
    /// recognized by the parser (i.e. a character literal that is not a valid UTF-8 character).
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the invalid character literal.
    #[error("invalid character immediate `{0}`")]
    CharacterImmediateError(String),

    /// Error that occurs when an attempted dereference is not possible.
    ///
    /// This error occurs when an attempted dereference is impossible. This can be caused by a
    /// dereference attempt on a type that doesn't represent memoery, or by a derefenence that is
    /// improperly formatted.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the attempted dereference.
    #[error("invalid dereference `{0}`")]
    DereferenceError(String),

    /// Error that occurs when a provided label definition is invalid.
    ///
    /// This error occurs when a label definition in the assembly is not valid. This can be caused
    /// by a label definition that is not formatted correctly, or by a label definition that is not
    /// recognized by the parser. Labels must be defined with a colon at the end of the label name.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the invalid label definition.
    #[error("invalid label definition `{0}`")]
    LabelDefinitionError(String),

    /// Error that occurs when a numberical immediate value is invalid.
    ///
    /// This error occurs when a numerical immediate value is not valid. This can be caused by a
    /// numerical literal that is not formatted correctly, or by a numerical literal that is not
    /// recognized by the parser. Numerical literals are valid integers or floating-point numbers
    /// (possibly including "f" suffix for floating-point numbers).
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the invalid numerical immediate.
    #[error("invalid numeric immediate `{0}`")]
    NumericalImmediateError(String),

    /// Error that occurs when a provided string immediate is invalid.
    ///
    /// This error occurs when a provided string immediate is not valid. This can be caused by a
    /// string literal that is not formatted correctly, or by a string literal that is not
    /// recognized by the parser. String literals must be enclosed in double quotes.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the invalid string immediate.
    #[error("invalid string immediate `{0}`")]
    StringImmediateError(String),

    /// Error that occurs when a provided register is invalid.
    ///
    /// This error occurs when a provided register is not valid. This can be caused by a register
    /// that is not recognized by the parser, or by a register that is not formatted correctly.
    /// Registers must be defined with a dollar sign at the beginning of the register name.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the invalid register.
    #[error("unrecognized register `{0}`")]
    UnknownRegisterError(String),

    /// Error that occurs when a provided token is unknown.
    ///
    /// This error occurs when a provided token is not recognized by the parser. This can be caused
    /// by a token that is not recognized by the parser, or by a token that is not formatted
    /// correctly.
    ///
    /// # Arguments:
    ///
    /// * `0` - `String` containing the unknown token.
    #[error("unrecognized token `{0}`")]
    UnknownTokenError(String),

    /// Error that occurs when a mutation is attempted on the zero register.
    ///
    /// This error occurs when a mutation is attempted on the zero register, which is immutable.
    /// The zero register is a special register that always contains the value zero, and cannot be
    /// modified.
    #[error("the zero register is not mutable")]
    ImmutableZeroRegisterError,
}

/// Represents errors that cannot be handled, but should not immediately panic.
///
/// When these errors are encountered, the user is notified of them, but the program does not halt.
/// Instead, the user can attempt to modify their input and try again. Should these errors occur,
/// it is likely due to a bug in the code, and should be reported to the developers.
#[derive(Error, Debug)]
pub enum InternalError {
    /// Error that occurs when a `try_into` fails, even though it shouldn't be possible.
    ///
    /// This error occurs when a `try_into` presumed to be unfailable fails. Should this error
    /// occur, it is likely due to a bug in the code, and should be reported to the developers.
    #[error("improper usage of try_into")]
    MismatchedTryIntoError,

    /// Error that occurs while accessing a target, even when it should be impossible.
    ///
    /// This error occurs when `get_input_output_target` is called in an invalid situation. Should
    /// this error occur, it is likely due to a bug in the code, and should be reported to the
    /// developers.
    #[error("improper usage of get_input_output_target")]
    GetInputOutputTargetError,
}

/// Represents errors that occur during the simulator's execution.
///
/// This includes errors from the EzASM instruction to `TransformationSequence` transpilation, as
/// well as those that occur during the execution of these sequences.
#[derive(Error, Debug)]
pub enum SimulatorError {
    
    /// Represents errors that occur during the parsing of EzASM instructions.
    ///
    /// Refer to the `ParserError` enum for more information.
    #[error(transparent)]
    ParserError(#[from] ParserError),

    /// Represents errors that cannot be handled, but should not immediately panic.
    ///
    /// Refer to the `InternalError` enum for more information.
    #[error(transparent)]
    InternalError(#[from] InternalError),

    /// Represents the errors that can occur during various types of I/O operations.
    ///
    /// Refer to the `IoError` enum for more information.
    #[error(transparent)]
    IoError(#[from] IoError),

    /// Error that occurs when a negative address is attempted to be read.
    ///
    /// This error occurs when an attempt is made to read from an address that is negative. This can
    /// be caused by an address that is not formatted correctly, or by an address that is out of
    /// bounds.
    ///
    /// # Arguments:
    ///
    /// * `0` - `i64` containing the negative address.
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
}

/// Represents the errors that can occur during various types of I/O operations.
#[derive(Error, Debug)]
pub enum IoError {

    /// Error that occurs within `std::io`.
    ///
    /// Refer to the `std::io::Error` struct for more information.
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    /// Error that occurs within `scanner_rust`.
    ///
    /// Refer to the `scanner_rust::ScannerError` struct for more information.
    #[error(transparent)]
    ScannerError(#[from] scanner_rust::ScannerError),

    /// Error that occurs when a file cannot be opened.
    #[error("could not open file `{0}`")]
    CouldNotOpenFileError(String),

    /// Error that occurs when a path that needs to point to a file doesn't do so.
    #[error("path `{0}` is not a file")]
    PathIsNotFileError(String),

    /// Error that occurs when a provided file is not found.
    #[error("file `{0}` does not exist")]
    FileDoesNotExistError(String),

    /// Error that occurs when the program attempts to read a file outside of its bounds.
    #[error("attempted to seek out of bounds in file")]
    OutOfBoundsError,

    /// Error that occurs when an input file is not encoded in UTF-8.
    #[error("some bytes are not UTF-8 in the input file")]
    UnsupportedEncodingError,

    /// Error that occurs when a file write fails.
    #[error("write operation failed")]
    WriteError,

    /// Error that occurs when a provided directory is not found.
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

// TODO: add example to following doccomment?

/// Panics on an error.
///
/// # Arguments:
///
/// * `error` - the error to display on panic.
///
/// # Panics
///
/// This method always panics with the process exit code of 1.
pub fn handle_error(error: EzasmError) -> ! {
    println!("{}", error);
    process::exit(1);
}
