// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-21

//! Functions related to math operations.

use crate::env::Env;
use crate::{error::Error, types::Expr};

use std::cell::RefCell;
use std::rc::Rc;
use crate::types::Number;

/// Perform modulo to number.
pub fn modulo(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::Number(a), Expr::Number(b)] => {
            let a = a.clone();
            let b = b.clone();
            Ok(Expr::Number((a % b)?))
        }
        _ => Err(Error::Message("expected exactly 2 arguments".to_string())),
    }
}

/// Apply exponent to number.
pub fn exponent(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::Number(a), Expr::Number(b)] => Ok(Expr::Number(a.pow(b)?)),
        _ => Err(Error::Message("expected exactly 2 arguments".to_string())),
    }
}

/// Get absolute value of number.
pub fn abs(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::Number(n)] => {
            let n = n.clone();
            if n < Number::from_i64(0) {
                return if let Ok(result) = n * Number::from_i64(-1) {
                    Ok(Expr::Number(result))
                } else {
                    Err(Error::Message("".to_string()))
                }
            }
            Ok(Expr::Number(n))
        }
        _ => Err(Error::Message("expected 1 number".to_string())),
    }
}