// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-25

use std::{cell::RefCell, rc::Rc};

use crate::{
    env::Env,
    error::Error,
    types::{Expr, Number},
};

/// Make a new list with an unbound number of expressions.
pub fn new_list(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    let mut list = Vec::new();
    for arg in args {
        list.push(arg.to_owned());
    }

    Ok(Expr::List(list))
}

/// Construct a new pair with a 'car' and a 'cdr'.
pub fn cons(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [a, Expr::List(b)] => {
            let mut new_pair: Vec<Expr> = b.clone();
            new_pair.insert(0, a.clone());
            Ok(Expr::List(new_pair))
        }
        _ => Err(Error::Message("expected 2 lists".to_string())),
    }
}

/// Append 2 lists together.
pub fn list_append(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::List(a), Expr::List(b)] => {
            Ok(Expr::List(a.iter().chain(b.iter()).cloned().collect()))
        }
        _ => Err(Error::Message("expected 2 lists".to_string())),
    }
}

/// Get length of list.
pub fn list_length(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::List(l)] => Ok(Expr::Number(Number::from_usize(l.len()))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Get first item from list.
pub fn car(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::List(l)] => Ok(Expr::List(l.iter().cloned().take(1).collect::<Vec<Expr>>())),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Get list without first item.
pub fn cdr(args: &[Expr], _: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match args {
        [Expr::List(l)] => Ok(Expr::List(l.iter().cloned().skip(1).collect::<Vec<Expr>>())),
        _ => Err(Error::Message("expected list".to_string())),
    }
}
