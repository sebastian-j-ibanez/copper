// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Functions for R7RS predicates.

use crate::env::EnvRef;
use crate::error::Error;
use crate::parser::parse_number;
use crate::types::number::IntVariant::Small;
use crate::types::{Expr, Number, Result};

/// Returns true if arg is a symbol.
pub fn is_symbol(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Symbol(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a string.
pub fn is_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a character.
pub fn is_char(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Char(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if char is alphabetic.
pub fn is_char_alphabetic(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_alphabetic())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is numeric.
pub fn is_char_numeric(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_numeric())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is whitespace.
pub fn is_char_whitespace(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_whitespace())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is uppercase.
pub fn is_char_uppercase(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_uppercase())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is lowercase.
pub fn is_char_lowercase(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_lowercase())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if arg is a boolean.
pub fn is_boolean(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Boolean(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a list.
pub fn is_list(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a procedure.
pub fn is_procedure(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Procedure(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

// Numbers

/// Returns true if arg is a number.
pub fn is_number(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a real number.
pub fn is_real(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Float(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a rational number.
pub fn is_rational(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Rational(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a complex number.
pub fn is_complex(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is an integer.
pub fn is_integer(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Int(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if number is even.
pub fn is_even(args: &[Expr], _: EnvRef) -> Result {
    args.first()
        .ok_or_else(|| Error::Message("expected one argument".to_string()))
        .and_then(|arg| parse_number(arg))
        .and_then(|num| num % Number::Int(Small(2)))
        .map(|result| Expr::Boolean(result == Number::Int(Small(0))))
}

/// Returns true if number is odd.
pub fn is_odd(args: &[Expr], _: EnvRef) -> Result {
    args.first()
        .ok_or_else(|| Error::Message("expected one argument".to_string()))
        .and_then(|arg| parse_number(arg))
        .and_then(|num| num % Number::Int(Small(2)))
        .map(|result| Expr::Boolean(result == Number::Int(Small(1))))
}
