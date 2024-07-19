use crate::simulation::writer::Writer;
use crate::util::as_any::AsAny;
use std::any::Any;
use std::fmt::Debug;
use std::io::Write;
use std::ops::Deref;

#[derive(Debug)]
pub struct TestWriter {
    string: String,
}

impl TestWriter {
    pub fn new() -> TestWriter {
        TestWriter {
            string: String::new(),
        }
    }

    pub fn get_data(&self) -> &String {
        &self.string
    }
}

impl Writer for TestWriter {}

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let data = String::from_utf8_lossy(buf);
        self.string = self.string.to_string() + data.deref();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl AsAny for TestWriter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
