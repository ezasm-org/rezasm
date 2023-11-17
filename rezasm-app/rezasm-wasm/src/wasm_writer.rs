use rezasm_core::simulation::writer::Writer;
use rezasm_core::util::as_any::AsAny;
use std::any::Any;
use std::io::Write;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = self, js_name = emitPrintString)]
    fn print_string(s: &str);
}

#[derive(Debug)]
pub struct WasmWriter {}

impl WasmWriter {
    pub fn new() -> WasmWriter {
        WasmWriter {}
    }
}

impl Writer for WasmWriter {}

impl Write for WasmWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        print_string(String::from_utf8_lossy(buf).to_string().as_str());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl AsAny for WasmWriter {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
