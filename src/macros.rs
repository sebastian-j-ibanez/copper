// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::{Env, EnvRef};
use crate::parser::eval;
use crate::{error::Error, types::Closure, types::Expr};

/// Associate a symbol with a value in an environment.
pub fn define(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Symbol(name), expr] => {
            let value = eval(&expr, env.clone())?;
            env.borrow_mut().data.insert(name.to_owned(), value);
        }
        [Expr::Pair(pair), expr] => {
            let name = match pair.get(0) {
                Some(Expr::Symbol(s)) => s,
                _ => {
                    return Err(Error::Message("ill-formed special form name".to_string()));
                }
            };

            let args_without_name = pair.cdr();
            let value = lambda(&[args_without_name, expr.clone()], env.clone())?;
            env.borrow_mut().data.insert(name.to_owned(), value);
        }
        _ => {
            return Err(Error::Message("ill-formed special form".to_string()));
        }
    }
    Ok(Expr::Void())
}

/// Sets the first element in a list or pair.
pub fn set_car(args: &[Expr], env_ref: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Symbol(name), expr] => {
            let env = env_ref.borrow_mut();
            if let Some(value) = env.find_var(name) {
                match value {
                    Expr::Pair(pair) => {
                        let new_value = eval(&expr, env_ref.clone())?;
                        pair.set_car(new_value.clone());
                    }
                    _ => return Err(Error::Message("pair expected".to_string())),
                }
            }
        }
        [Expr::Pair(pair), expr] => {
            let new_value = eval(&expr, env_ref.clone())?;
            pair.set_car(new_value.clone());
        }
        [] => return Err(Error::Message("expected 2 arguments".to_string())),
        _ => {}
    }

    Ok(Expr::Void())
}

/// Sets the last element in a list or pair.
pub fn set_cdr(args: &[Expr], env_ref: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Symbol(name), expr] => {
            let env = env_ref.borrow_mut();
            if let Some(value) = env.find_var(name) {
                match value {
                    Expr::Pair(pair) => {
                        let new_value = eval(&expr, env_ref.clone())?;
                        pair.set_cdr(new_value.clone());
                    }
                    _ => return Err(Error::Message("expected pair".to_string())),
                }
            }
        }
        [Expr::Pair(pair), expr] => {
            let new_value = eval(&expr, env_ref.clone())?;
            pair.set_cdr(new_value.clone());
        }
        [] => return Err(Error::Message("expected 2 arguments".to_string())),
        _ => {}
    }

    Ok(Expr::Void())
}

/// Lambda macro returns a closure (scoped environment and a body).
pub fn lambda(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    // Example:
    // (x y) (+ x y)
    // args  function

    let mut iter = args.iter();

    // Get argument symbols.
    let arg_list = match iter.next() {
        Some(Expr::Pair(p)) => p,
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

// /// Evaluate lambda with arguments.
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

/// Process literal into expression.
pub fn quote(args: &[Expr], _: EnvRef) -> Result<Expr, Error> {
    match args {
        [expr] => Ok(expr.clone()),
        _ => Err(Error::Message("quote expects 1 argument".to_string())),
    }
}

/// If predicate is true evaluate first expression, otherwise evaluate second expression.
pub fn if_statement(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [conditional, first_branch, second_branch] => {
            let cond_result = eval(conditional, env.to_owned())?;
            match cond_result {
                Expr::Boolean(false) => eval(second_branch, env),
                _ => eval(first_branch, env),
            }
        }
        _ => Err(Error::Message("ill-formed special form".to_string())),
    }
}

pub fn cond(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    for arg in args {
        match arg {
            Expr::Pair(pair) => {
                let collected_args = pair.iter().collect::<Vec<Expr>>();
                match collected_args.as_slice() {
                    [conditional, result] => {
                        let cond_result = eval(conditional, env.to_owned())?;
                        if let Expr::Boolean(true) = cond_result {
                            return eval(result, env);
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    Ok(Expr::Void())
}
