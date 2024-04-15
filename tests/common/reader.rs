use std::io::{Read, Write};
use std::collections::VecDeque;

use rezasm_core::simulation::reader::Reader;
use rezasm_core::util::as_any::AsAny;

#[derive(Debug)]
pub struct TestReader {
    buffer: VecDeque<u8>,
}

impl TestReader {
    pub fn new() -> TestReader {
        TestReader {
            buffer: VecDeque::new()
        }
    }
}

impl Reader for TestReader {}

impl Read for TestReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl Write for TestReader {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl AsAny for TestReader {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
