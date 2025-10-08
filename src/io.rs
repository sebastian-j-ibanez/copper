// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Functions for REPL IO.

use std::fs::File;
use std::io::{self, BufRead, Write, stdout};
use std::process;

use colored::{self, Colorize};

use crate::env::EnvRef;
use crate::error::Error;
use crate::parser;
use crate::types::Expr;

pub const COPPER_VERSION: &str = "0.2.2";

/// Get expression from stdin.
pub fn stdin_input() -> String {
    if let Err(e) = stdout().flush() {
        eprintln!("error: {}", e.to_string());
        process::exit(1);
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

    buf
}

/// Get expressions from file.
pub fn file_input(path: String) -> Vec<String> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("error: {}", e),
    };

    let lines = io::BufReader::new(file).lines();
    let mut buf = String::new();
    let mut expressions = Vec::new();

    for line in lines {
        match line {
            Ok(line) => {
                buf.push_str(&line);
                buf.push_str("\n");

                if parser::expression_closed(&buf) {
                    expressions.push(buf.to_string());
                    buf.clear();
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                process::exit(1);
            }
        }
    }

    expressions
}

/// Process file input in an environment.
pub fn process_file_input(expressions: Vec<String>, env: EnvRef) {
    for expr in expressions {
        match parser::parse_and_eval(expr, env.clone()) {
            Ok(Expr::Void()) => continue,
            Ok(result) => println!("{}", result),
            Err(Error::Message(e)) => println!("error: {}", e),
        }
    }
}

/// Print REPL greeting.
pub fn print_greeting() {
    let banner = r#"
  _________  ____  ____  ___  _____
 / ___/ __ \/ __ \/ __ \/ _ \/ ___/
/ /__/ /_/ / /_/ / /_/ /  __/ /    
\___/\____/ .___/ .___/\___/_/     
         /_/   /_/"#;

    println!(
        "{}\n\nVersion {}",
        banner.truecolor(82, 127, 118).bold(),
        COPPER_VERSION
    );
    println!("Press Ctrl+C to exit!\n");
}

/// Print CLI help.
pub fn print_help() {
    println!("A Scheme interpreter written in Rust.\n");
    println!("Usage:\n\tcopper [flags]\n");
    println!("If no flags are provided, copper starts in REPL mode.\n");
    println!("Flags:\n");
    println!("-f, --file <PATH>\tRead Scheme file and open REPL.");
    println!("-h, --help\t\tPrint help.");
    println!("-v, --version\t\tPrint version.");
}

/// Print REPL prompt.
pub fn print_repl_prompt() {
    print!("> ");
}

/// Print version.
pub fn print_version() {
    println!("copper v{COPPER_VERSION}");
}
