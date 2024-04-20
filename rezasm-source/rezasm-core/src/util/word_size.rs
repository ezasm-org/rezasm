use crate::util::error::SimulatorError;

pub const DEFAULT_WORD_SIZE: WordSize = WordSize::Four;

/// Enum that represents the simulator's word size.
///
/// The word size is the number of bytes that the simulator uses to represent an integer or a float.
/// The word size can be either 4 bytes (32 bits) or 8 bytes (64 bits).
#[derive(Debug, Copy, Clone)]
pub enum WordSize {
    Four,
    Eight,
}

impl WordSize {

    /// Get the number of bytes in a word according to the word size.
    ///
    /// # Returns
    ///
    /// The number of bytes in a word.
    pub fn value(&self) -> usize {
        match &self {
            WordSize::Four => 4,
            WordSize::Eight => 8,
        }
    }

    /// Make a WordSize instance from a given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the word.
    ///
    /// # Returns
    ///
    /// A new WordSize instance.
    ///
    /// # Errors
    ///
    /// If the size is not 4 or 8, an InvalidWordSizeError is returned.
    pub fn from(size: usize) -> Result<WordSize, SimulatorError> {
        match size {
            4 => Ok(WordSize::Four),
            8 => Ok(WordSize::Eight),
            _ => Err(SimulatorError::InvalidWordSizeError(size)),
        }
    }
}
