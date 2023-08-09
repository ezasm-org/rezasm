#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;

use crate::instructions::instruction::something;
use crate::parser::lexer::{
    self, get_character_immediate, get_number_type, is_label, is_register, parse_float_string,
    text_to_number, tokenize_line,
};
use crate::simulation::memory;
use crate::simulation::memory::{Memory};
use crate::simulation::registry;
use crate::simulation::registry::Registry;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;
use crate::instructions::targets::*;
use crate::instructions::targets::input_output_target::{InputOutput, InputOutputTarget};
use crate::instructions::targets::input_target::{Input, InputTarget};
use crate::instructions::targets::output_target::Output;
use crate::simulation::register::Register;
use crate::simulation::simulator::Simulator;

mod error;
mod instructions;
mod parser;
mod simulation;
mod util;


fn main() {
    // println!("{:?}", tokenize_line(&String::from("add $t0 1 2 # hello there")));
    // // Should be ["add", "$t0", "1" "2"]
    //
    // let word_size = WordSize::Four;
    // let mut memory: Memory = Memory::new();
    // let data = RawData::from_int(100, &word_size);
    // memory.write(memory.current_heap_pointer(), &data).unwrap();
    // println!("{:?}", memory
    //     .read(memory.current_heap_pointer())
    //     .unwrap()
    //     .int_value()
    // ); // Should be 100
    //
    // let mut registry: Registry = Registry::new(&word_size);
    // registry.get_register_mut(&String::from(registry::T0)).unwrap().set_data(RawData::from_int(255, &word_size));
    // println!("{:?}", registry.get_register(&String::from(registry::T0)).unwrap().get_data().int_value()); // Should be 255
    //
    // println!("{:?}", text_to_number(String::from("0x0010.8000")).unwrap()); // Should be Float(16.5)
    //
    // let k = RawData::from(0.1f64);
    // println!("{}", <RawData as Into<f64>>::into(k));
    //
    // let mut sim: Simulator = Simulator::new();
    //
    // let z: Box<dyn Target> = Box::new(InputOutputTarget::RegisterInputOutput(0usize));
    // let mut h = sim.downcast::<InputOutputTarget>(z).unwrap();
    // h.set(&mut sim, RawData::from(123.5f32)).unwrap();
    // println!("{:?}", h.get(&sim).unwrap().float_value());
    something();
}
