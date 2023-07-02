#[macro_use]
extern crate lazy_static;

use crate::parser::lexer::{
    get_number_type, is_label, is_register, parse_float_string, text_to_float, tokenize_line,
};
use crate::simulator::registry;
mod parser;
mod simulator;

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
    for (key, value) in registry::REGISTER_BY_STRING.iter() {
        println!("{key}: {value}");
    }
    // println!("{:?}", registry::REGISTER_BY_INT.len());
    println!("{}", registry::is_register("$t0"));
    println!("{}", registry::is_register("$53"));
}
