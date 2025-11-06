// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-11-06

//! Standard library functions relating to booleans.

use crate::{env::EnvRef, types::Expr, types::Result};

/// Returns the opposite value of a `bool`.
pub fn not(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Boolean(false)] => Ok(Expr::Boolean(true)),
        _ => Ok(Expr::Boolean(false)),
    }
}

/// Returns `true` if any arguments are `false`.
pub fn and(args: &[Expr], _: EnvRef) -> Result {
    let contains_false = args.iter().all(|arg| !matches!(arg, Expr::Boolean(false)));
    Ok(Expr::Boolean(contains_false))
}

/// Returns `false` if any arguments are `true`.
pub fn or(args: &[Expr], _: EnvRef) -> Result {
    let contains_true = args.iter().all(|arg| !matches!(arg, Expr::Boolean(true)));
    Ok(Expr::Boolean(contains_true))
}
