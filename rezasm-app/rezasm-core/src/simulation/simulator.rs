use std::collections::HashMap;
use std::fmt::Debug;

use crate::parser::line::Line;
use crate::simulation::memory;
use crate::simulation::memory::Memory;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::util::error::EzasmError;
use crate::util::raw_data::RawData;
use crate::util::word_size::{WordSize, DEFAULT_WORD_SIZE};

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
            .get_register_mut(&String::from(registry::SP))
            .unwrap()
            .set_data(RawData::from_int(
                self.memory.initial_stack_pointer() as i64,
                &self.word_size,
            ));
    }

    pub fn reset_data(&mut self) {
        self.memory.reset();
        self.registry.reset();
    }

    pub fn reset(&mut self) {
        self.reset_data();
        self.lines.clear();
        self.label_map.clear();
        self.initialize();
    }

    pub fn add_line(&mut self, line: Line) -> Result<(), EzasmError> {
        match &line {
            Line::Label(label) => {
                if self.label_map.contains_key(label) {
                    return Err(EzasmError::LabelInUseError(label.to_string()));
                } else {
                    self.label_map
                        .insert(String::from(label), self.lines.len() as i64);
                }
            }
            _ => {}
        };
        match self
            .memory
            .add_string_immediates(line.get_string_immediates())
        {
            Ok(_) => {}
            Err(error) => return Err(error),
        };
        self.lines.push(line);
        Ok(())
    }

    pub fn add_lines(&mut self, lines: Vec<Line>) -> Result<(), EzasmError> {
        for line in lines {
            match self.add_line(line) {
                Ok(_) => {}
                Err(error) => return Err(error),
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
        return self.lines.len();
    }

    pub fn is_done(&self) -> bool {
        let pc = self.registry.get_pc().get_data().int_value();
        self.lines.is_empty() && pc == 0 || pc == self.end_pc() as i64
    }

    pub fn is_error(&self) -> bool {
        let line = self.registry.get_pc().get_data().int_value();
        (line > self.lines.len() as i64) || (line < 0)
    }

    pub fn validate_pc(&self) -> Result<i64, EzasmError> {
        if self.is_error() {
            Err(EzasmError::InvalidProgramCounterError(
                self.registry.get_pc().get_data().int_value(),
            ))
        } else {
            Ok(self.registry.get_pc().get_data().int_value())
        }
    }

    pub fn run_line(&mut self, line: &Line) -> Result<(), EzasmError> {
        let result = match line {
            Line::Instruction(instruction, args) => {
                instruction.get_function()(self, instruction.get_types(), &args)
            }
            Line::Label(label) => {
                // no-op
                Ok(())
            }
        };
        let new_pc = self.registry.get_pc().get_data().int_value() + 1;
        self.registry
            .get_pc_mut()
            .set_data(RawData::from_int(new_pc, &self.word_size));

        result
    }

    pub fn run_line_from_pc(&mut self) -> Result<(), EzasmError> {
        let line_number = match self.validate_pc() {
            Ok(x) => x,
            Err(error) => return Err(error),
        };
        let line = match self.lines.get(line_number as usize) {
            None => return Err(EzasmError::SimualtorError),
            Some(x) => x,
        };
        self.run_line(&line.clone())
    }

    pub fn apply_transformation(&self) -> Result<(), EzasmError> {
        todo!()
    }

    pub fn get_label_line_number(&self, label: &String) -> Result<&i64, EzasmError> {
        self.label_map
            .get(label)
            .ok_or(EzasmError::NonExistentLabelError(label.clone()))
    }
}
