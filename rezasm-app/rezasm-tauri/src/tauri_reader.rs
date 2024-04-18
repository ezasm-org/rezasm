use std::collections::VecDeque;
use std::io::{Read, Write};

use rezasm_core::{simulation::reader::Reader, util::as_any::AsAny};

#[derive(Debug)]
pub struct TauriReader {
    buffer: VecDeque<u8>,
}

impl TauriReader {
    pub fn new() -> TauriReader {
        TauriReader {
            buffer: VecDeque::new(),
        }
    }
}

impl Reader for TauriReader {}

impl Read for TauriReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl Write for TauriReader {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl AsAny for TauriReader {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
