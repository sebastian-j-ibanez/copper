// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Functions for REPL IO.

use std::fs::File;
use std::io::{self, BufRead, Write, stdout};
use std::process;

use crate::parser;

pub const COPPER_VERSION: &str = "0.2.0";

#[derive(Clone)]
pub enum InputType {
    Stdin,
    File(String),
}

/// Read unparsed expression from stdout or a file.
pub fn read_expression(input: InputType) -> String {
    match input {
        InputType::Stdin => stdin_input(),
        InputType::File(path) => file_input(path),
    }
}

/// Get expression from stdin.
fn stdin_input() -> String {
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

/// Get expression from file.
fn file_input(path: String) -> String {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("error: unable to open file: {}", e),
    };

    let mut lines = io::BufReader::new(file).lines();
    let mut buf = String::new();

    while !parser::expression_closed(&buf) {
        match lines.next() {
            Some(Ok(line)) => {
                buf.push_str(&line);
            }
            Some(Err(e)) => {
                eprintln!("error: unable to read line: {e}");
                process::exit(1);
            }
            None => {
                break;
            }
        }
    }

    buf
}

/// Print REPL greeting.
pub fn print_greeting() {
    println!("copper version {}", COPPER_VERSION);
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
