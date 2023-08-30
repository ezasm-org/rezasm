use crate::parser::lexer::*;
use crate::util::error::EzasmError;

#[derive(Debug)]
pub enum Line {
    Instruction(String, Vec<Token>),
    Label(String),
}

impl Line {
    pub fn new(instruction: &String, args: Vec<String>) -> Result<Self, EzasmError> {
        if is_label(instruction) {
            println!("?");
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
        Ok(Line::Instruction(instruction.clone(), args_out))
    }

    pub fn get_string_immediates(&self) -> Vec<&String> {
        match self {
            Line::Instruction(_, args) => {
                let mut string_immediates = Vec::new();
                for arg in args {
                    match arg {
                        Token::StringImmediate(s) => string_immediates.push(s),
                        _ => {}
                    }
                }
                string_immediates
            }
            _ => Vec::new(),
        }
    }
}
