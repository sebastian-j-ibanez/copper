// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

pub mod env;
pub mod error;
pub mod parser;
pub mod tests;
pub mod types;
pub mod ui;

use std::io::{self, BufRead, Write, stdout};
use std::process;

use crate::error::Error;
use crate::parser::parse_eval;
use crate::env::Env;

fn main() {
    ui::print_greeting();
    let env = &mut Env::default_env();

    loop {
        print!("{}", ui::REPL_PROMPT);

        if let Err(e) = stdout().flush() {
            eprintln!("error: {}", e.to_string());
            process::exit(-1);
        }

        let mut buf = String::new();
        let mut handle = io::stdin().lock();

        if let Err(e) = handle.read_line(&mut buf) {
            eprintln!("error: {}", e.to_string());
        }

        while !parser::expression_closed(&buf) {
            if let Err(e) = handle.read_line(&mut buf) {
                eprintln!("error: {}", e.to_string());
            }
        }

        match parse_eval(buf, env) {
            Ok(result) => println!("{}", result),
            Err(e) => match e {
                Error::Message(msg) => println!("error: {}", msg),
            },
        }
    }
}
