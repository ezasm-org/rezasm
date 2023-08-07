use crate::error::EzasmError;
use crate::simulator::memory::WordSize;
use bytebuffer::ByteBuffer;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct RawData {
    pub data: Vec<u8>,
    pub word_size: WordSize,
}

impl RawData {
    pub fn empty_data(size: &WordSize) -> RawData {
        RawData {
            data: vec![0; size.value()],
            word_size: size.clone(),
        }
    }

    pub fn new(data: &[u8]) -> RawData {
        RawData {
            data: data.to_vec(),
            word_size: WordSize::from(data.len()),
        }
    }

    pub fn get_iter(&self) -> impl Iterator<Item = &'_ u8> {
        self.data.iter()
    }

    pub fn int_value(&self) -> i64 {
        let mut buffer = ByteBuffer::from(self.data.clone());
        match self.word_size {
            WordSize::Four => buffer.read_i32().unwrap() as i64,
            WordSize::Eight => buffer.read_i64().unwrap(),
            WordSize::Error => 0i64,
        }
    }

    pub fn float_value(&self) -> f64 {
        let mut buffer = ByteBuffer::from(self.data.clone());
        match self.word_size {
            WordSize::Four => buffer.read_f32().unwrap() as f64,
            WordSize::Eight => buffer.read_f64().unwrap(),
            WordSize::Error => 0f64,
        }
    }

    pub fn from_int(int: i64, size: &WordSize) -> RawData {
        let mut buffer = ByteBuffer::new();
        match size {
            WordSize::Four => buffer.write_i32(int as i32),
            WordSize::Eight => buffer.write_i64(int),
            WordSize::Error => {}
        };
        RawData {
            data: buffer.into_vec(),
            word_size: size.clone(),
        }
    }

    pub fn from_float(float: f64, size: &WordSize) -> RawData {
        let mut buffer = ByteBuffer::new();
        match size {
            WordSize::Four => buffer.write_f32(float as f32),
            WordSize::Eight => buffer.write_f64(float),
            WordSize::Error => {}
        };
        RawData {
            data: buffer.into_vec(),
            word_size: size.clone(),
        }
    }
}

impl From<i64> for RawData {
    fn from(value: i64) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_i64(value);
        RawData {
            data: buffer.into_vec(),
            word_size: WordSize::Eight,
        }
    }
}

impl From<i32> for RawData {
    fn from(value: i32) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_i32(value);
        RawData {
            data: buffer.into_vec(), //TODO unwrap
            word_size: WordSize::Four,
        }
    }
}

impl Into<i64> for RawData {
    fn into(self) -> i64 {
        let mut buffer = ByteBuffer::from(self.data);
        match self.word_size {
            WordSize::Four => buffer.read_i32().unwrap() as i64,
            WordSize::Eight => buffer.read_i64().unwrap(),
            WordSize::Error => 0i64,
        }
    }
}

impl From<f64> for RawData {
    fn from(value: f64) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_f64(value);
        RawData {
            data: buffer.into_vec(), //TODO unwrap
            word_size: WordSize::Eight,
        }
    }
}

impl From<f32> for RawData {
    fn from(value: f32) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_f32(value);
        RawData {
            data: buffer.into_vec(), //TODO unwrap
            word_size: WordSize::Four,
        }
    }
}

impl Into<f64> for RawData {
    fn into(self) -> f64 {
        let mut buffer = ByteBuffer::from(self.data);
        match self.word_size {
            WordSize::Four => buffer.read_f32().unwrap() as f64,
            WordSize::Eight => buffer.read_f64().unwrap(),
            WordSize::Error => 0f64,
        }
    }
}
