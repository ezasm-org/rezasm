#![allow(dead_code)]
#![allow(unused_variables)]

extern crate thiserror;

extern crate scanner_rust;

#[macro_use]
pub mod instructions;

pub mod parser;

pub mod simulation;

pub mod util;
