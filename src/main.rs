#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;

use crate::parser::lexer::{
    self, get_character_immediate, get_number_type, is_label, is_register, parse_float_string,
    text_to_number, tokenize_line,
};
use crate::simulation::memory;
use crate::simulation::memory::{Memory};
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::util::raw_data::RawData;
use std::f64::NAN;
use crate::util::word_size::WordSize;

mod error;
mod instructions;
mod parser;
mod simulation;
mod util;


fn main() {
    println!("{:?}", tokenize_line(&String::from("add $t0 1 2 # hello there")));
    // Should be ["add", "$t0", "1" "2"]

    let word_size = WordSize::Four;
    let mut memory: Memory = Memory::new();
    let data = RawData::from_int(100, &word_size);
    memory.write(memory.current_heap_pointer(), &data).unwrap();
    println!("{:?}", memory
        .read(memory.current_heap_pointer())
        .unwrap()
        .int_value()
    ); // Should be 100

    let mut registry: Registry = Registry::new(&word_size);
    registry.get_register_mut(&String::from("T0")).set_data(RawData::from_int(255, &word_size));
    println!("{:?}", registry.get_register(&String::from("T0")).get_data().int_value()); // Should be 255

    println!("{:?}", text_to_number(String::from("0x0010.8000")).unwrap()); // Should be Float(16.5)

    let k = RawData::from(0.1f64);
    println!("{}", <RawData as Into<f64>>::into(k));
}
