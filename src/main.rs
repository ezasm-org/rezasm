#[macro_use]
extern crate lazy_static;

use crate::parser::lexer::{
    self, get_character_immediate, get_number_type, is_label, is_register, parse_float_string,
    text_to_number, tokenize_line,
};
use crate::simulator::registry;
mod error;
mod instructions;
mod parser;
mod simulator;

fn main() {
    println!(
        "{:?}",
        tokenize_line(&String::from("add $t0 1 2 # hello there"))
    );
    println!("{:?}", text_to_number(String::from("10.5")).unwrap());
    // println!("{}", is_label(&String::from("a:")));
    // registry::initialize();
    println!("{:?}", registry::REGISTERS_MAP.len());

    for x in registry::REGISTERS_MAP.iter() {
        println!("{:?}", x);
    }
}
