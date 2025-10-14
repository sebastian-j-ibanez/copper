// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-29

use crate::env::EnvRef;
use crate::error::Error;
use crate::types::{Expr, Number, Result};

pub fn str_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(a), Expr::String(b)] => {
            let c = a.clone() + b;
            Ok(Expr::String(c))
        }
        _ => Err(Error::Message(format!("expected 2 strings"))),
    }
}

pub fn str_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Number(Number::from_usize(s.len()))),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

pub fn new_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [] => Ok(Expr::String(String::new())),
        [Expr::Char(c)] => Ok(Expr::String(String::from(*c))),
        _ => Err(Error::Message("expected character".to_string())),
    }
}
