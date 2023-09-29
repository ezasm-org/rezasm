use std::{path::{Path, PathBuf}, fs};
use super::error::{IoError, EzasmError};

/// Rezasm file representation.
#[derive(Debug)]
pub struct RezAsmFile {
    path_buf: PathBuf,
    bytes: Vec<u8>,
    cursor: usize,
}

impl RezAsmFile {
    /// Parse a file from path into a rezasm file object.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, EzasmError> {
        let path_buf = path.as_ref().to_path_buf();
        let bytes = fs::read(path_buf.clone())
            .map_err(|_| EzasmError::FileDoesNotExistError(path_buf.to_string_lossy().to_string()))?;
        Ok(Self {
            path_buf: path_buf,
            bytes: bytes,
            cursor: 0,
        })
    }
    /// Get a clone of the bytes.
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    /// Move the cursor to a specific byte relative to the
    /// start position, returning that byte.
    pub fn seek_absolute_byte(&mut self, abs_offset: usize) -> Result<u8, IoError> {
        self.cursor = abs_offset;
        self.peek_absolute_byte(self.cursor)
    }
    /// Move the cursor to a specific byte relative to the 
    /// current position, returning that byte.
    pub fn seek_relative_byte(&mut self, rel_offset: isize) -> Result<u8, IoError> {
        let abs_offset = (self.cursor as isize + rel_offset) as usize;
        self.seek_absolute_byte(abs_offset)
    }
    /// Peek a byte relative to the start position. 
    /// Does not set the cursor.
    pub fn peek_absolute_byte(&mut self, abs_offset: usize) -> Result<u8, IoError> {
        let byte = self.bytes
            .get(abs_offset)
            .cloned()
            .ok_or_else(|| IoError::OutOfBounds)?;
        Ok(byte)
    }
    /// Peek a byte relative to the current position. 
    /// Does not set the cursor.
    pub fn peek_relative_byte(&mut self, rel_offset: isize) -> Result<u8, IoError> {
        let abs_offset = (self.cursor as isize + rel_offset) as usize;
        self.peek_absolute_byte(abs_offset)
    }
    /// Advance the cursor forward by one, returning the byte at that 
    /// position or None if the cursor is out of bounds.
    pub fn next(&mut self) -> Option<u8> {
        self.seek_absolute_byte(self.cursor + 1).ok()
    }
    /// Advance the cursor backward by one, returning the byte at that 
    /// position or None if the cursor is out of bounds.
    pub fn prev(&mut self) -> Option<u8> {
        self.seek_absolute_byte(self.cursor - 1).ok()
    }
    /// Check validity of cursor.
    pub fn is_cursor_valid(&self) -> bool {
        self.cursor < self.bytes.len()
    }
    /// Get the lines of the file.
    pub fn lines(&mut self) -> Result<Vec<String>, IoError> {
        let full = String::from_utf8(self.bytes())
            .map_err(|_| IoError::UnsupportedEncoding)?;
        let lines = full.lines().map(|line| line.to_string()).collect();
        Ok(lines)
    }
}