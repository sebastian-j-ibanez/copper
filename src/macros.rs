// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::{Env, EnvRef};
use crate::parser::eval;
use crate::{error::Error, types::Closure, types::Expr, types::Parameter};

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
                    return Err(Error::new("ill-formed special form name"));
                }
            };

            let args_without_name = pair.cdr();
            let value = lambda(&[args_without_name, expr.clone()], env.clone())?;
            env.borrow_mut().data.insert(name.to_owned(), value);
        }
        _ => {
            return Err(Error::new("ill-formed special form"));
        }
    }
    Ok(Expr::Void())
}

/// Temporarily shadow parameters for the duration of the body evaluation.
/// Syntax: (parameterize ((param value) ...) body ...)
pub fn parameterize(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    // args[0] should be the bindings list: ((param1 val1) (param2 val2) ...)
    // args[1..] are the body expressions
    let (bindings_expr, body) = match args {
        [Expr::Pair(bindings), rest @ ..] if !rest.is_empty() => (bindings, rest),
        [Expr::Null, rest @ ..] if !rest.is_empty() => {
            let mut result = Expr::Void();
            for expr in rest {
                result = eval(expr, env.clone())?;
            }
            return Ok(result);
        }
        _ => return Err(Error::new("parameterize: ill-formed syntax")),
    };

    // Create a new environment scope for the parameterize body
    let param_env = Env::local_env(env.clone());

    // Process each binding: ((param val) ...)
    for binding in bindings_expr.iter() {
        match binding {
            Expr::Pair(pair) if pair.len() == 2 => {
                let items: Vec<Expr> = pair.iter().collect();
                let param_expr = &items[0];
                let val_expr = &items[1];

                // Evaluate the parameter expression to get the Parameter object
                let param = eval(param_expr, env.clone())?;
                let Parameter { id, ref converter } = match param {
                    Expr::Parameter(p) => p,
                    _ => return Err(Error::new("parameterize: expected parameter object")),
                };

                // Evaluate the value expression
                let raw_value = eval(val_expr, env.clone())?;

                // Apply converter if present
                let value = if let Some(conv) = converter {
                    match conv.as_ref() {
                        Expr::Procedure(f) => f(&[raw_value], env.clone())?,
                        Expr::Closure(c) => apply_lambda(c, vec![raw_value])?,
                        _ => return Err(Error::new("invalid converter")),
                    }
                } else {
                    raw_value
                };

                // Store in the new parameterize environment
                param_env.borrow_mut().set_param(&id.to_string(), &value);
            }
            _ => {
                return Err(Error::new(
                    "parameterize: each binding must be (param value)",
                ));
            }
        }
    }

    // Evaluate body expressions in the new environment
    let mut result = Expr::Void();
    for expr in body {
        result = eval(expr, param_env.clone())?;
    }

    Ok(result)
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
                    _ => return Err(Error::new("pair expected")),
                }
            }
        }
        [Expr::Pair(pair), expr] => {
            let new_value = eval(&expr, env_ref.clone())?;
            pair.set_car(new_value.clone());
        }
        [] => return Err(Error::new("expected 2 arguments")),
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
                    _ => return Err(Error::new("expected pair")),
                }
            }
        }
        [Expr::Pair(pair), expr] => {
            let new_value = eval(&expr, env_ref.clone())?;
            pair.set_cdr(new_value.clone());
        }
        [] => return Err(Error::new("expected 2 arguments")),
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
        _ => return Err(Error::new("expected lambda body")),
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
        _ => Err(Error::new("quote expects 1 argument")),
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
        _ => Err(Error::new("ill-formed special form")),
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
