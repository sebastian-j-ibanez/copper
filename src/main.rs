// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

pub mod cli;
pub mod env;
pub mod error;
pub mod io;
pub mod macros;
pub mod parser;
pub mod tests;
pub mod types;

use crate::cli::{Flag, parse_args};
use crate::env::Env;
use crate::error::Error;
use crate::parser::parse_and_eval;
use crate::types::Expr;

fn main() {
    let args = std::env::args().skip(1).collect();
    let mut input_type = io::InputType::Stdin;
    match parse_args(args) {
        Some(Flag::File(f)) => input_type = io::InputType::File(f),
        Some(Flag::Help) => {
            io::print_help();
            std::process::exit(0);
        }
        Some(Flag::Version) => {
            io::print_version();
            std::process::exit(0);
        }
        None => {}
    }

    io::print_greeting();
    let env = Env::standard_env();

    // Read, eval, print loop
    loop {
        io::print_repl_prompt();

        let input = io::read_expression(input_type.clone());

        match parse_and_eval(input, env.clone()) {
            Ok(Expr::Void()) => continue,
            Ok(result) => println!("{}", result),
            Err(Error::Message(e)) => println!("error: {}", e),
        }
    }
}
