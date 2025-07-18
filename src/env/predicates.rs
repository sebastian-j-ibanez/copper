// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

use crate::error::Error;
use crate::types::{Expr, Number};

pub fn is_number(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Number(_) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_real(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Number(Number::Real(_)) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_rational(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Number(Number::Rational(_)) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_complex(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Number(Number::Complex(_)) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_integer(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Number(Number::Integer(_)) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_string(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(_) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_boolean(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Boolean(_) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_list(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::List(_) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_procedure(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(arg) = args.first() {
        match arg {
            Expr::Func(_) => return Ok(Expr::Boolean(true)),
            _ => return Ok(Expr::Boolean(false)),
        }
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}
