//! This module contains the core functionality of the Rezasm core.
//!
//! The core functionality of Rezasm is split into modules. The modules are:
//!
//! * `as_any`: Contains the AsAny trait, which is used to downcast a trait object.
//! * `error`: Contains the error types used in Rezasm.
//! * `io`: Contains the IO functionality of Rezasm.
//! * `raw_data`: Contains the RawData struct, which is used to hold raw binary data and provide
//!   methods to interact with it.
//! * `word_size`: Contains the WordSize enum, which is used to represent the simulator's word size.

pub mod as_any;
pub mod error;
pub mod io;
pub mod raw_data;
pub mod word_size;
