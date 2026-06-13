// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::{Env, EnvRef, try_borrow_env};
use crate::parser;
use crate::types::{Pair, Vector};
use crate::{error::Error, types::Closure, types::Expr, types::Parameter};
use std::rc::Rc;

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

/// Assigns a value to an existing variable.
pub fn set(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Symbol(s), expr] => {
            let value = parser::eval(expr, env.clone())?;

            let mut borrowed_env = env
                .try_borrow_mut()
                .map_err(|_| Error::new("unable to borrow runtime environment"))?;

            if !borrowed_env.set_expr(s, &value)? {
                return Err(Error::new(&format!("unbound variable: {}", &s)));
            }

            Ok(Expr::Void())
        }
        _ => Err(Error::new("ill-formed special form")),
    }
}

/// Bind arguments and evaluate expressions in a locally scoped environment.
pub fn let_binding(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Pair(bindings), body_expressions @ ..] => {
            let binding_env = Env::local_env(env.clone());
            // Eval bindings in outer env, then insert into new env.
            for binding_pair in bindings.iter() {
                match binding_pair {
                    Expr::Pair(b) => {
                        let items: Vec<Expr> = b.iter().collect();
                        match items.as_slice() {
                            [Expr::Symbol(s), val_expr] => {
                                let value = parser::eval(val_expr, env.clone())?;
                                binding_env
                                    .try_borrow_mut()
                                    .map_err(|_| Error::new("unable to borrow local env"))?
                                    .insert_expr(&s, value);
                            }
                            _ => return Err(Error::new("ill-formed special form")),
                        }
                    }
                    _ => return Err(Error::new("ill-formed special form")),
                }
            }

            // Eval body
            for (i, expr) in body_expressions.iter().enumerate() {
                let value = parser::eval(expr, binding_env.clone())?;

                if i == body_expressions.len() - 1 {
                    return Ok(value);
                }
            }
            Err(Error::new("missing body expression"))
        }
        _ => Err(Error::new("ill-formed special form")),
    }
}

/// Bind arguments and evaluate expressions in a locally scoped environment.
///
/// Differs from `let_bindings` by first evaluating bindings in the outer/global
/// environment, then inserting bindings into a locally scoped environment.
pub fn let_star_binding(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Pair(bindings), body_expressions @ ..] => {
            let mut binding_envs = vec![env.clone()];

            // Eval bindings in outer env, then insert into new env.
            for binding_pair in bindings.iter() {
                match binding_pair {
                    Expr::Pair(binding) => {
                        let items: Vec<Expr> = binding.iter().collect();
                        match items.as_slice() {
                            [Expr::Symbol(s), val_expr] => {
                                let outer_env_ref = binding_envs
                                    .last()
                                    .ok_or(Error::new("expected a local binding env"))?;
                                let value = parser::eval(val_expr, outer_env_ref.clone())?;
                                let local_env = Env::local_env(outer_env_ref.clone());
                                local_env
                                    .try_borrow_mut()
                                    .map_err(|_| Error::new("unable to borrow local env"))?
                                    .insert_expr(&s, value);
                                binding_envs.push(local_env);
                            }
                            _ => return Err(Error::new("ill-formed let binding")),
                        }
                    }
                    _ => return Err(Error::new("ill-formed let binding")),
                }
            }

            // Eval body.
            let last_env = binding_envs
                .last()
                .ok_or(Error::new("expected a local binding env"))?;
            for (i, expr) in body_expressions.iter().enumerate() {
                let value = parser::eval(expr, last_env.clone())?;

                if i == body_expressions.len() - 1 {
                    return Ok(value);
                }
            }
            Err(Error::new("missing body expression"))
        }
        _ => Err(Error::new("ill-formed special form")),
    }
}

/// Bind arguments and evaluate expressions in a locally scoped environment.
///
/// Differs from `let_binding` by temporarily initializing each new variable
/// to `Expr::Null`, before evaluating and assigning binding values. This
/// allows new variables bound in `letrec_binding` to reference other new variables.
pub fn letrec_binding(args: &[Expr], env: EnvRef) -> Result<Expr, Error> {
    match args {
        [Expr::Pair(binding_pairs), body_expressions @ ..] => {
            let binding_env = Env::local_env(env.clone());

            let mut bindings: Vec<(String, Expr)> = Vec::new();

            // Initialize each variable to `Expr::Null`.
            for binding_pair in binding_pairs.clone().iter() {
                match binding_pair {
                    Expr::Pair(b) => {
                        let binding_items: Vec<Expr> = b.iter().collect();
                        match binding_items.as_slice() {
                            [Expr::Symbol(s), value] => {
                                bindings.push((s.clone(), value.clone()));
                                binding_env
                                    .try_borrow_mut()
                                    .map_err(|_| Error::new("unable to borrow local env"))?
                                    .insert_expr(s, Expr::Null)
                            }
                            _ => return Err(Error::new("ill-formed special form")),
                        }
                    }
                    _ => return Err(Error::new("ill-formed special form")),
                }
            }

            // Evaluate and assign binding values in the letrec env so closures
            // capture it (enabling self- and mutual recursion).
            for (name, value_expr) in bindings {
                let value = parser::eval(&value_expr, binding_env.clone())?;
                binding_env
                    .try_borrow_mut()
                    .map_err(|_| Error::new("unable to borrow local env"))?
                    .insert_expr(&name, value);
            }

            // Eval body.
            for (i, expr) in body_expressions.iter().enumerate() {
                let value = parser::eval(expr, binding_env.clone())?;

                if i == body_expressions.len() - 1 {
                    return Ok(value);
                }
            }
            Err(Error::new("missing body expression"))
        }
        _ => Err(Error::new("ill-formed special form")),
    }
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
    let params: Vec<String> = match iter.next() {
        Some(Expr::Null) => vec![],
        Some(Expr::Pair(p)) => p
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
            .collect::<Result<_, _>>()?,
        _ => return Err(Error::Message(format!("ill-formed lambda"))),
    };

    let body_expressions: Vec<Expr> = args[1..].to_vec();
    let closure = Rc::new(Closure::init(env.clone(), params, body_expressions));
    Ok(Expr::Closure(closure))
}

// /// Evaluate lambda with arguments.
pub fn apply_lambda(closure: &Closure, args: Vec<Expr>) -> Result<Expr, Error> {
    if args.len() != closure.parameters.len() {
        return Err(Error::Message(format!(
            "wrong number of arguments passed to procedure"
        )));
    }

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
    let env = try_borrow_env(&env)?;

    if let Some(value) = env.find_value(symbol) {
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
