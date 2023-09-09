use std::collections::HashMap;

use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;
use crate::util::word_size::{WordSize, DEFAULT_WORD_SIZE};

pub const DEFAULT_MEMORY_WORDS: usize = 0x20_0000;

const DEFAULT_OFFSET: usize = 0x1_0000;
const DEFAULT_STRING_OFFSET: usize = 0x1_0000;

#[derive(Debug)]
pub struct Memory {
    // const
    word_size: WordSize,
    memory_size: usize,
    offset_bytes: usize,
    disallowed_bytes: usize,

    // non-const
    memory: Vec<u8>,
    alloc_index: usize,
    string_alloc_index: usize,
    string_address_map: HashMap<String, RawData>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory::new_sized(&DEFAULT_WORD_SIZE, DEFAULT_MEMORY_WORDS)
    }

    pub fn new_sized(word_size: &WordSize, memory_size: usize) -> Memory {
        let word_size = word_size.clone();
        let offset_bytes = word_size.value() * (DEFAULT_OFFSET + DEFAULT_STRING_OFFSET);
        let disallowed_bytes = word_size.value() * DEFAULT_OFFSET;
        let memory_size = offset_bytes + memory_size * word_size.value();

        Memory {
            word_size,
            memory_size,
            offset_bytes,
            disallowed_bytes,

            memory: vec![0; memory_size],
            alloc_index: offset_bytes,
            string_alloc_index: DEFAULT_STRING_OFFSET * word_size.value(),
            string_address_map: HashMap::new(),
        }
    }

    pub fn word_size(&self) -> &WordSize {
        &self.word_size
    }

    pub fn memory_size(&self) -> usize {
        self.memory_size
    }

    pub fn reset(&mut self) {
        self.memory = vec![0; self.memory_size];
        self.alloc_index = self.offset_bytes;
        self.string_alloc_index = DEFAULT_STRING_OFFSET * self.word_size.value();
        self.string_address_map = HashMap::new();
    }

    pub fn initial_stack_pointer(&self) -> usize {
        self.memory_size
    }

    pub fn initial_heap_pointer(&self) -> usize {
        self.offset_bytes
    }

    pub fn initial_text_pointer(&self) -> usize {
        DEFAULT_STRING_OFFSET * self.word_size.value()
    }

    pub fn current_heap_pointer(&self) -> usize {
        self.alloc_index
    }

    pub fn set_heap_pointer(&mut self, address: usize) -> Result<(), SimulatorError> {
        if address < self.offset_bytes || address > self.memory_size {
            return Err(SimulatorError::InvalidHeapPointerError(address));
        }
        self.alloc_index = address;
        Ok(())
    }

    pub fn read_bytes(&self, address: usize, count: usize) -> Result<RawData, SimulatorError> {
        if address < self.disallowed_bytes || address + count > self.memory_size {
            return Err(SimulatorError::ReadOutOfBoundsError(address));
        }
        Ok(RawData::new(&self.memory[address..address + count]))
    }

    pub fn read(&self, address: usize) -> Result<RawData, SimulatorError> {
        self.read_bytes(address, self.word_size.value())
    }

    pub fn write(&mut self, address: usize, data: &RawData) -> Result<(), SimulatorError> {
        if address + data.data.len() > self.memory_size {
            Err(SimulatorError::WriteOutOfBoundsError(address))
        } else {
            self.unsafe_write(address, data)
        }
    }

    pub fn unsafe_write(&mut self, address: usize, data: &RawData) -> Result<(), SimulatorError> {
        if address + data.data.len() > self.memory_size {
            Err(SimulatorError::WriteOutOfBoundsError(address))
        } else {
            for (index, byte) in data.get_iter().enumerate() {
                self.memory[address + index] = byte.clone();
            }
            Ok(())
        }
    }

    pub fn add_string_immediates(&mut self, strings: Vec<&String>) -> Result<(), SimulatorError> {
        for string in strings {
            if !self.string_address_map.contains_key(string) {
                if self.string_alloc_index + string.len() >= self.offset_bytes {
                    return Err(SimulatorError::StringRegionOutOfMemoryError(
                        string.to_string(),
                    ));
                }
                for (index, c) in string.as_bytes().iter().enumerate() {
                    match self.unsafe_write(
                        self.string_alloc_index + index * self.word_size.value(),
                        &RawData::from(c.clone() as i64),
                    ) {
                        Ok(_) => {}
                        Err(error) => return Err(error),
                    };
                }
                match self.unsafe_write(
                    self.string_alloc_index + string.len() * self.word_size.value(),
                    &RawData::empty_data(&self.word_size),
                ) {
                    Ok(_) => {}
                    Err(error) => return Err(error),
                };
                self.string_address_map.insert(
                    string.clone(),
                    RawData::from(self.string_alloc_index as i64),
                );
            }
        }
        Ok(())
    }

    pub fn get_string_immediate_address(
        &self,
        string: &String,
    ) -> Result<&RawData, SimulatorError> {
        match self.string_address_map.get(string) {
            Some(s) => Ok(s),
            None => Err(SimulatorError::StringImmediateDoesNotExistError(
                string.to_string(),
            )),
        }
    }
}
