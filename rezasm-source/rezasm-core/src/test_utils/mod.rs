#![cfg(test)]

mod writer;
pub use writer::TestWriter;

use std::path::PathBuf;

pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
