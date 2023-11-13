use crate::util::error::SimulatorError;

pub const DEFAULT_WORD_SIZE: WordSize = WordSize::Four;

#[derive(Debug, Copy, Clone)]
pub enum WordSize {
    Four,
    Eight,
}

impl WordSize {
    pub fn value(&self) -> usize {
        match &self {
            WordSize::Four => 4,
            WordSize::Eight => 8,
        }
    }

    pub fn from(size: usize) -> Result<WordSize, SimulatorError> {
        match size {
            4 => Ok(WordSize::Four),
            8 => Ok(WordSize::Eight),
            _ => Err(SimulatorError::InvalidWordSizeError(size)),
        }
    }
}
