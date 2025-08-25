// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-29

//! Functions related to IO.

use crate::env::Env;
use crate::error::Error;
use crate::io;
use crate::types::Expr;

use std::cell::RefCell;
use std::rc::Rc;

/// Display raw expression in stdout.
pub fn display(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args.first() {
        Some(arg) => {
            print!("{}", arg);
            Ok(Expr::Void())
        }
        _ => Err(Error::Message("expected 1 valid expression".to_string())),
    }
}

/// Return newline character.
pub fn newline(_: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    println!();
    Ok(Expr::Void())
}

/// Print formatted value of expression in stdout.
pub fn print(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => print!("{}", s),
            _ => print!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::Message("expected 1 valid expression".to_string()))
}

/// Print formatted value of expression in stdout with a newline.
pub fn println(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => println!("{}", s),
            _ => println!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::Message("expected 1 valid expression".to_string()))
}

/// Evaluate the contents of a file.
pub fn load_file(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    let file = match args.first() {
        Some(Expr::String(f)) => f,
        _ => return Err(Error::Message("expected a string path".to_string())),
    };

    let expressions = io::file_input(file.to_owned());
    io::process_file_input(expressions, env);

    Ok(Expr::Void())
}

/// End process.
pub fn exit(_: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    std::process::exit(0);
}
