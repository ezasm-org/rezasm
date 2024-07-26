use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry;
use crate::instructions::instruction_registry::is_instruction_name_registered;
use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::*;
use crate::util::error::ParserError;
use crate::util::word_size::WordSize;
use std::any::TypeId;
use std::fmt::{Display, Formatter};

use super::lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Instruction(&'static Instruction, Vec<ArgumentType>),
    Label(String),
}

impl Line {
    pub fn new(
        instruction: &str,
        args: Vec<String>,
        word_size: &WordSize,
    ) -> Result<Self, ParserError> {
        if looks_like_label(instruction) {
            return if is_label(instruction) {
                Ok(Line::Label(
                    instruction[0..instruction.len() - 1].to_string(),
                ))
            } else {
                Err(ParserError::LabelDefinitionError(instruction.to_string()))
            };
        } else if !is_instruction_name_registered(instruction) {
            return Err(ParserError::InvalidInstructionError(
                instruction.to_string(),
            ));
        }

        let mut args_out: Vec<Token> = Vec::new();
        for arg in &args {
            if looks_like_string_immediate(arg) {
                args_out.push(Token::StringImmediate(lexer::get_string_immediate(
                    &arg.trim_matches('"').to_string(),
                )?));
            } else if looks_like_dereference(arg) {
                args_out.push(get_dereference(arg)?);
            } else if looks_like_character_immediate(arg) {
                args_out.push(get_character_immediate(arg)?);
            } else if looks_like_numerical_immediate(arg) {
                args_out.push(get_numerical_immediate(arg)?);
            } else if is_register(arg) {
                args_out.push(get_register(arg)?);
            } else if looks_like_label_reference(arg) {
                args_out.push(Token::LabelReference(arg.to_string()));
            } else {
                return Err(ParserError::UnknownTokenError(arg.to_string()).into());
            }
        }

        let mut arguments: Vec<ArgumentType> = Vec::new();
        let instruction_retrieved =
            match instruction_registry::get_instruction(instruction, args_out.len()) {
                Ok(instruction) => instruction,
                Err(e) => return Err(e),
            };

        for (index, (argument, type_of)) in args_out
            .iter()
            .zip(instruction_retrieved.get_types())
            .enumerate()
        {
            if type_of == &TypeId::of::<&mut InputTarget>() {
                arguments.push(argument.get_input_target(word_size)?);
            } else if type_of == &TypeId::of::<&mut InputOutputTarget>()
                && argument.can_parse_input_output()
            {
                arguments.push(argument.get_input_output_target(word_size)?);
            } else {
                return Err(ParserError::InvalidArgumentsError(
                    instruction.to_string(),
                    args[index].to_string(),
                    index,
                ));
            }
        }
        Ok(Line::Instruction(instruction_retrieved, arguments))
    }

    pub fn get_string_immediates(&self) -> Vec<String> {
        match self {
            Line::Instruction(_, args) => {
                let mut string_immediates = Vec::new();
                for arg in args {
                    match arg {
                        ArgumentType::Input(input) => match input {
                            InputTarget::StringInput(string) => {
                                string_immediates.push(string.clone())
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                string_immediates
            }
            _ => Vec::new(),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{}",
                match self {
                    Line::Instruction(instruction, arguments) =>
                        format!("Instruction {{ {:?} }}", instruction),
                    Line::Label(label) => format!("Label {{ {} }}", label),
                }
            )
            .as_str(),
        )
    }
}
