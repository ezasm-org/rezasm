#![allow(dead_code)]
#![allow(unused_variables)]

mod util;

extern crate lazy_static;
extern crate rezasm_core;
extern crate scanner_rust;

use std::process;

use rezasm_core::util::error::handle_error;

use crate::util::application::Application;
use crate::util::cli;
use crate::util::cli::Arguments;
use crate::util::cli_arguments::handle_arguments;

fn main() {
    let args: Arguments = cli::get_args();
    let application: Application = match handle_arguments(args) {
        Ok(app) => app,
        Err(error) => handle_error(error),
    };

    let exit_code = match application.run_until_completion() {
        Ok(exit_code) => exit_code,
        Err(error) => handle_error(error.into()),
    };

    process::exit(exit_code as i32);
}
