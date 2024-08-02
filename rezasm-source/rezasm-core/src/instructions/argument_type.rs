use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::InputTarget;
use crate::parser::lexer::Token;
use crate::util::error::{InternalError, ParserError};
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
    InputOutput(InputOutputTarget),
    Input(InputTarget),
}

impl TryInto<InputOutputTarget> for ArgumentType {
    type Error = InternalError;

    fn try_into(self) -> Result<InputOutputTarget, Self::Error> {
        match self {
            ArgumentType::InputOutput(s) => Ok(s),
            ArgumentType::Input(_) => Err(Self::Error::MismatchedTryIntoError),
        }
    }
}

impl TryInto<InputTarget> for ArgumentType {
    type Error = InternalError;

    fn try_into(self) -> Result<InputTarget, Self::Error> {
        match self {
            ArgumentType::InputOutput(_) => Err(Self::Error::MismatchedTryIntoError),
            ArgumentType::Input(s) => Ok(s),
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
    ) -> Result<ArgumentType, InternalError> {
        Ok(ArgumentType::InputOutput(match self {
            Token::Dereference(o, r) => {
                match InputOutputTarget::new_dereference_offset(r.clone(), o.clone()) {
                    Ok(value) => value,
                    Err(_) => return Err(InternalError::GetInputOutputTargetError),
                }
            }
            Token::Register(r) => match InputOutputTarget::new_register(r) {
                Ok(value) => value,
                Err(_) => return Err(InternalError::GetInputOutputTargetError),
            },
            _ => return Err(InternalError::GetInputOutputTargetError),
        }))
    }

    pub fn can_parse_input_output(&self) -> bool {
        match self {
            Token::Dereference(_, _) => true,
            Token::Register(_) => true,
            _ => false,
        }
    }
}
