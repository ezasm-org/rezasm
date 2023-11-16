use crate::get_window;
use rezasm_core::simulation::writer::Writer;
use rezasm_core::util::as_any::AsAny;
use std::any::Any;
use std::io::{ErrorKind, Write};

#[derive(Debug)]
pub struct GuiWriter {}

impl GuiWriter {
    pub fn new() -> GuiWriter {
        GuiWriter {}
    }
}

impl Writer for GuiWriter {}

impl Write for GuiWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let function = "tauri_print";
        match get_window().emit(function, String::from_utf8_lossy(buf)) {
            Ok(_) => Ok(buf.len()),
            Err(_) => Err(std::io::Error::new(ErrorKind::Other, "Web print error")),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl AsAny for GuiWriter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
