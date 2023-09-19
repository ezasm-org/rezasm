use std::fmt::Debug;

use crate::parser::line::Line;
use crate::simulation::memory;
use crate::simulation::memory::Memory;
use crate::simulation::program::Program;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;
use crate::util::word_size::{WordSize, DEFAULT_WORD_SIZE};

#[derive(Debug)]
pub struct Simulator {
    memory: Memory,
    registry: Registry,
    program: Program,
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
            program: Program::new(),
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
        self.program.reset();
        self.initialize();
    }

    pub fn add_line(&mut self, line: Line) -> Result<(), SimulatorError> {
        match self
            .memory
            .add_string_immediates(line.get_string_immediates())
        {
            Ok(_) => {}
            Err(error) => return Err(error),
        };
        self.program.add_line(line, "".to_string())
    }

    pub fn add_lines(&mut self, lines: Vec<Line>) -> Result<(), SimulatorError> {
        for line in lines {
            match self.add_line(line) {
                Ok(_) => {}
                Err(error) => return Err(error),
            };
        }
        Ok(())
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
        let fid = self
            .registry
            .get_register(&registry::FID.to_string())
            .unwrap()
            .get_data()
            .int_value();
        return self.program.end_pc(fid);
    }

    pub fn is_done(&self) -> bool {
        let fid = self
            .registry
            .get_register(&registry::FID.to_string())
            .unwrap()
            .get_data()
            .int_value();
        let pc = self.registry.get_pc().get_data().int_value();
        self.program.is_done(fid, pc)
    }

    pub fn is_error(&self) -> bool {
        let fid = self
            .registry
            .get_register(&registry::FID.to_string())
            .unwrap()
            .get_data()
            .int_value();
        let pc = self.registry.get_pc().get_data().int_value();
        self.program.is_error(fid, pc)
    }

    pub fn validate_pc(&self) -> Result<i64, SimulatorError> {
        if self.is_error() {
            Err(SimulatorError::InvalidProgramCounterError(
                self.registry.get_pc().get_data().int_value(),
            ))
        } else {
            Ok(self.registry.get_pc().get_data().int_value())
        }
    }

    pub fn run_line(&mut self, line: &Line) -> Result<(), SimulatorError> {
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

    pub fn run_line_from_pc(&mut self) -> Result<(), SimulatorError> {
        let line_number = match self.validate_pc() {
            Ok(x) => x,
            Err(error) => return Err(error),
        };
        let fid = self
            .registry
            .get_register(&registry::FID.to_string())
            .unwrap();
        let line = self
            .program
            .get_line(fid.get_data().int_value(), line_number)?;
        self.run_line(&line.clone())
    }

    pub fn apply_transformation(&self) -> Result<(), SimulatorError> {
        todo!()
    }

    pub fn get_label_line_number(&self, label: &String) -> Result<i64, SimulatorError> {
        match self.program.resolve_label(label) {
            None => Err(SimulatorError::NonExistentLabelError(label.clone())),
            Some((_, line_number)) => Ok(line_number.clone()),
        }
    }
}
