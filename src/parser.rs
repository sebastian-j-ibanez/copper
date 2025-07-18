// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

use crate::env::Env;
use crate::error::Error;
use crate::types::{Expr, Number};

/// Parse s-expression, evaluate it, return result.
pub fn parse_eval(expr: String, env: &mut Env) -> Result<Expr, Error> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

/// Evaluate an s-expression.
pub fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, Error> {
    match expr {
        Expr::Number(_a) => Ok(expr.clone()),
        Expr::Symbol(k) => env
            .data
            .get(k)
            .ok_or(Error::Message(format!("unexpected symbol '{}'", k)))
            .map(|x| x.clone()),
        Expr::String(s) => Ok(Expr::String(s.clone())),
        Expr::Boolean(b) => Ok(Expr::Boolean(*b)),
        Expr::List(list) => {
            let first_form = list
                .first()
                .ok_or(Error::Message("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                Expr::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<Expr>, Error>>();
                    f(&args_eval?)
                }
                _ => Err(Error::Message("first form must be a function".to_string())),
            }
        }
        Expr::Func(_) => Err(Error::Message("unexpected form".to_string())),
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
    if token.starts_with('"') && token.ends_with('"') && token.len() >= 2 {
        let inner_string = &token[1..token.len() - 1];
        return Expr::String(inner_string.to_string());
    }

    match Number::from_token(token) {
        Ok(num) => Expr::Number(num),
        Err(_) => Expr::Symbol(token.to_string()),
    }
}

pub fn parse_number_list(expressions: &[Expr]) -> Result<Vec<Number>, Error> {
    expressions.iter().map(|e| parse_number(e)).collect()
}

pub fn parse_number(expr: &Expr) -> Result<Number, Error> {
    match expr {
        Expr::Number(num) => Ok(num.clone()),
        _ => Err(Error::Message("expected a number".to_string())),
    }
}

/// Tokenize a string s-expression.
pub fn tokenize(expression: String) -> Vec<String> {
    expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
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
