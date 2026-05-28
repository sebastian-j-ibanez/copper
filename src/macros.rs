// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::{Env, EnvRef};
use crate::parser;
use crate::types::{Pair, Vector};
use crate::{error::Error, types::Closure, types::Expr, types::Parameter};

/// Associate a symbol with a value in an environment.
pub fn define(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Symbol(name), expr] => {
            let value = parser::eval(&expr, env.clone())?;
            env.borrow_mut().data.insert(name.to_owned(), value);
        }
        [Expr::Pair(pair), body_expressions @ ..] => {
            let proc_name = match pair.car() {
                Expr::Symbol(s) => s,
                _ => {
                    return Err(Error::new("ill-formed special form name"));
                }
            };

            // Lambda parameters (cdr) and body expressions.
            let lambda_args = [&[pair.cdr()], body_expressions].concat();
            let value = lambda(&lambda_args, env.clone())?;
            env.borrow_mut().data.insert(proc_name.to_owned(), value);
        }
        _ => {
            return Err(Error::new("ill-formed special form"));
        }
    }
    Ok(Expr::Void())
}

/// Evaluate all arguments sequentially, and return the value of the last expression.
pub fn begin(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    let last_value = args
        .iter()
        .try_fold(None, |_, expr| parser::eval(expr, env.clone()).map(Some))?
        .unwrap_or(Expr::Void());

    Ok(last_value)
}

/// Lambda macro returns a closure (scoped environment and a body).
pub fn lambda(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    // Example:
    // (x y) (+ x y)
    // args  function
    if args.len() < 2 {
        return Err(Error::new("ill-formed lambda"));
    }

    let mut iter = args.iter();

    // Get argument symbols.
    let arg_list = match iter.next() {
        Some(Expr::Pair(p)) => p,
        _ => return Err(Error::Message(format!("ill-formed lambda"))),
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

    let body_expressions: Vec<Expr> = args[1..].to_vec();
    let closure = Box::new(Closure::init(env.clone(), params, body_expressions));
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

    begin(&closure.body, new_env)
}

/// Process literal into expression.
pub fn quote(args: &[Expr], _: EnvRef) -> Result<Expr, Error> {
    match args {
        [expr] => Ok(expr.clone()),
        _ => Err(Error::new("quote expects 0 or 1 arguments")),
    }
}

/// Process literal into expression, replacing comma prefixed symbols
/// with their `env` value.
pub fn quasiquote(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [expr] => unquote(expr, env),
        _ => Err(Error::new("quasiquote expects 1 expression")),
    }
}

/// Replace comma prefixed symbols with their `env` value recursively.
fn unquote(expr: &Expr, env: EnvRef) -> Result<Expr, Error> {
    match expr {
        Expr::Symbol(s) => {
            if s == "," {
                return Err(Error::new("expected symbol after ','"));
            }

            if s.starts_with(",") {
                let value = resolve_unquoted_symbol(s, env)?;
                return Ok(value);
            }

            Ok(expr.clone())
        }
        Expr::Pair(p) if p.is_pair() => {
            let pair = match p.car() {
                // Edgecase where dotted pair has 1 unquoted element. Example: `(,a)
                Expr::Symbol(s) if s == "," => {
                    let car = unquote(&p.cdr(), env)?;
                    Expr::Pair(Pair::cons((car, Expr::Null)))
                }
                _ => {
                    let car = unquote(&p.car(), env.clone())?;
                    let cdr = unquote(&p.cdr(), env)?;
                    Expr::Pair(Pair::cons((car, cdr)))
                }
            };
            Ok(pair)
        }
        Expr::Pair(p) => {
            let mut list = Vec::new();
            for elem in p.iter() {
                match elem {
                    Expr::Pair(elem_pair) => match elem_pair.car() {
                        // Case where pair is like this: `(,example)
                        // The car becomes ',' and the cdr is 'example'.
                        Expr::Symbol(s) if s == "," => {
                            list.push(unquote(&elem_pair.cdr(), env.clone())?);
                        }
                        _ => list.push(unquote(&Expr::Pair(elem_pair), env.clone())?),
                    },
                    Expr::Symbol(s) if s.starts_with(",") => {
                        let value = resolve_unquoted_symbol(&s, env.clone())?;
                        list.push(value);
                    }
                    _ => list.push(elem),
                }
            }

            Ok(Pair::list(&list))
        }
        Expr::Vector(vec) => {
            let elements = vec
                .iter()
                .map(|elem| unquote(&elem, env.clone()))
                .collect::<Result<Vec<Expr>, _>>()?;

            let new_vec = Expr::Vector(Vector::from(&elements));
            Ok(new_vec)
        }
        expr => Ok(expr.clone()),
    }
}

/// Resolve a symbol that starts with a comma.
fn resolve_unquoted_symbol(symbol: &String, env: EnvRef) -> Result<Expr, Error> {
    let symbol = &symbol[1..];
    let env = env
        .try_borrow()
        .map_err(|_| Error::new("unable to borrow reference to runtime environment"))?;

    if let Some(value) = env.find_var(symbol) {
        return Ok(value);
    }

    return Err(Error::new(&format!("unbound variable: {symbol}")));
}

/// If predicate is true evaluate first expression, otherwise evaluate second expression.
pub fn if_statement(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [conditional, first_branch, second_branch] => {
            let cond_result = parser::eval(conditional, env.to_owned())?;
            match cond_result {
                Expr::Boolean(false) => parser::eval(second_branch, env),
                _ => parser::eval(first_branch, env),
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
                        let cond_result = parser::eval(conditional, env.to_owned())?;
                        if let Expr::Boolean(true) = cond_result {
                            return parser::eval(result, env);
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

/// Sets the first element in a list or pair.
pub fn set_car(args: &[Expr], _: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Pair(pair), value] => {
            pair.set_car(value.clone());
            Ok(Expr::Void())
        }
        [_, _] => Err(Error::new("set-car!: expected pair as first argument")),
        _ => Err(Error::new("set-car!: expected 2 arguments")),
    }
}

/// Sets the last element in a list or pair.
pub fn set_cdr(args: &[Expr], _: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Pair(pair), value] => {
            pair.set_cdr(value.clone());
            Ok(Expr::Void())
        }
        [_, _] => Err(Error::new("set-cdr!: expected pair as first argument")),
        _ => Err(Error::new("set-cdr!: expected 2 arguments")),
    }
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
                result = parser::eval(expr, env.clone())?;
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
                let param = parser::eval(param_expr, env.clone())?;
                let Parameter { id, ref converter } = match param {
                    Expr::Parameter(p) => p,
                    _ => return Err(Error::new("parameterize: expected parameter object")),
                };

                // Evaluate the value expression
                let raw_value = parser::eval(val_expr, env.clone())?;

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
        result = parser::eval(expr, param_env.clone())?;
    }

    Ok(result)
}
