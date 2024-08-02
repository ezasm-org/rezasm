use std::fmt::Debug;

use scanner_rust::ScannerAscii;

use super::reader::DummyReader;
use super::reader_cell::{ReaderCell, Scanner};
use super::transform::transformable::Transformable;
use super::transform::transformation_sequence::TransformationSequence;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::parser::line::Line;
use crate::simulation::memory;
use crate::simulation::memory::Memory;
use crate::simulation::program::Program;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::simulation::writer::{DummyWriter, WriterBox};
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;
use crate::util::word_size::{WordSize, DEFAULT_WORD_SIZE};

#[derive(Debug)]
pub struct Simulator {
    memory: Memory,
    registry: Registry,
    program: Program,
    word_size: WordSize,
    scanner: Scanner,
    reader: ReaderCell,
    writer: WriterBox,
    sequence: Vec<TransformationSequence>,
    can_undo: bool,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator::new_custom(
            &DEFAULT_WORD_SIZE,
            memory::DEFAULT_MEMORY_WORDS,
            ReaderCell::new(DummyReader::new()),
            Box::new(DummyWriter::new()),
        )
    }
    
    pub fn new_writer(writer: WriterBox) -> Self {
        Simulator::new_custom_reader_writer(
            ReaderCell::new(DummyReader::new()),
            writer
        )
    }

    pub fn new_custom_reader_writer(reader: ReaderCell, writer: WriterBox) -> Simulator {
        Simulator::new_custom(
            &DEFAULT_WORD_SIZE,
            memory::DEFAULT_MEMORY_WORDS,
            reader,
            writer,
        )
    }

    pub fn new_custom(
        word_size: &WordSize,
        memory_size: usize,
        reader: ReaderCell,
        writer: WriterBox,
    ) -> Simulator {
        let mut sim = Simulator {
            memory: Memory::new_sized(word_size, memory_size),
            registry: Registry::new(word_size),
            program: Program::new(),
            word_size: word_size.clone(),
            scanner: ScannerAscii::new(reader.clone()),
            reader,
            writer,
            sequence: Vec::new(),
            can_undo: true,
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
        self.sequence.clear();
    }

    pub fn reset(&mut self) {
        self.reset_data();
        self.program.reset();
        self.initialize();
    }

    pub fn add_line(&mut self, line: Line, file: String) -> Result<(), SimulatorError> {
        self.memory
            .add_string_immediates(line.get_string_immediates())?;
        self.program.add_line(line, file)
    }

    pub fn add_lines(&mut self, lines: Vec<Line>, file: String) -> Result<(), SimulatorError> {
        for line in lines {
            self.add_line(line, file.clone())?;
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

    pub fn get_program(&self) -> &Program {
        &self.program
    }

    pub fn get_scanner_mut(&mut self) -> &mut Scanner {
        &mut self.scanner
    }

    pub fn get_reader_cell(&self) -> ReaderCell {
        self.reader.clone()
    }

    pub fn get_writer(&self) -> &WriterBox {
        &self.writer
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

    pub fn get_program_mut(&mut self) -> &mut Program {
        &mut self.program
    }

    pub fn get_reader_mut(&mut self) -> &mut ReaderCell {
        &mut self.reader
    }

    pub fn get_writer_mut(&mut self) -> &mut WriterBox {
        &mut self.writer
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

    fn run_line(&mut self, line: &Line) -> Result<(), SimulatorError> {
        let result = match line {
            Line::Instruction(instruction, args) => {
                instruction.get_function()(self, instruction.get_types(), &args)?
            }
            Line::Label(label) => {
                // no-op
                TransformationSequence::new_empty()
            }
        };
        if result.contains_nullop() {
            return Ok(());
        }
        self.apply_transformation(result)
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

    pub fn apply_transformation(
        &mut self,
        mut transform: TransformationSequence,
    ) -> Result<(), SimulatorError> {
        transform.apply(self)?;
        let pc_transformable = Transformable::InputOutputTransformable(
            InputOutputTarget::RegisterInputOutput(registry::PC_NUMBER),
        );
        let pc_transformation = pc_transformable.create_transformation(
            self,
            RawData::from_int(pc_transformable.get(self)?.int_value() + 1, &self.word_size),
        )?;
        pc_transformation.apply(self)?;

        if self.can_undo {
            transform.concatenate(TransformationSequence::new_single(pc_transformation));
            self.sequence.push(transform.clone());
            if transform.contains_nullop() {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn undo_last_transformation(&mut self) -> Result<bool, SimulatorError> {
        if !self.can_undo || self.sequence.is_empty() {
            Ok(false)
        } else {
            // unwrap is safe because emptiness is checked
            self.sequence.pop().unwrap().invert().apply(self)?;
            Ok(true)
        }
    }

    pub fn get_label_line_number(&self, label: &String) -> Result<i64, SimulatorError> {
        match self.program.resolve_label(label) {
            None => Err(SimulatorError::NonExistentLabelError(label.clone())),
            Some((_, line_number)) => Ok(line_number.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::implementation::register_instructions,
        parser::lexer::{parse_line, parse_lines},
    };

    use super::*;

    // Moved from Trevor's test in tests/core.rs
    #[test]
    pub fn test_simulator_instruction() {
        register_instructions();
        let mut simulator: Simulator = Simulator::new();

        let line = parse_line(&"add $t0 $t0 1".to_string(), simulator.get_word_size())
            .unwrap()
            .unwrap();
        let _ = simulator.add_line(line, "".to_string());
        let _ = simulator.run_line_from_pc();

        assert_eq!(
            simulator
                .get_registers()
                .get_register(&registry::T0.to_string())
                .unwrap()
                .get_data()
                .int_value(),
            1i64
        );
    }

    // Moved from Trevor's test in tests/core.rs
    #[test]
    pub fn test_simulator_labels() {
        register_instructions();
        let mut simulator: Simulator = Simulator::new();
        let program = "
        add $t0 0 0
        add $t1 0 1
        fib:
        add $t2 $t0 $t1
        add $t0 0 $t1
        add $t1 0 $t2
        add $pc 0 fib";
        let lines = parse_lines(&program.to_string(), simulator.get_word_size()).unwrap();
        match simulator.add_lines(lines, "".to_string()) {
            Ok(_) => {}
            Err(_) => assert!(false),
        }

        for _ in 0..50 {
            match simulator.run_line_from_pc() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }
        }

        assert_eq!(
            simulator
                .get_registers()
                .get_register(&registry::T1.to_string())
                .unwrap()
                .get_data()
                .int_value(),
            233i64
        );
    }
}
