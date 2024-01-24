use std::io::Read;

use rezasm_core::{simulation::reader::Reader, util::as_any::AsAny};

#[derive(Debug)]
pub struct TauriReader {
    buffer: Vec<char>
}

impl TauriReader {
    pub fn new() -> TauriReader {
        TauriReader { buffer: Vec::new() }
    }
}

impl Read for TauriReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let fallback_length = self.buffer.len();
        for i in 0..buf.len() {
            match self.buffer.first() {
                None => {return Ok(i);},
                Some(c) => buf[i] = c.clone() as u8,
            };
            self.buffer.remove(0);
        }
        Ok(fallback_length)
    }
}

impl Reader for TauriReader {
    fn expand_buffer(&mut self, new_input: &str) {
        let other_vec: Vec<char> = new_input.chars().collect();
        for c in other_vec {
            self.buffer.push(c);
        }
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
