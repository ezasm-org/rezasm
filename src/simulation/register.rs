
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

#[derive(Debug)]
pub struct Register {
    data: RawData,
}

impl Register {
    pub fn new(word_size: &WordSize) -> Register {
        Register {
            data: RawData::empty_data(word_size),
        }
    }

    pub fn set_data(&mut self, data: RawData) {
        self.data = data;
    }

    pub fn get_data(&self) -> &RawData {
        &self.data
    }
}
