use std::collections::HashMap;
use crate::error::EzasmError;
use crate::parser::line::Line;
use crate::simulation::memory::Memory;
use crate::util::word_size::{DEFAULT_WORD_SIZE, WordSize};
use crate::simulation::register::Register;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::util::raw_data::RawData;
use crate::simulation::memory;

#[derive(Debug)]
pub struct Simulator {
    memory: Memory,
    registry: Registry,
    lines: Vec<Line>,
    label_map: HashMap<String, i64>,
    word_size: WordSize,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator::new_custom(&DEFAULT_WORD_SIZE, memory::DEFAULT_MEMORY_WORDS)
    }

    pub fn new_custom(word_size: &WordSize, memory_size: usize) -> Simulator {
        let mut sim = Simulator {
            memory: Memory::new_sized(word_size, memory_size),
            registry: Registry::new(word_size),
            lines: Vec::new(),
            label_map: HashMap::new(),
            word_size: word_size.clone(),
        };
        sim.initialize();
        sim
    }

    fn initialize(&mut self) {
        self.registry
            .get_register_mut(&String::from(registry::SP)).unwrap()
            .set_data(RawData::from_int(self.memory.initial_stack_pointer() as i64, &self.word_size));
    }

    pub fn reset_data(&mut self) {
        self.memory.reset();
        self.registry.reset();
    }

    pub fn reset(&mut self) {
        self.reset_data();
        self.label_map.clear();
        self.initialize();
    }

    pub fn add_line(&mut self, line: Line) -> Result<(), EzasmError> {
        match &line {
            Line::Label(string) => {
                self.label_map.insert(String::from(string), self.lines.len() as i64);
            }
            _ => {},
        };
        match self.memory.add_string_immediates(line.get_string_immediates()) {
            Ok(_) => {},
            Err(error) => return Err(error)
        };
        self.lines.push(line);
        Ok(())
    }

    pub fn add_lines(&mut self, lines: Vec<Line>) -> Result<(), EzasmError>{
        for line in lines {
            match self.add_line(line) {
                Ok(_) => {},
                Err(error) => return Err(error)
            };
        }
        Ok(())
    }

    fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn get_word_size(&self) -> &WordSize {
        &self.word_size
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn get_registers(&self) -> &Registry {
        &self.registry
    }

    pub fn get_word_size_mut(&mut self) -> &mut WordSize {
        &mut self.word_size
    }

    pub fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn get_registers_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }

    pub fn end_pc(&self) -> usize {
        return self.lines.len() + 1;
    }

    pub fn is_done(&self) -> bool {
        self.lines.is_empty() || self.registry.get_pc().get_data().int_value() == self.end_pc() as i64
    }

    pub fn is_error(&self) -> bool {
        let line = self.registry.get_pc().get_data().int_value();
        (line > self.lines.len() as i64) || (line < 0)
    }

    pub fn validate_pc(&self) -> Result<i64, EzasmError> {
        if self.is_error() {
            Err(EzasmError::InvalidProgramCounterError(self.registry.get_pc().get_data().int_value()))
        } else {
            Ok(self.registry.get_pc().get_data().int_value())
        }
    }

    pub fn run_line(&self, line: &Line) -> Result<(), EzasmError> {
        todo!()
    }

    pub fn run_line_from_pc(&self) -> Result<(), EzasmError> {
        let line_number = match self.validate_pc() {
            Ok(x) => {x},
            Err(error) => return Err(error)
        };
        self.run_line(&self.lines[line_number as usize])
    }

    pub fn apply_transformation(&self) -> Result<(), EzasmError> {
        todo!()
    }

    pub fn get_label_line_number(&self, label: &String) -> Result<&i64, EzasmError> {
        self.label_map.get(label).ok_or(EzasmError::NonExistantLabelError(label.clone()))
    }

}
