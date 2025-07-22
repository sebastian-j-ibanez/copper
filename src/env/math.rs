// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-21

//! Functions related to math operations.

use crate::{error::Error, types::{Expr}};

pub fn modulo(args: &[Expr]) -> Result<Expr, Error> {
    match args {
        [Expr::Number(a), Expr::Number(b)] => {
            let a = a.clone();
            let b = b.clone();
            Ok(Expr::Number((a % b)?))
        },
        _ => Err(Error::Message("expected exactly 2 arguments".to_string()))
    }
}
