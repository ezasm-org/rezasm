use crate::util::cli_io::InputSource::{ConsoleInput, FileInput};
use crate::util::cli_io::OutputSink::{ConsoleOutput, FileOutput};
use rezasm_core::simulation::writer::Writer;
use rezasm_core::util::as_any::AsAny;
use rezasm_core::util::error::IoError;
use rezasm_core::util::io::{RezasmFileReader, RezasmFileWriter};
use scanner_rust::Scanner;
use std::any::Any;
use std::io::{stdin, stdout, Stdin, Write};

pub enum InputSource {
    FileInput(Scanner<RezasmFileReader>),
    ConsoleInput(Scanner<Stdin>),
}

impl InputSource {
    pub fn new_console() -> InputSource {
        ConsoleInput(Scanner::new(stdin()))
    }

    pub fn new_file(file: RezasmFileReader) -> InputSource {
        FileInput(Scanner::new(file))
    }

    pub fn read_line(&mut self) -> Result<String, IoError> {
        let s = match self {
            FileInput(s) => s.next_line()?,
            ConsoleInput(s) => s.next_line()?,
        };
        Ok(s.ok_or(IoError::OutOfBoundsError)?.trim().to_string())
    }

    pub fn read_word(&mut self) -> Result<String, IoError> {
        let s = match self {
            FileInput(s) => s.next()?,
            ConsoleInput(s) => s.next()?,
        };
        s.ok_or(IoError::OutOfBoundsError)
    }

    pub fn read_char(&mut self) -> Result<char, IoError> {
        let c = match self {
            FileInput(s) => s.next_char()?,
            ConsoleInput(s) => s.next_char()?,
        }
        .ok_or(IoError::OutOfBoundsError)?;
        if char::is_whitespace(c) {
            self.read_char()
        } else {
            Ok(c)
        }
    }

    pub fn read_raw(&mut self) -> Result<u8, IoError> {
        let b = match self {
            FileInput(s) => s.next_bytes(1)?,
            ConsoleInput(s) => s.next_bytes(1)?,
        };
        Ok(b.ok_or(IoError::OutOfBoundsError)?[0])
    }
}

#[derive(Debug)]
pub enum OutputSink {
    FileOutput(RezasmFileWriter),
    ConsoleOutput,
}

impl OutputSink {
    pub fn new_console() -> OutputSink {
        ConsoleOutput
    }

    pub fn new_file(file: RezasmFileWriter) -> OutputSink {
        FileOutput(file)
    }
}

impl Writer for OutputSink {}

impl Write for OutputSink {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        match self {
            ConsoleOutput => stdout().write(buf),
            FileOutput(file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        match self {
            ConsoleOutput => stdout().flush(),
            FileOutput(file) => file.flush(),
        }
    }
}

impl AsAny for OutputSink {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
