// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-10-31

//! Standard library functions to convert between data types.

use crate::env::EnvRef;
use crate::error::Error;
use crate::types::{Expr, Number, Result};

// Convert a string into a symbol.
pub fn symbol_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Symbol(s)] => Ok(Expr::String(s.to_owned())),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

// Convert a string into a symbol.
pub fn string_to_symbol(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Symbol(s.to_owned())),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

// Convert a string into a number.
pub fn string_to_num(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(num_str)] => match Number::from_token(&num_str) {
            Ok(n) => Ok(Expr::Number(n)),
            Err(e) => Err(e),
        },
        _ => Err(Error::Message("expected string".to_string())),
    }
}

// Convert a number into a string.
pub fn num_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(num)] => Ok(Expr::String(String::from(num.to_string()))),
        _ => Err(Error::Message("expected string".to_string())),
    }
}
