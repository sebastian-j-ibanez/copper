// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-29

//! Functions related to IO.

use crate::error::Error;
use crate::types::Expr;

/// Display raw expression in stdout.
pub fn display(args: &[Expr]) -> Result<Expr, Error> {
    match args.first() {
        Some(arg) => Ok(arg.to_owned()),
        _ => Err(Error::Message("expected 1 valid expression".to_string())),
    }
}

pub fn newline(_: &[Expr]) -> Result<Expr, Error> {
    Ok(Expr::String("\n".to_string()))
}

pub fn print(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        return Ok(arg.to_owned());
    }
    Err(Error::Message("expected 1 valid expression".to_string()))
}
