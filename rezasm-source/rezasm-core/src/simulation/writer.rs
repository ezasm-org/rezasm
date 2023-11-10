use std::fmt::Debug;
use std::io::Write;

pub trait Writer: Write + Send + Debug {}

#[derive(Debug)]
pub struct DummyWriter {}

impl DummyWriter {
    pub fn new() -> DummyWriter {
        DummyWriter {}
    }
}

impl Writer for DummyWriter {}

impl Write for DummyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
