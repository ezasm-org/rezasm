use std::fmt::Debug;
use std::io::Write;
use std::sync::{Mutex, MutexGuard};

pub trait Writer: Write + Send + Debug {}

#[derive(Debug)]
pub struct WriterMutex(Mutex<Box<dyn Writer>>);

pub type WriterGuard<'a> = MutexGuard<'a, Box<dyn Writer>>;

impl WriterMutex {
    pub fn new(data: Box<dyn Writer>) -> WriterMutex {
        WriterMutex(Mutex::new(data))
    }

    pub fn get(&self) -> WriterGuard {
        self.0.lock().unwrap()
    }
}

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
