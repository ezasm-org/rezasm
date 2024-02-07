use crate::util::cli_io::InputSource::{ConsoleInput, FileInput};
use crate::util::cli_io::OutputSink::{ConsoleOutput, FileOutput};
use rezasm_core::simulation::reader::Reader;
use rezasm_core::simulation::writer::Writer;
use rezasm_core::util::as_any::AsAny;
use rezasm_core::util::error::IoError;
use rezasm_core::util::io::{RezasmFileReader, RezasmFileWriter};
use scanner_rust::Scanner;
use std::any::Any;
use std::io::{self, stdin, stdout, Stdin, Write};

#[derive(Debug)]
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

    pub fn read_raw(&mut self) -> Result<u8, IoError> {
        let b = match self {
            FileInput(s) => s.next_bytes(1)?,
            ConsoleInput(s) => s.next_bytes(1)?,
        };
        Ok(b.ok_or(IoError::OutOfBoundsError)?[0])
    }
}

impl Reader for InputSource {
    fn expand_buffer(&mut self, new_input: &str) {
        //does nothing
    }

    fn flush_buffer(&mut self) {
        //does nothing
    }
}

impl io::Read for InputSource {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        match self {
            ConsoleInput(scanner) => {
                let next = scanner.next().unwrap().unwrap();
                buf.write(next.as_bytes())
            }
            FileInput(file) => {
                let next = file.next().unwrap().unwrap();
                buf.write(next.as_bytes())
            }
        }
    }
}

impl AsAny for InputSource {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
