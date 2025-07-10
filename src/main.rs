// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

pub mod error;
pub mod eval;
pub mod parser;
pub mod ui;

use std::io::{self, stdout, BufRead, Write};
use std::process;

use crate::eval::Symbol;
use crate::parser::parse_line;

fn main() {
    ui::print_greeting();    

    loop {
        print!("{}", ui::REPL_PROMPT);

        if let Err(e) = stdout().flush() {
            eprintln!("error: {}", e.to_string());
            process::exit(-1);
        }

        let mut buf = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        
        if let Err(e) = handle.read_line(&mut buf) {
            eprintln!("error: {}", e.to_string());
        }

        if let Some(symbols) = parse_line(&buf) {
            if symbols.len() >= 3 {
                match (&symbols[0], &symbols[1], &symbols[2]) {
                    (Symbol::Operator(op), Symbol::DataType(a), Symbol::DataType(b)) => {
                        match op.eval_operator(a, b) {
                            Ok(result) => {
                                println!("{}", result);
                            },
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                            }
                        }
                    },
                    (_, _, _) => {
                        eprintln!("expected [operator] [number] [number]");
                    },

                }
            } 
        }
    }
}

