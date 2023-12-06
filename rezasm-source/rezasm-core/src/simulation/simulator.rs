use std::fmt::Debug;

use super::reader::{DummyReader, Reader, ReaderBox};
use super::transform::transformable::Transformable;
use super::transform::transformation_sequence::TransformationSequence;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::parser::line::Line;
use crate::simulation::memory;
use crate::simulation::memory::Memory;
use crate::simulation::program::Program;
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::simulation::writer::{DummyWriter, Writer, WriterBox};
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;
use crate::util::word_size::{WordSize, DEFAULT_WORD_SIZE};

#[derive(Debug)]
pub struct Simulator {
    memory: Memory,
    registry: Registry,
    program: Program,
    word_size: WordSize,
    reader: ReaderBox,
    writer: WriterBox,
    sequence: Vec<TransformationSequence>,
    can_undo: bool,
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator::new_custom(
            &DEFAULT_WORD_SIZE,
            memory::DEFAULT_MEMORY_WORDS,
            Box::new(DummyReader::new()),
            Box::new(DummyWriter::new()),
        )
    }

    pub fn new_writer(writer: Box<dyn Writer>) -> Simulator {
        Simulator::new_custom(
            &DEFAULT_WORD_SIZE,
            memory::DEFAULT_MEMORY_WORDS,
            Box::new(DummyReader::new()),
            writer,
        )
    }

    pub fn new_reader(reader: Box<dyn Reader>) -> Simulator {
        Simulator::new_custom(
            &DEFAULT_WORD_SIZE,
            memory::DEFAULT_MEMORY_WORDS,
            reader,
            Box::new(DummyWriter::new()),
        )
    }

    pub fn new_custom(
        word_size: &WordSize,
        memory_size: usize,
        reader: Box<dyn Reader>,
        writer: Box<dyn Writer>,
    ) -> Simulator {
        let mut sim = Simulator {
            memory: Memory::new_sized(word_size, memory_size),
            registry: Registry::new(word_size),
            program: Program::new(),
            word_size: word_size.clone(),
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

    pub fn get_reader(&self) -> &ReaderBox {
        &self.reader
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

    pub fn get_reader_mut(&mut self) -> &mut ReaderBox {
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
            self.sequence.push(transform);
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

    pub fn set_writer(&mut self, writer: WriterBox) {
        self.writer = writer;
    }
}
