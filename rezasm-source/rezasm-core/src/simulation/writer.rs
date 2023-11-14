use crate::util::as_any::AsAny;
use std::any::Any;
use std::fmt::Debug;
use std::io::Write;

pub trait Writer: Write + AsAny + Sync + Send + Debug {}

pub type WriterBox = Box<dyn Writer>;

#[derive(Debug)]
pub struct DummyWriter {}

impl DummyWriter {
    pub fn new() -> DummyWriter {
        DummyWriter {}
    }
}

impl Writer for DummyWriter {}

impl AsAny for DummyWriter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Write for DummyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
