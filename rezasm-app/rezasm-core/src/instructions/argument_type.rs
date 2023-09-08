use std::any::TypeId;
use std::str::FromStr;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::{Input, InputTarget};
use crate::instructions::targets::Target;
use crate::parser::lexer::Token;
use crate::util::error::EzasmError;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

#[derive(Debug, Clone)]
pub enum ArgumentType {
    InputOutput(InputOutputTarget),
    Input(InputTarget),
}

pub trait Downcast {
    fn downcast<T: Input>(&self) -> Option<&T>;
}

impl Downcast for ArgumentType {
    fn downcast<T: Input>(&self) -> Option<&T> {
        match self {
            ArgumentType::InputOutput(x) => x.as_any().downcast_ref::<T>(),
            ArgumentType::Input(x) => x.as_any().downcast_ref::<T>(),
        }
    }
}

impl ArgumentType {
    pub fn is_input_output(&self) -> bool {
        match self {
            ArgumentType::InputOutput(_) => true,
            ArgumentType::Input(_) => false,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            ArgumentType::InputOutput(_) => false,
            ArgumentType::Input(_) => true,
        }
    }

    pub fn get_input(&self) -> Option<Box<&InputTarget>> {
        match self {
            ArgumentType::InputOutput(_) => return None,
            ArgumentType::Input(x) => Some(Box::new(x)),
        }
    }

    pub fn get_input_output(&self) -> Option<Box<&InputOutputTarget>> {
        match self {
            ArgumentType::InputOutput(x) => Some(Box::new(x)),
            ArgumentType::Input(_) => return None,
        }
    }

    pub fn into_input(self) -> Option<Box<InputTarget>> {
        match self {
            ArgumentType::InputOutput(_) => return None,
            ArgumentType::Input(x) => Some(Box::new(x)),
        }
    }

    pub fn into_input_output(self) -> Option<Box<InputOutputTarget>> {
        match self {
            ArgumentType::InputOutput(x) => Some(Box::new(x)),
            ArgumentType::Input(_) => return None,
        }
    }

    pub fn get_mut_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<&mut InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<&mut InputTarget>(),
        }
    }

    pub fn get_ref_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<&InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<&InputTarget>(),
        }
    }

    pub fn get_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<InputTarget>(),
        }
    }

    pub fn get_target(&self) -> Box<dyn Target> {
        match self {
            ArgumentType::Input(t) => Box::new(t.clone()),
            ArgumentType::InputOutput(t) => Box::new(t.clone()),
        }
    }
}

impl Token {
    pub fn get_target(&self, word_size: &WordSize) -> Result<ArgumentType, EzasmError> {
        Ok(ArgumentType::Input(match self {
            Token::LabelReference(r) => InputTarget::new_label_reference(r),
            Token::NumericalImmediate(crate::parser::lexer::EZNumber::Float(f)) => {
                InputTarget::new_immediate(RawData::from_float(f.clone(), word_size))
            }
            Token::NumericalImmediate(crate::parser::lexer::EZNumber::Integer(i)) => {
                InputTarget::new_immediate(RawData::from_int(i.clone(), word_size))
            }
            Token::StringImmediate(s) => InputTarget::new_string(s),
            Token::CharacterImmediate(c) => {
                InputTarget::new_immediate(RawData::from_int(c.clone() as i64, word_size))
            }
            Token::Register(r) => {
                return Ok(ArgumentType::InputOutput(
                    match InputOutputTarget::new_register(r) {
                        Ok(t) => t,
                        Err(e) => return Err(e),
                    },
                ))
            }
            Token::Dereference(d) => {
                let lparen = d.find('(').unwrap();
                let rparen = d.rfind(')').unwrap();

                let register_string: String = d
                    .chars()
                    .skip(lparen + 1)
                    .take(rparen - lparen - 1)
                    .collect();
                let offset_string: String = d.chars().take(lparen - 1).collect();

                let offset: i64 = if offset_string.is_empty() {
                    0
                } else {
                    match i64::from_str(&offset_string) {
                        Ok(x) => x,
                        Err(_) => return Err(EzasmError::ParserError),
                    }
                };

                return Ok(ArgumentType::InputOutput(
                    match InputOutputTarget::new_dereference_offset(&register_string, offset) {
                        Ok(t) => t,
                        Err(e) => return Err(e),
                    },
                ));
            }
        }))
    }
}
