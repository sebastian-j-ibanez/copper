// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-29

//! Functions related to IO.

use crate::error::Error;
use crate::types::Expr;

/// Display raw expression in stdout.
pub fn display(args: &[Expr]) -> Result<Expr, Error> {
    match args.first() {
        Some(arg) => {
            print!("{}", arg);
            Ok(Expr::Void())
        }
        _ => Err(Error::Message("expected 1 valid expression".to_string())),
    }
}

/// Return newline character.
pub fn newline(_: &[Expr]) -> Result<Expr, Error> {
    println!();
    Ok(Expr::Void())
}

/// Print formatted value of expression in stdout.
pub fn print(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => print!("\"{}\"", s),
            _ => print!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::Message("expected 1 valid expression".to_string()))
}

/// Print formatted value of expression in stdout with a newline.
pub fn println(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => println!("\"{}\"", s),
            _ => println!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::Message("expected 1 valid expression".to_string()))
}
