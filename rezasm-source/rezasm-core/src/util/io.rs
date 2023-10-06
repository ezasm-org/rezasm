use std::{path::{Path, PathBuf}, fs, io::Write, ops::{Deref, DerefMut}};
use super::error::{IoError, EzasmError};

/// Rezasm file representation (reader).
#[derive(Debug)]
pub struct RezasmFileReader {
    path_buf: PathBuf,
    bytes: Vec<u8>,
    cursor: usize,
}

impl RezasmFileReader {
    /// Parse a file from path into a Rezasm file object.
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
    /// Move the cursor to the start.
    pub fn seek_start(&mut self) {
        self.cursor = 0;
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
            .ok_or(IoError::OutOfBoundsError)?;
        Ok(byte)
    }
    /// Peek a byte relative to the current position. 
    /// Does not set the cursor.
    pub fn peek_relative_byte(&mut self, rel_offset: isize) -> Result<u8, IoError> {
        let abs_offset = (self.cursor as isize + rel_offset) as usize;
        self.peek_absolute_byte(abs_offset)
    }
    /// Return the byte at the current position and then advance the cursor forward by one.
    /// Returns none if out of bounds.
    pub fn next(&mut self) -> Option<u8> {
        if self.cursor >= self.bytes.len() {
            None
        } else {
            self.cursor += 1;
            self.peek_relative_byte(-1).ok()
        }
    }
    /// Return the byte at the current position and then advance the cursor backward by one.
    /// Returns none if out of bounds.
    pub fn prev(&mut self) -> Option<u8> {
        if self.cursor == 0 {
            None
        } else {
            self.cursor -= 1;
            self.peek_relative_byte(1).ok()
        }
    }
    /// Check validity of cursor.
    pub fn is_cursor_valid(&self) -> bool {
        self.cursor < self.bytes.len()
    }
    /// Get the lines of the file.
    pub fn lines(&self) -> Result<Vec<String>, IoError> {
        let full = String::from_utf8(self.bytes())
            .map_err(|_| IoError::UnsupportedEncodingError)?;
        let lines = full.lines().map(|line| line.to_string()).collect();
        Ok(lines)
    }
    /// Convert the reader to a writer.
    /// Moves over the read bytes in the reader to the writer.
    /// If no path is provided, uses the same path from the reader.
    /// If you do not want to keep the bytes, just create the writer
    /// manually.
    /// This is useful for cases where you are done reading from a specific file
    /// and would want to edit that file now. Offering a path to this function
    /// would essentially be equivalent to a 'save as', where flushing would flush to a
    /// new file, whereas offering no path to this function will edit the same file that
    /// was read from, essentially acting as a file editor.
    pub fn into_writer<P: AsRef<Path>>(self, path: Option<P>) -> Result<RezasmFileWriter, IoError> {
        let path: PathBuf = path
            .map(|p| p.as_ref().to_path_buf())
            .unwrap_or(self.path_buf);
        let mut writer = RezasmFileWriter::new(path)?;
        writer.extend_from_slice(self.bytes.as_slice());
        Ok(writer)
    }
}

/// Rezasm file representation (writer).
/// Implicity derefs to its bytes vec.
#[derive(Debug)]
pub struct RezasmFileWriter {
    file: fs::File,
    bytes: Vec<u8>,
}

impl RezasmFileWriter {
    /// Create a Rezasm file writer object and the path to write to.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let path_buf = path.as_ref().to_path_buf();
        let file = fs::File::create(path_buf.clone())
            .map_err(|_| IoError::DirectoryError)?;
        Ok(Self {
            file: file,
            bytes: Vec::new(),
        })
    }
    /// Flush the byte buffer to the file.
    pub fn flush(&mut self) -> Result<(), IoError> {
        self.file.write_all(&self.bytes).map_err(|_| IoError::WriteError)
    }
}

impl Deref for RezasmFileWriter {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl DerefMut for RezasmFileWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}