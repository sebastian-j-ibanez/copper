// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Functions that parse text and convert s-expressions to data types.

use crate::env::EnvRef;
use crate::error::Error;
use crate::macros::{apply_lambda, define, if_statement, lambda, quote};
use crate::types::{Expr, Number, BOOLEAN_FALSE_STR, BOOLEAN_TRUE_STR};

/// Parse s-expression, evaluate it, and return result.
pub fn parse_and_eval(expr: String, env: EnvRef) -> Result<Expr, Error> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

/// Evaluate an s-expression.
pub fn eval(expr: &Expr, env: EnvRef) -> Result<Expr, Error> {
    match expr {
        Expr::Number(_) | Expr::String(_) | Expr::Char(_) | Expr::Boolean(_) => Ok(expr.clone()),
        Expr::Symbol(k) => env
            .borrow()
            .find_var(k)
            .ok_or(Error::Message(format!("unbound symbol '{}'", k))),
        Expr::List(list) => {
            // Return empty list if there are no args.
            let [first, args @ ..] = list.as_slice() else {
                return Ok(Expr::List(vec![Expr::Void()]));
            };

            // Check for special forms (like define)
            if let Expr::Symbol(s) = first {
                match s.as_str() {
                    "define" => return define(args, env),
                    "lambda" => return lambda(args, env),
                    "quote" => return quote(args, env),
                    "if" => return if_statement(args, env),
                    _ => {}
                }
            }

            let func_val = eval(first, env.clone())?;

            let arg_vals = args
                .iter()
                .map(|x| eval(x, env.clone()))
                .collect::<Result<Vec<_>, _>>()?;

            match func_val {
                Expr::Procedure(f) => f(&arg_vals, env),
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
    // If `tokens` is empty, return void.
    if tokens.is_empty() {
        return Ok((Expr::Void(), &[]));
    }

    let (token, right_expr) = tokens
        .split_first()
        .ok_or(Error::Message("could not parse first token".to_string()))?;

    match &token[..] {
        "(" => parse_right_expr(right_expr),
        ")" => Err(Error::Message("invalid ')'".to_string())),
        "'" => {
            let (quoted_expr, remaining) = parse(right_expr)?;
            Ok((
                Expr::List(vec![Expr::Symbol("quote".to_string()), quoted_expr]),
                remaining,
            ))
        }
        _ => match eval_atom(token) {
            Ok(atom) => Ok((atom, right_expr)),
            Err(e) => Err(e),
        },
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

const CHARACTER_ALIASES: &[(&str, char)] = &[
    ("alarm", '\u{0007}'),
    ("backspace", '\u{0008}'),
    ("delete", '\u{007F}'),
    ("escape", '\u{001B}'),
    ("newline", '\u{000A}'),
    ("null", '\u{0000}'),
    ("return", '\u{000D}'),
    ("space", '\u{0020}'),
    ("tab", '\u{0009}'),
];

/// Create an Expr from a &str.
pub fn eval_atom(token: &str) -> crate::types::Result {
    // String
    if token.starts_with('"') && token.ends_with('"') && token.len() >= 2 {
        let inner_string = &token[1..token.len() - 1];
        return Ok(Expr::String(inner_string.to_string()));
    }

    // Char
    let char_delim = "#\\";
    if token.starts_with(char_delim) && token.len() > char_delim.len() {
        let literal = &token[char_delim.len()..];
        // #\x[hex value] (example: '#\x123')
        if let Some(hex_str) = literal.strip_prefix('x') {
            let codepoint = u32::from_str_radix(hex_str, 16)
                .map_err(|_| Error::Message(format!("invalid hex value: {hex_str}")))?;
            return char::from_u32(codepoint)
                .map(Expr::Char)
                .ok_or_else(|| Error::Message(format!("character out of range: {hex_str}")));
        }

        // #\[character name] (example: '#\space')
        let mut chars = literal.chars();
        if let (Some(c), None) = (chars.next(), chars.next()) {
            return Ok(Expr::Char(c));
        }

        // #\[character] (example: '#\a')
        return CHARACTER_ALIASES
            .iter()
            .find_map(|(name, ch)| literal.starts_with(name).then_some(Expr::Char(*ch)))
            .ok_or_else(|| Error::Message(format!("invalid '#\\': {}", literal)));
    }

    // Boolean
    if token == BOOLEAN_TRUE_STR {
        return Ok(Expr::Boolean(true));
    } else if token == BOOLEAN_FALSE_STR {
        return Ok(Expr::Boolean(false));
    }

    // Number
    if let Ok(num) = Number::from_token(token) {
        return Ok(Expr::Number(num));
    }

    return Ok(Expr::Symbol(token.to_string()));
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

/// Tokenize a string s-expression.
pub fn tokenize(expression: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0;
    let is_delimiter = |c: char| c.is_whitespace() || c == '(' || c == ')' || c == '\'';
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
            '\'' => {
                tokens.push("'".to_string());
                i += 1;
            }
            _ => {
                let start = i;
                while i < chars.len() && !is_delimiter(chars[i]) {
                    i += 1;
                }
                let atom: String = chars[start..i].iter().collect();
                tokens.push(atom);
            }
        }
    }

    tokens
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

    // Not a symbolic expression. Covers edge case when an atom contains parentheses.
    // For example, "example string (with parentheses)".
    let not_an_expression = !expression.starts_with('(') && !expression.ends_with(')');
    let paren_are_equal = open_paren == close_paren;
    not_an_expression || paren_are_equal
}
