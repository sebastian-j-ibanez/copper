// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Functions that parse text and convert s-expressions to data types.

use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::error::Error;
use crate::macros::{apply_lambda, define, lambda};
use crate::types::{BOOLEAN_FALSE_STR, BOOLEAN_TRUE_STR, Expr, Number};

/// Parse s-expression, evaluate it, and return result.
pub fn parse_and_eval(expr: String, env: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

/// Evaluate an s-expression.
pub fn eval(expr: &Expr, env: Rc<RefCell<Env>>) -> Result<Expr, Error> {
    match expr {
        Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) => Ok(expr.clone()),
        Expr::Symbol(k) => env
            .borrow()
            .find_var(k)
            .ok_or(Error::Message(format!("unbound symbol '{}'", k))),
        Expr::List(list) => {
            let [first, args @ ..] = list.as_slice() else {
                return Err(Error::Message("empty list".to_string()));
            };

            // Check for special forms (like define)
            if let Expr::Symbol(s) = first {
                match s.as_str() {
                    "define" => return define(args, env),
                    "lambda" => return lambda(args, env),
                    _ => {}
                }
            }

            let func_val = eval(first, env.clone())?;

            let arg_vals = args
                .iter()
                .map(|x| eval(x, env.clone()))
                .collect::<Result<Vec<_>, _>>()?;

            match func_val {
                Expr::Func(f) => f(&arg_vals, env),
                Expr::Closure(c) => apply_lambda(&c, arg_vals),
                e => {
                    let msg = format!("not a function: {}", e);
                    Err(Error::Message(msg))
                }
            }
        }
        Expr::Void() => Ok(Expr::Void()),
        _ => Err(Error::Message("unexpected form".to_string())),
    }
}

/// Parse tokenized s-expressions.
pub fn parse(tokens: &[String]) -> Result<(Expr, &[String]), Error> {
    let (token, right_expr) = tokens
        .split_first()
        .ok_or(Error::Message("could not parse first token".to_string()))?;

    match &token[..] {
        "(" => parse_right_expr(right_expr),
        ")" => Err(Error::Message("error: invalid ')'".to_string())),
        _ => Ok((eval_atom(token), right_expr)),
    }
}

/// Recursively parse remaining s-expressions.
pub fn parse_right_expr(tokens: &[String]) -> Result<(Expr, &[String]), Error> {
    let mut expressions: Vec<Expr> = vec![];
    let mut tokens_copy = tokens;
    loop {
        let (car, cdr) = tokens_copy.split_first().ok_or(Error::Message(
            "unable to parse rest of expression".to_string(),
        ))?;
        if car == ")" {
            return Ok((Expr::List(expressions), cdr));
        }
        let (expr, new_copy) = parse(&tokens_copy)?;
        expressions.push(expr);
        tokens_copy = new_copy;
    }
}

/// Create an Expr from a &str.
pub fn eval_atom(token: &str) -> Expr {
    // String
    if token.starts_with('"') && token.ends_with('"') && token.len() >= 2 {
        let inner_string = &token[1..token.len() - 1]; // Remove quotes. 
        return Expr::String(inner_string.to_string());
    }

    // Boolean
    if token == BOOLEAN_TRUE_STR {
        return Expr::Boolean(true);
    } else if token == BOOLEAN_FALSE_STR {
        return Expr::Boolean(false);
    }

    // Number
    if let Ok(num) = Number::from_token(token) {
        return Expr::Number(num);
    }

    return Expr::Symbol(token.to_string());
}

/// Get vec of numbers from an s-expression.
pub fn parse_number_list(expressions: &[Expr]) -> Result<Vec<Number>, Error> {
    expressions.iter().map(|e| parse_number(e)).collect()
}

/// Get a single number from an s-expression.
pub fn parse_number(expr: &Expr) -> Result<Number, Error> {
    match expr {
        Expr::Number(num) => Ok(num.clone()),
        _ => Err(Error::Message("expected a number".to_string())),
    }
}

/// Check if s-expression has been closed with a parenthesis.
pub fn expression_closed(buf: &str) -> bool {
    let expression = buf.trim();
    let mut open_paren = 0;
    let mut close_paren = 0;

    for e in expression.chars() {
        match e {
            '(' => open_paren += 1,
            ')' => close_paren += 1,
            _ => {}
        }
    }

    (open_paren == close_paren) || (!expression.starts_with('(') && !expression.ends_with(')'))
}

/// Tokenize a string s-expression.
pub fn tokenize(expression: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            // Skip whitespace.
            ' ' | '\t' | '\r' | '\n' => {
                i += 1;
            }
            '(' | ')' => {
                tokens.push(chars[i].to_string());
                i += 1;
            }
            '"' => {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                let string: String = chars[start..i].iter().collect();
                tokens.push(string);
            }
            _ => {
                let start = i;
                while i < chars.len()
                    && !chars[i].is_whitespace()
                    && chars[i] != '('
                    && chars[i] != ')'
                {
                    i += 1;
                }
                let atom: String = chars[start..i].iter().collect();
                tokens.push(atom);
            }
        }
    }

    tokens
}
