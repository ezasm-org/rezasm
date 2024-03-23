#![cfg_attr(rustfmt, rustfmt_skip)] // prevent rustfmt from breaking 0 argument instruction macros

use crate::instructions::argument_type::ArgumentType;
use crate::instructions::implementation::memory_instructions;
use crate::instructions::implementation::memory_instructions::POP;
use crate::instructions::implementation::memory_instructions::PUSH;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::simulation::registry::FID_NUMBER;
use crate::simulation::registry::PC_NUMBER;
use crate::simulation::registry::RA_NUMBER;
use crate::simulation::transform::transformable::Transformable;
use crate::simulation::transform::transformation::Transformation;
use crate::simulation::transform::transformation_sequence::TransformationSequence;
use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::parse_line;
use crate::parser::line::Line;
use crate::simulation::registry;
use crate::util::error::{ParserError, SimulatorError};
use crate::util::io::RezasmFileReader;
use crate::util::raw_data::RawData;

lazy_static! {
    pub static ref IMPORT: Instruction =
        instruction!(import, |simulator: Simulator, input: InputTarget| {
            let address = input.get(&simulator)?.int_value() as usize;
            let given_file = simulator.get_memory().get_string(address)?;
            if simulator.get_program().file_exists(&given_file) {
                return Ok(TransformationSequence::new_empty());
            }
            let mut relative_location = simulator.get_program().main_file();
            relative_location = match relative_location.rsplit_once('/') {
                    Some((first, _)) => format!("{}/", first),
                    None => String::new(),
            };
            let file_name = format!("{}{}", relative_location, given_file);
            let file = RezasmFileReader::new(&file_name)?;
            let line_results: Vec<Option<Result<Line, ParserError>>> = file
                .lines()?
                .iter()
                .map(|line| parse_line(line, simulator.get_word_size()))
                .collect();
            let mut lines = Vec::new();
            for line in line_results {
                match line {
                    Some(l) => lines.push(l?),
                    None => {}
                }
            }
            simulator.add_lines(lines, given_file)?;
            Ok(TransformationSequence::new_empty())
        });

    pub static ref JUMP: Instruction =
        instruction!(jump, |simulator: Simulator, input: InputTarget| {
            let word_size = simulator.get_word_size().clone();
            let value = input.get(&simulator)?;
            let output = InputOutputTarget::new_register(&PC_NUMBER)?;
            let transformation = Transformation::new(
                Transformable::InputOutputTransformable(output),
                output.get(simulator)?,
                value.clone()
                );
            return Ok(TransformationSequence::new_single(transformation));
        });

    pub static ref CALL: Instruction =
        instruction!(call, |simulator: Simulator, input: InputTarget| {
            let ra_output = InputOutputTarget::new_register(&RA_NUMBER)?;
            let ra_transformable = Transformable::InputOutputTransformable(ra_output);
            let pc_output = InputOutputTarget::new_register(&PC_NUMBER)?;
            let pc_transformable = Transformable::InputOutputTransformable(pc_output);
            let fid_input = InputTarget::new_register(&FID_NUMBER)?;
            let fid_output = InputOutputTarget::new_register(&FID_NUMBER)?;
            let fid_transformable = Transformable::InputOutputTransformable(fid_output);
            let ra_register = ArgumentType::Input(InputTarget::RegisterInput(RA_NUMBER));
            let pc_register = ArgumentType::Input(InputTarget::RegisterInput(PC_NUMBER));
            let fid_register = ArgumentType::Input(InputTarget::RegisterInput(FID_NUMBER));
            let word_size = simulator.get_word_size().clone();
            let mut final_sequence = TransformationSequence::new_empty();

            final_sequence.concatenate(PUSH.call_function(simulator, &vec![ra_register])?);
            final_sequence.concatenate(memory_instructions::consecutive_push(simulator, fid_input, 1)?);

            let (fid, pc) = match input {
                InputTarget::LabelReferenceInput(label) => {
                    match simulator.get_program().resolve_label(&label) {
                        Some(value) => value.clone(),
                        None => return Err(SimulatorError::NonExistentLabelError(label)),
                    }
                }
                _ => (
                    input.get(&simulator)?.int_value(),
                    simulator.get_registers().get_fid().get_data().int_value(),
                ),
            };
            final_sequence.concatenate(TransformationSequence::new_single(pc_transformable.create_transformation(simulator, RawData::from_int(pc, &word_size))?));
            final_sequence.concatenate(TransformationSequence::new_single(fid_transformable.create_transformation(simulator, RawData::from_int(fid, &word_size))?));
            Ok(final_sequence)
        });

    pub static ref RETURN: Instruction = instruction!(_return, |simulator: Simulator,| {
        let ra_output = InputOutputTarget::new_register(&RA_NUMBER)?;
        let mut final_sequence = TransformationSequence::new_empty();
        let ra_register = ArgumentType::Input(InputTarget::RegisterInput(RA_NUMBER));
        let pc_register = ArgumentType::InputOutput(InputOutputTarget::RegisterInputOutput(PC_NUMBER));
        let fid_register = ArgumentType::InputOutput(InputOutputTarget::RegisterInputOutput(FID_NUMBER));
        final_sequence.concatenate(JUMP.call_function(simulator, &vec![ra_register.clone()])?);
        final_sequence.concatenate(POP.call_function(simulator, &vec![fid_register])?);
        final_sequence.concatenate(POP.call_function(simulator, &vec![ra_register])?);
        final_sequence.concatenate(memory_instructions::consecutive_pop(simulator, ra_output, 1)?);
        Ok(final_sequence)
    });

    pub static ref EXIT: Instruction = instruction!(exit, |simulator: Simulator,| {
        let pc_transformable = Transformable::InputOutputTransformable(InputOutputTarget::RegisterInputOutput(PC_NUMBER));
        let fid_transformable = Transformable::InputOutputTransformable(InputOutputTarget::RegisterInputOutput(FID_NUMBER));
        let word_size = simulator.get_word_size().clone();
        let end = simulator.get_program().end_pc(0) - 1;
        let mut final_sequence = TransformationSequence::new_empty();
        final_sequence.concatenate(TransformationSequence::new_single(pc_transformable.create_transformation(simulator, RawData::from_int(end as i64, &word_size))?));
        final_sequence.concatenate(TransformationSequence::new_single(fid_transformable.create_transformation(simulator, RawData::from_int(0i64, &word_size))?));
        Ok(final_sequence)
    });
    
    pub static ref EXIT_STATUS: Instruction =
        instruction!(exit, |simulator: Simulator, input: InputTarget| {
            let return_value = input.get(&simulator)?;
            let r0 = simulator
                .get_registers_mut()
                .get_register_mut(&registry::R0.into())
                .unwrap();
            let r0_transformable = Transformable::InputOutputTransformable(InputOutputTarget::RegisterInputOutput(registry::get_register_number(&registry::R0.to_string())?));
            let pc_transformable = Transformable::InputOutputTransformable(InputOutputTarget::RegisterInputOutput(PC_NUMBER));
            let fid_transformable = Transformable::InputOutputTransformable(InputOutputTarget::RegisterInputOutput(FID_NUMBER));
            let word_size = simulator.get_word_size().clone();
            let end = simulator.get_program().end_pc(0) - 1;
            let mut final_sequence = TransformationSequence::new_empty();
            final_sequence.concatenate(TransformationSequence::new_single(pc_transformable.create_transformation(simulator, RawData::from_int(end as i64, &word_size))?));
            final_sequence.concatenate(TransformationSequence::new_single(fid_transformable.create_transformation(simulator, RawData::from_int(0i64, &word_size))?));
            final_sequence.concatenate(TransformationSequence::new_single(r0_transformable.create_transformation(simulator, input.get(simulator)?)?));
            Ok(final_sequence)
        });
}

pub fn register_instructions() {
    register_instruction(&IMPORT);
    register_instruction(&JUMP);
    register_instruction(&CALL);
    register_instruction(&RETURN);
    register_instruction(&EXIT);
    register_instruction(&EXIT_STATUS);
}
