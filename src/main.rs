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

use std::cell::RefCell;
use std::rc::Rc;

use repl_lib::{ProcessFunc, TerminatedLineFunc};

use crate::cli::{Flag, parse_args};
use crate::env::Env;
use crate::error::Error;
use crate::parser::parse_and_eval;

fn process_line(env: Rc<RefCell<Env>>) -> ProcessFunc {
    Box::new(
        move |line: String| match parse_and_eval(line, env.clone()) {
            Ok(result) => Ok(result.to_string()),
            Err(Error::Message(e)) => Err(repl_lib::Error::ProcessLine(format!("{}", e))),
        },
    )
}

fn expression_closed() -> TerminatedLineFunc {
    Box::new(move |line: String| parser::expression_closed(line.as_str()))
}

fn main() {
    let env = Env::standard_env();

    // Process CLI args.
    let args = std::env::args().skip(1).collect();
    match parse_args(args) {
        Some(Flag::File(f)) => {
            let expressions = io::file_input(f.clone());
            io::process_file_input(expressions, env);
            std::process::exit(0);
        }
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

    let prompt = String::from("> ");
    let banner = String::from(
        r#"
        _________  ____  ____  ___  _____
        / ___/ __ \/ __ \/ __ \/ _ \/ ___/
        / /__/ /_/ / /_/ / /_/ /  __/ /    
        \___/\____/ .___/ .___/\___/_/     
        /_/   /_/"#,
    );
    let welcome_msg = format!("Version {}", io::COPPER_VERSION);
    let mut repl = match repl_lib::Repl::new(
        prompt,
        banner,
        welcome_msg,
        process_line(env),
        expression_closed(),
    ) {
        Ok(r) => r,
        Err(e) => panic!("bruh: {}", e),
    };

    // REPL.
    loop {
        repl.print_prompt();

        match repl.get_line() {
            Ok(line) => println!("{}", line),
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
