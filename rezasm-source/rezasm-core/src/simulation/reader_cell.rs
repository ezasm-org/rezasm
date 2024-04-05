use std::cell::RefCell;
use std::io::{self, Read};
use std::sync::Arc;

use scanner_rust::ScannerAscii;

use crate::util::as_any::AsAny;

use super::reader::Reader;

/// ASCII Scanner of a reader cell.
pub type Scanner = ScannerAscii<ReaderCell>;

/// Structure for a reference-counted pointer to a `Reader` with interior mutability.
///
/// Essentially, this means that multiple structures/variables can create mutable references to the
/// enclosed `Reader`.
///
/// # Trait Implementations
///
/// * `Reader` - This structure passes through the implementation of the enclosed `Reader` trait,
///   by providing the following implementations:
///   * `io::Read` - for consuming part of the reader's buffer.
///   * `AsAny`
///   * `Debug`
///   * `Send`
///   * `Sync`
///
/// # Panics
///
/// * When a mutable reference to the interior `Reader` already exists, yet another one is
///   requested, the program must panic in order to preserve Rust's memory safety guarantees.
#[derive(Debug)]
pub struct ReaderCell(Arc<RefCell<dyn Reader>>);

impl ReaderCell {
    /// Creates a new reader cell from a reader.
    ///
    /// # Examples
    ///
    /// ```
    /// let reader_cell = ReaderCell::new(std::io::stdin());
    /// ```
    pub fn new<R: Reader + 'static>(reader: R) -> Self {
        Self(Arc::new(RefCell::new(reader)))
    }
}

impl Reader for ReaderCell {}
unsafe impl Send for ReaderCell {}
unsafe impl Sync for ReaderCell {}

impl Clone for ReaderCell {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Read for ReaderCell {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (*self.0.try_borrow_mut().unwrap()).read(buf)
    }
}

impl AsAny for ReaderCell {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
