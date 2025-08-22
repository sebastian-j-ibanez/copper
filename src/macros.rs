// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::Env;
use crate::{error::Error, types::Expr, types::Closure};
use crate::parser::eval;

use std::rc::Rc;
use std::cell::RefCell; 


/// Associate a symbol with a value in an environment.
pub fn define(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    let mut iter = args.iter();
    let name = match iter.next() {
        Some(Expr::Symbol(n)) => n.to_owned(),
        _ => return Err(Error::Message("expected symbol name".to_string())),
    };

    let expr = match iter.next() {
        Some(e) => e.clone(),
        _ => return Err(Error::Message("invalid data".to_string())),
    };

    let value = eval(&expr, env.clone())?;

    env.borrow_mut().data.insert(name, value);

    Ok(Expr::Void())
}

/// Lambda macro returns a closure (local environment and a body).
pub fn lambda(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    // Example:
    // (x y) (+ x y)
    // args  function

    let mut iter = args.iter();

    // Get argument symbols.
    let arg_list = match iter.next() {
        Some(Expr::List(l)) => l,
        _ => return Err(Error::Message("malformed lambda".to_string())),
    };

    // Add argument symbols to env.
    let params: Vec<String> = arg_list.iter().map(|arg| {
        if let Expr::Symbol(s) = arg {
            Ok(s.clone())
        } else {
            Err(Error::Message("lambda params must be symbols".to_string()))
        }
    }).collect::<Result<_, _>>()?;

    // Get function.
    let body = match iter.next() {
        Some(e) => e,
        _ => return Err(Error::Message("expected function in lambda".to_string()))
    };

    let closure = Box::new(Closure::init(env.clone(), params, body.clone()));
    Ok(Expr::Closure(closure))
}

pub fn apply_lambda(closure: &Closure, args: Vec<Expr>) -> Result<Expr, Error> {
    if args.len() != closure.parameters.len() {
        return Err(Error::Message("arity mismatch".to_string()));
    }

    // new environment extends the closure’s captured env
    let new_env = Env::local_env(closure.env.clone());

    {
        let mut env_mut = new_env.borrow_mut();
        for (param, arg) in closure.parameters.iter().zip(args.into_iter()) {
            env_mut.data.insert(param.clone(), arg);
        }
    }

   eval(&closure.body, new_env)
}

