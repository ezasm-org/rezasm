use std::io::Stdin;

use scanner_rust::Scanner;

#[derive(Debug)]
pub enum StreamManager {
    Gui,
    Terminal(Scanner<Stdin>),
}