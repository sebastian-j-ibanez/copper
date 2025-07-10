// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

pub mod error;
pub mod types;
pub mod parser;
pub mod ui;

use std::io::{self, stdout, BufRead, Write};
use std::process;

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

        if let Some(line_objects) = parse_line(&buf) {
            print!("[");
            for (index, object) in line_objects.iter().enumerate() {
                print!("{}", object.get_type_name());
                if index != line_objects.len() - 1 {
                    print!(" ");
                }
            }
            println!("]");
            for (index, object) in line_objects.iter().enumerate() {
                print!("{}", object.to_str());
                if index != line_objects.len() - 1 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

