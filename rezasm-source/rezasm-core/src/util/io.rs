use std::{path::{Path, PathBuf}, fs, error::Error, slice::Iter};
use super::error::IoError;

/// Rezasm file representation.
#[derive(Debug)]
pub struct RezAsmFile {
    lines: Vec<String>,
}

impl RezAsmFile {
    /// Parse a file from path into a rezasm file object.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let lines = contents
            .lines()
            .map(|s| s.to_string())
            .collect();
        Ok(Self {
            lines: lines,
        })
    }
    /// Get a line from lines.
    pub fn read_line(&self, line: usize) -> Result<&String, IoError> {
        self.lines.get(line).ok_or_else(|| IoError::SeekOutOfBounds(line))
    }
}

impl<'a> IntoIterator for &'a RezAsmFile {
    type Item = &'a String;
    type IntoIter = Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.iter()
    }
}