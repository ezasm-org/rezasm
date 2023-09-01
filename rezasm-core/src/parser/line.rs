use crate::instructions::argument_type::ArgumentType;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::*;
use crate::util::error::EzasmError;
use crate::util::word_size::WordSize;

#[derive(Debug, Clone)]
pub enum Line {
    Instruction(String, Vec<ArgumentType>),
    Label(String),
}

impl Line {
    pub fn new(
        instruction: &String,
        args: Vec<String>,
        word_size: &WordSize,
    ) -> Result<Self, EzasmError> {
        if is_label(instruction) {
            //cloning here might not be ideal long term.
            return Ok(Line::Label(
                instruction[0..instruction.len() - 1].to_string(),
            ));
        } else if !is_instruction(instruction) {
            return Err(EzasmError::ParserError);
        }

        let mut args_out: Vec<Token> = Vec::new();
        for arg in args {
            if looks_like_string_immediate(&arg) {
                args_out.push(Token::StringImmediate(arg));
            } else if looks_like_dereference(&arg) {
                args_out.push(Token::Dereference(arg));
            } else if looks_like_character_immediate(&arg) {
                match get_character_immediate(&arg) {
                    Ok(c) => args_out.push(Token::CharacterImmediate(c)),
                    Err(_) => return Err(EzasmError::ParserError),
                }
            } else if looks_like_numerical_immediate(&arg) {
                match text_to_number(arg) {
                    Ok(i) => args_out.push(Token::NumericalImmediate(i)),
                    Err(_) => return Err(EzasmError::ParserError),
                }
            } else if is_register(&arg) {
                args_out.push(Token::Register(arg));
            } else if looks_like_label_reference(&arg) {
                args_out.push(Token::LabelReference(arg));
            } else {
                return Err(EzasmError::ParserError);
            }
        }

        let mut arguments: Vec<ArgumentType> = Vec::new();
        for argument in args_out {
            arguments.push(match argument.get_target(word_size) {
                Ok(arg) => arg,
                Err(error) => return Err(error),
            });
        }

        Ok(Line::Instruction(instruction.clone(), arguments))
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
