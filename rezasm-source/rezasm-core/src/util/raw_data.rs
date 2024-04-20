//! RawData module.
//!
//! This module contains the RawData struct, which is used to hold raw binary data and provide
//! methods to interact with it, along with any implementations of the struct.

use bytebuffer::ByteBuffer;

use crate::util::word_size::WordSize;

/// Struct that holds raw binary data and provides methods to interact with it.
#[derive(Debug)]
pub struct RawData {
    pub data: Vec<u8>,
}

impl RawData {

    /// Create a new RawData instance full of 0s with a specified size.
    pub fn empty_data(size: &WordSize) -> RawData {
        RawData {
            data: vec![0; size.value()],
        }
    }

    /// Create a new RawData instance from a slice of bytes.
    pub fn new(data: &[u8]) -> RawData {
        RawData {
            data: data.to_vec(),
        }
    }

    /// Create an iterator over the bytes of the RawData instance.
    pub fn get_iter(&self) -> impl Iterator<Item = &'_ u8> {
        self.data.iter()
    }

    /// Read the integer value from the RawData instance.
    ///
    /// # Returns
    ///
    /// The integer value of the RawData instance if it is a valid length to be an integer, 0
    /// otherwise.
    pub fn int_value(&self) -> i64 {
        let mut buffer = ByteBuffer::from(self.data.clone());
        let size = match WordSize::from(self.data.len()) {
            Ok(x) => x,
            Err(_) => return 0i64,
        };
        match size {
            WordSize::Four => buffer.read_i32().unwrap() as i64,
            WordSize::Eight => buffer.read_i64().unwrap(),
        }
    }

    /// Read the float value from the RawData instance.
    ///
    /// # Returns
    ///
    /// The float value of the RawData instance if it is a valid length to be a float, 0f64
    /// otherwise.
    pub fn float_value(&self) -> f64 {
        let mut buffer = ByteBuffer::from(self.data.clone());
        let size = match WordSize::from(self.data.len()) {
            Ok(x) => x,
            Err(_) => return 0f64,
        };
        match size {
            WordSize::Four => buffer.read_f32().unwrap() as f64,
            WordSize::Eight => buffer.read_f64().unwrap(),
        }
    }

    /// Create a new RawData instance from an integer value.
    ///
    /// # Arguments
    ///
    /// * `int` - The integer value to be converted to RawData.
    /// * `size` - The simulator's word size.
    ///
    /// # Returns
    ///
    /// A new RawData instance with the bytes from the integer.
    pub fn from_int(int: i64, size: &WordSize) -> RawData {
        let mut buffer = ByteBuffer::new();
        match size {
            WordSize::Four => buffer.write_i32(int as i32),
            WordSize::Eight => buffer.write_i64(int),
        };
        RawData {
            data: buffer.into_vec(),
        }
    }

    /// Create a new RawData instance from a float value.
    ///
    /// # Arguments
    ///
    /// * `float` - The float value to be converted to RawData.
    /// * `size` - The simulator's word size.
    ///
    /// # Returns
    ///
    /// A new RawData instance with the bytes from the float.
    pub fn from_float(float: f64, size: &WordSize) -> RawData {
        let mut buffer = ByteBuffer::new();
        match size {
            WordSize::Four => buffer.write_f32(float as f32),
            WordSize::Eight => buffer.write_f64(float),
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
        let size = match WordSize::from(self.data.len()) {
            Ok(x) => x,
            Err(_) => return 0i64,
        };
        let mut buffer = ByteBuffer::from(self.data);
        match size {
            WordSize::Four => buffer.read_i32().unwrap() as i64,
            WordSize::Eight => buffer.read_i64().unwrap(),
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
        let size = match WordSize::from(self.data.len()) {
            Ok(x) => x,
            Err(_) => return 0f64,
        };
        let mut buffer = ByteBuffer::from(self.data);
        match size {
            WordSize::Four => buffer.read_f32().unwrap() as f64,
            WordSize::Eight => buffer.read_f64().unwrap(),
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
