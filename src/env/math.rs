// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-21

//! Functions related to math operations.

use crate::env::EnvRef;
use crate::error::Error;
use crate::types::{Expr, Number, Result};

/// Perform modulo to number.
pub fn modulo(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(a), Expr::Number(b)] => {
            let a = a.clone();
            let b = b.clone();
            Ok(Expr::Number((a % b)?))
        }
        _ => Err(Error::Message("expected 2 numbers".to_string())),
    }
}

/// Apply exponent to number.
pub fn exponent(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(a), Expr::Number(b)] => Ok(Expr::Number(a.pow(b)?)),
        _ => Err(Error::Message("expected 2 numbers".to_string())),
    }
}

/// Get absolute value of number.
pub fn abs(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(n)] => {
            let n = n.clone();
            if n < Number::from_i64(0) {
                return if let Ok(result) = n.clone() * Number::from_i64(-1) {
                    Ok(Expr::Number(result))
                } else {
                    Err(Error::Message(format!(
                        "unable to get absolute value from n: {}",
                        n
                    )))
                };
            }
            Ok(Expr::Number(n))
        }
        _ => Err(Error::Message("expected 1 number".to_string())),
    }
}

/// Round number up to the nearest integer.
pub fn ceil(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => {
            Err(Error::Message("unable to round complex number".to_string()))
        }
        [Expr::Number(n)] => {
            if let Some(result) = n.to_f64() {
                return Ok(Expr::Number(Number::from_f64(result.ceil())));
            }
            Err(Error::Message(
                "unable to convert number to float".to_string(),
            ))
        }
        _ => Err(Error::Message("expected real number".to_string())),
    }
}

/// Round number down to the nearest integer.
pub fn floor(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => {
            Err(Error::Message("unable to round complex number".to_string()))
        }
        [Expr::Number(n)] => {
            if let Some(result) = n.to_f64() {
                return Ok(Expr::Number(Number::from_f64(result.floor())));
            }
            Err(Error::Message(
                "unable to convert number to float".to_string(),
            ))
        }
        _ => Err(Error::Message("expected real number".to_string())),
    }
}

/// Return smallest real number from arguments.
pub fn min(args: &[Expr], _: EnvRef) -> Result {
    if args.is_empty() {
        return Err(Error::Message("expected real numbers".to_string()));
    }

    let mut min: Option<Number> = None;

    for arg in args {
        match arg {
            Expr::Number(current) => match current {
                Number::Complex(_) => {
                    return Err(Error::Message("expected real numbers".to_string()));
                }
                _ => match min {
                    None => min = Some(current.clone()),
                    Some(ref current_min) => {
                        if current < current_min {
                            min = Some(current.clone());
                        }
                    }
                },
            },
            _ => {
                return Err(Error::Message("expected real numbers".to_string()));
            }
        }
    }

    Ok(Expr::Number(min.unwrap()))
}

/// Return largest real number from arguments.
pub fn max(args: &[Expr], _: EnvRef) -> Result {
    if args.is_empty() {
        return Err(Error::Message("expected real numbers".to_string()));
    }

    let mut min: Option<Number> = None;

    for arg in args {
        match arg {
            Expr::Number(current) => match current {
                Number::Complex(_) => {
                    return Err(Error::Message("expected real numbers".to_string()));
                }
                _ => match min {
                    None => min = Some(current.clone()),
                    Some(ref current_min) => {
                        if current > current_min {
                            min = Some(current.clone());
                        }
                    }
                },
            },
            _ => {
                return Err(Error::Message("expected real numbers".to_string()));
            }
        }
    }

    Ok(Expr::Number(min.unwrap()))
}
