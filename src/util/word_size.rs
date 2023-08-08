pub const DEFAULT_WORD_SIZE: WordSize = WordSize::Four;

#[derive(Debug, Copy, Clone)]
pub enum WordSize {
    Four,
    Eight,
    Error,
}

impl WordSize {
    pub fn value(&self) -> usize {
        match &self {
            WordSize::Four => 4,
            WordSize::Eight => 8,
            WordSize::Error => 0,
        }
    }

    pub fn from(size: usize) -> WordSize {
        match size {
            4 => WordSize::Four,
            8 => WordSize::Eight,
            _ => WordSize::Error,
        }
    }
}
