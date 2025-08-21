// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-29

use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::error::Error;
use crate::types::{Expr, Number};

pub fn string_length(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::String(s) => Ok(Expr::Number(Number::from_usize(s.len()))),
            _ => Err(Error::Message("expected string".to_string())),
        };
    }

    Err(Error::Message("expected 1 valid expression".to_string()))
}
