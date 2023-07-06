#[macro_use]
extern crate lazy_static;

use crate::parser::lexer::{
    get_number_type, is_label, is_register, parse_float_string, tokenize_line, self,
    get_character_immediate
};
use crate::simulator::registry;
mod parser;
mod simulator;
mod error;
mod instructions;

fn main() {
    println!(
        "{:?}",
        tokenize_line(&String::from("add $t0 1 2 # hello there"))
    );
    // println!("{:?}", text_to_float(get_number_type(String::from("10.5"))).unwrap());
    // println!("{:?}", parse_hex_string(&String::from("10.1"), 2));
    // println!("{}", is_label(&String::from("a:")));
    // registry::initialize();
    // println!("{:?}", registry::REGISTER_BY_STRING);
    // println!("{:?}", registry::REGISTER_BY_INT.len());
}
