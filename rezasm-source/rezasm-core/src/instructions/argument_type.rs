use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::Token;
use crate::util::error::ParserError;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

#[derive(Debug, Clone)]
pub enum ArgumentType {
    InputOutput(InputOutputTarget),
    Input(InputTarget),
}

impl Into<InputOutputTarget> for ArgumentType {
    fn into(self) -> InputOutputTarget {
        match self {
            ArgumentType::InputOutput(s) => s,
            ArgumentType::Input(_) => panic!("Incorrect Into call (should never ever ever happen)"),
        }
    }
}

impl Into<InputTarget> for ArgumentType {
    fn into(self) -> InputTarget {
        match self {
            ArgumentType::InputOutput(_) => {
                panic!("Incorrect Into call (should neve ever ever happen)")
            }
            ArgumentType::Input(s) => s,
        }
    }
}

impl Token {
    pub fn get_input_target(&self, word_size: &WordSize) -> Result<ArgumentType, ParserError> {
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
            Token::Register(r) => InputTarget::new_register(r)?,
            Token::Dereference(o, r) => InputTarget::new_dereference_offset(r.clone(), o.clone())?,
        }))
    }

    pub fn get_input_output_target(
        &self,
        word_size: &WordSize,
    ) -> Result<ArgumentType, ParserError> {
        Ok(ArgumentType::InputOutput(match self {
            Token::Dereference(o, r) => {
                InputOutputTarget::new_dereference_offset(r.clone(), o.clone())?
            }
            Token::Register(r) => InputOutputTarget::new_register(r)?,
            _ => return Err(ParserError::InternalError),
        }))
    }
}
