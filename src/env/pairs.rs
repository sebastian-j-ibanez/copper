// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-25

use crate::{
    env::EnvRef,
    error::Error,
    types::{Expr, Result},
};

/// Construct a new pair from 2 expressions.
pub fn cons(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [a, Expr::List(list)] => {
            let new_list = std::iter::once(a.clone())
                .chain(list.iter().cloned())
                .collect();
            Ok(Expr::List(new_list))
        }
        [a, b] => Ok(Expr::Pair(Box::new((a.clone(), b.clone())))),
        _ => Err(Error::Message("expected 2 arguments".to_string())),
    }
}
