use bytebuffer::ByteBuffer;

use crate::util::word_size::WordSize;

#[derive(Debug)]
pub struct RawData {
    pub data: Vec<u8>,
}

impl RawData {
    pub fn empty_data(size: &WordSize) -> RawData {
        RawData {
            data: vec![0; size.value()],
        }
    }

    pub fn new(data: &[u8]) -> RawData {
        RawData {
            data: data.to_vec(),
        }
    }

    pub fn get_iter(&self) -> impl Iterator<Item = &'_ u8> {
        self.data.iter()
    }

    pub fn int_value(&self) -> i64 {
        let mut buffer = ByteBuffer::from(self.data.clone());
        match WordSize::from(self.data.len()) {
            WordSize::Four => buffer.read_i32().unwrap() as i64,
            WordSize::Eight => buffer.read_i64().unwrap(),
            WordSize::Error => 0i64,
        }
    }

    pub fn float_value(&self) -> f64 {
        let mut buffer = ByteBuffer::from(self.data.clone());
        match WordSize::from(self.data.len()) {
            WordSize::Four => buffer.read_f32().unwrap() as f64,
            WordSize::Eight => buffer.read_f64().unwrap(),
            WordSize::Error => 0f64,
        }
    }

    pub fn string_value(&self) -> String {
        let mut buffer = ByteBuffer::from(self.data.clone());
        buffer.read_string().unwrap()
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
        }
    }
}

impl From<i64> for RawData {
    fn from(value: i64) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_i64(value);
        RawData {
            data: buffer.into_vec(),
        }
    }
}

impl From<i32> for RawData {
    fn from(value: i32) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_i32(value);
        RawData {
            data: buffer.into_vec(),
        }
    }
}

impl Into<i64> for RawData {
    fn into(self) -> i64 {
        let size = WordSize::from(self.data.len());
        let mut buffer = ByteBuffer::from(self.data);
        match size {
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
            data: buffer.into_vec(),
        }
    }
}

impl From<f32> for RawData {
    fn from(value: f32) -> Self {
        let mut buffer = ByteBuffer::new();
        buffer.write_f32(value);
        RawData {
            data: buffer.into_vec(),
        }
    }
}

impl Into<f64> for RawData {
    fn into(self) -> f64 {
        let size = WordSize::from(self.data.len());
        let mut buffer = ByteBuffer::from(self.data);
        match size {
            WordSize::Four => buffer.read_f32().unwrap() as f64,
            WordSize::Eight => buffer.read_f64().unwrap(),
            WordSize::Error => 0f64,
        }
    }
}

impl Clone for RawData {
    fn clone(&self) -> Self {
        RawData {
            data: self.data.clone(),
        }
    }
}
