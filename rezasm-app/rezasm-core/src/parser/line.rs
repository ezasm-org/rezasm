use crate::instructions::argument_type::ArgumentType;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::{get_instruction, is_instruction_name_registered};
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::*;
use crate::util::error::EzasmError;
use crate::util::error::ParserError;
use crate::util::word_size::WordSize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Line {
    Instruction(&'static Instruction, Vec<ArgumentType>),
    Label(String),
}

impl Line {
    pub fn new(
        instruction: &String,
        args: Vec<String>,
        word_size: &WordSize,
    ) -> Result<Self, EzasmError> {
        if is_label(instruction) {
            return Ok(Line::Label(
                instruction[0..instruction.len() - 1].to_string(),
            ));
        } else if !is_instruction_name_registered(instruction) {
            return Err(EzasmError::InvalidInstructionError(instruction.to_string()));
        }

        let mut args_out: Vec<Token> = Vec::new();
        for arg in args {
            if looks_like_string_immediate(&arg) {
                args_out.push(Token::StringImmediate(arg));
            } else if looks_like_dereference(&arg) {
                args_out.push(get_dereference(&arg)?);
            } else if looks_like_character_immediate(&arg) {
                args_out.push(get_character_immediate(&arg)?);
            } else if looks_like_numerical_immediate(&arg) {
                args_out.push(get_numerical_immediate(&arg)?);
            } else if is_register(&arg) {
                args_out.push(get_register(&arg)?);
            } else if looks_like_label_reference(&arg) {
                args_out.push(Token::LabelReference(arg));
            } else {
                return Err(ParserError::UnknownTokenError(arg).into());
            }
        }

        let mut arguments: Vec<ArgumentType> = Vec::new();
        for argument in args_out {
            arguments.push(match argument.get_target(word_size) {
                Ok(arg) => arg,
                Err(error) => return Err(error),
            });
        }

        let matching_instruction = get_instruction(instruction, &arguments)?;

        Ok(Line::Instruction(matching_instruction, arguments))
    }

    pub fn get_string_immediates(&self) -> Vec<&String> {
        match self {
            Line::Instruction(_, args) => {
                let mut string_immediates = Vec::new();
                for arg in args {
                    match arg {
                        ArgumentType::Input(input) => match input {
                            InputTarget::StringInput(string) => string_immediates.push(string),
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
