// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-25

use crate::{
    env::EnvRef,
    error::Error,
    types::{Expr, Number, Result},
};

/// Make a new list with an unbound number of expressions.
pub fn new_list(args: &[Expr], _: EnvRef) -> Result {
    let mut list = Vec::new();
    for arg in args {
        list.push(arg.to_owned());
    }

    Ok(Expr::List(list))
}

/// Append 2 lists together.
pub fn list_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(a), Expr::List(b)] => {
            Ok(Expr::List(a.iter().chain(b.iter()).cloned().collect()))
        }
        _ => Err(Error::Message("expected 2 lists".to_string())),
    }
}

/// Get length of list.
pub fn list_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(l)] => Ok(Expr::Number(Number::from_usize(l.len()))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Get first element from list or pair.
pub fn car(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.as_ref().0.clone()),
        [Expr::List(l)] => Ok(Expr::List(l.iter().cloned().take(1).collect::<Vec<Expr>>())),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Return list without first element or return second element from pair.
pub fn cdr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.as_ref().1.clone()),
        [Expr::List(l)] => Ok(Expr::List(l.iter().cloned().skip(1).collect::<Vec<Expr>>())),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Get second item.
pub fn cadr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(l)] => {
            if l.len() < 2 {
                return Err(Error::Message(
                    "expected list of at least 2 items".to_string(),
                ));
            }

            Ok(Expr::from(l[1].clone()))
        }
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Reverse list.
pub fn list_reverse(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(l)] => Ok(Expr::List(l.iter().cloned().rev().collect::<Vec<Expr>>())),
        _ => Err(Error::Message("expected list".to_string())),
    }
}
