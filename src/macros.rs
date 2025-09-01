// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::Env;
use crate::parser::eval;
use crate::{error::Error, types::Closure, types::Expr};

use std::cell::RefCell;
use std::rc::Rc;

/// Associate a symbol with a value in an environment.
pub fn define(args: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match (args.get(0), args.get(1)) {
        (Some(Expr::Symbol(name)), Some(expr)) => {
            let value = eval(&expr, env.clone())?;
            env.borrow_mut().data.insert(name.to_owned(), value);
        }
        (Some(Expr::List(l)), Some(_)) => {
            // Get name + remove it from l.
            let name = match l.first() {
                Some(Expr::Symbol(s)) => s,
                _ => {
                    return Err(Error::Message("ill-formed special form name".to_string()));
                }
            };

            let args_without_name = Expr::List(l.iter().skip(1).cloned().collect());
            let value = lambda(&[args_without_name, args[1].clone()], env.clone())?;
            env.borrow_mut().data.insert(name.to_owned(), value);
        }
        _ => {
            return Err(Error::Message("ill-formed special form".to_string()));
        }
    }
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
        e => return Err(Error::Message(format!("ill-formed lambda: {:?}", e))),
    };

    // Add argument symbols to env.
    let params: Vec<String> = arg_list
        .iter()
        .map(|arg| {
            if let Expr::Symbol(s) = arg {
                Ok(s.clone())
            } else {
                Err(Error::Message(format!(
                    "lambda params must be symbols: {:?}",
                    arg
                )))
            }
        })
        .collect::<Result<_, _>>()?;

    // Get function.
    let body = match iter.next() {
        Some(e) => e,
        _ => return Err(Error::Message("expected lambda body".to_string())),
    };

    let closure = Box::new(Closure::init(env.clone(), params, body.clone()));
    Ok(Expr::Closure(closure))
}

/// Evaluate lambda with arguments.
pub fn apply_lambda(closure: &Closure, args: Vec<Expr>) -> Result<Expr, Error> {
    if args.len() != closure.parameters.len() {
        return Err(Error::Message(format!(
            "wrong number of arguments passed to procedure"
        )));
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

