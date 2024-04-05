use crate::util::as_any::AsAny;
use std::any::Any;
use std::fmt::Debug;
use std::io;

/// A trait for any readers used with EzASM
pub trait Reader: io::Read + io::Write + AsAny + Sync + Send + Debug {}

/// Placeholder reader that the GUI is created with, which should be replaced by said GUI during
/// its initialization.
///
/// HACK: in reality, the GUI should just be initialized with the correct reader
#[derive(Debug)]
pub struct DummyReader {}

impl DummyReader {
    pub fn new() -> DummyReader {
        DummyReader {}
    }
}

impl Reader for DummyReader {}

impl io::Write for DummyReader {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsAny for DummyReader {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl io::Read for DummyReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(0usize)
    }
}
