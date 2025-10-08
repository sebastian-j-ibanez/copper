// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Functions for R7RS predicates.

use crate::env::EnvRef;
use crate::error::Error;
use crate::parser::parse_number;
use crate::types::number::IntVariant::Small;
use crate::types::{Expr, Number, Result};

pub fn is_number(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Number(_) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_real(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Number(Number::Float(_)) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_rational(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Number(Number::Rational(_)) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_complex(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Number(Number::Complex(_)) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_integer(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Number(Number::Int(_)) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_string(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::String(_) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_boolean(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Boolean(_) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_list(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::List(_) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_procedure(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Procedure(_) => Ok(Expr::Boolean(true)),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

pub fn is_even(args: &[Expr], _: EnvRef) -> Result {
    args.first()
        .ok_or_else(|| Error::Message("expected one argument".to_string()))
        .and_then(|arg| parse_number(arg))
        .and_then(|num| num % Number::Int(Small(2)))
        .map(|result| Expr::Boolean(result == Number::Int(Small(0))))
}

pub fn is_odd(args: &[Expr], _: EnvRef) -> Result {
    args.first()
        .ok_or_else(|| Error::Message("expected one argument".to_string()))
        .and_then(|arg| parse_number(arg))
        .and_then(|num| num % Number::Int(Small(2)))
        .map(|result| Expr::Boolean(result == Number::Int(Small(1))))
}
