use crate::instructions::argument_type::ArgumentType;
use crate::instructions::implementation::memory_instructions::POP;
use crate::instructions::implementation::memory_instructions::PUSH;
use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::parser::lexer::parse_line;
use crate::parser::line::Line;
use crate::simulation::registry;
use crate::simulation::simulator::Simulator;
use crate::util::error::{ParserError, SimulatorError};
use crate::util::io::RezasmFileReader;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

lazy_static! {
    pub static ref IMPORT: Instruction =
        instruction!(import, |simulator: Simulator, input: InputTarget| {
            let address = input.get(&simulator)?.int_value() as usize;
            let file_path = simulator.get_memory().get_string(address)?;
            let mut relative_location = simulator.get_program().main_file();
            let relative_location_split = relative_location =
                match relative_location.rsplit_once('/') {
                    Some((first, _)) => format!("{}/", first),
                    None => String::new(),
                };
            let file = RezasmFileReader::new(&format!("{}{}", relative_location, file_path))?;
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
            simulator.add_lines(lines, file_path)
        });
    pub static ref JUMP: Instruction =
        instruction!(jump, |simulator: Simulator, input: InputTarget| {
            let word_size = simulator.get_word_size().clone();
            let value = input.get(&simulator)?;
            let pc = simulator.get_registers_mut().get_pc_mut();
            pc.set_data(value.clone());
            Ok(())
        });
    pub static ref CALL: Instruction =
        instruction!(call, |simulator: Simulator, input: InputTarget| {
            let word_size = simulator.get_word_size().clone();
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
            let pc_register = ArgumentType::Input(InputTarget::RegisterInput(
                registry::get_register_number(&registry::PC.into())?,
            ));
            let fid_register = ArgumentType::Input(InputTarget::RegisterInput(
                registry::get_register_number(&registry::FID.into())?,
            ));
            PUSH.call_function(simulator, &vec![pc_register])?;
            PUSH.call_function(simulator, &vec![fid_register])?;
            simulator
                .get_registers_mut()
                .get_pc_mut()
                .set_data(RawData::from_int(pc, &word_size));
            simulator
                .get_registers_mut()
                .get_fid_mut()
                .set_data(RawData::from_int(fid, &word_size));
            Ok(())
        });
    pub static ref RETURN: Instruction = instruction!(_return, |simulator: Simulator| {
        let pc_register = ArgumentType::InputOutput(InputOutputTarget::RegisterInputOutput(
            registry::get_register_number(&registry::PC.into())?,
        ));
        let fid_register = ArgumentType::InputOutput(InputOutputTarget::RegisterInputOutput(
            registry::get_register_number(&registry::FID.into())?,
        ));
        POP.call_function(simulator, &vec![fid_register])?;
        POP.call_function(simulator, &vec![pc_register])?;
        Ok(())
    });
    pub static ref EXIT: Instruction = instruction!(exit, |simulator: Simulator| {
        let word_size = simulator.get_word_size().clone();
        let end = simulator.get_program().end_pc(0) - 1;
        simulator
            .get_registers_mut()
            .get_pc_mut()
            .set_data(RawData::from_int(end as i64, &word_size));
        simulator
            .get_registers_mut()
            .get_fid_mut()
            .set_data(RawData::from_int(0i64, &word_size));
        Ok(())
    });
    pub static ref EXIT_STATUS: Instruction =
        instruction!(exit, |simulator: Simulator, input: InputTarget| {
            let return_value = input.get(&simulator)?;
            let r0 = simulator
                .get_registers_mut()
                .get_register_mut(&registry::R0.into())
                .unwrap();
            r0.set_data(return_value);
            let word_size = simulator.get_word_size().clone();
            let end = simulator.get_program().end_pc(0) - 1;
            simulator
                .get_registers_mut()
                .get_pc_mut()
                .set_data(RawData::from_int(end as i64, &word_size));
            simulator
                .get_registers_mut()
                .get_fid_mut()
                .set_data(RawData::from_int(0i64, &word_size));
            Ok(())
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
