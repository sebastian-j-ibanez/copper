// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Functions that parse text and convert s-expressions to data types.

use std::rc::Rc;

use crate::env::EnvRef;
use crate::error::Error;
use crate::macros;
use crate::types::{BOOLEAN_FALSE_STR, BOOLEAN_TRUE_STR, Expr, Number, Pair, Parameter};

type Cont = Option<Rc<Node>>;

#[derive(Debug, Clone)]
enum State {
    Eval(Expr, EnvRef, Cont),
    Return(Expr, Cont),
}

#[derive(Debug, Clone)]
enum Node {
    Application {
        done: Vec<Expr>,
        pending: Vec<Expr>,
        env: EnvRef,
        next: Cont,
    },
    If {
        if_branch: Expr,
        else_branch: Option<Expr>,
        env: EnvRef,
        next: Cont,
    },
}

pub fn eval(expr: &Expr, env: EnvRef) -> Result<Expr, Error> {
    let mut state = State::Eval(expr.clone(), env, None);
    loop {
        state = match state {
            State::Eval(expr, env, cont) => match expr {
                Expr::Number(_)
                | Expr::String(_)
                | Expr::Char(_)
                | Expr::Boolean(_)
                | Expr::Vector(_)
                | Expr::ByteVector(_)
                | Expr::Procedure(_)
                | Expr::Closure(_)
                | Expr::Port(_)
                | Expr::Parameter(_) => State::Return(expr.clone(), cont),
                Expr::Symbol(k) => {
                    let value = env
                        .borrow()
                        .find_value(&k)
                        .ok_or(Error::Message(format!("unbound symbol '{}'", k)))?;
                    State::Return(value, cont)
                }
                Expr::Pair(pair) => {
                    let list_elements: Vec<Expr> = pair.iter().collect();

                    let [first, args @ ..] = list_elements.as_slice() else {
                        return Ok(Expr::Null);
                    };

                    let apply_frame = Node::Application {
                        done: vec![],
                        pending: args.to_vec(),
                        env: env.clone(),
                        next: cont,
                    };

                    State::Eval(first.clone(), env.clone(), Some(Rc::new(apply_frame)))
                }
                Expr::Null => State::Return(Expr::Null, cont),
                Expr::Eof => State::Return(Expr::Eof, cont),
                Expr::Void() => State::Return(Expr::Void(), cont),
            },
            State::Return(value, Some(cont)) => match cont.as_ref() {
                Node::Application {
                    done,
                    pending,
                    env,
                    next,
                } => {
                    let mut updated_done = done.clone();
                    updated_done.push(value);

                    match pending.as_slice() {
                        [expr, updated_pending @ ..] => {
                            let new_frame = Node::Application {
                                done: updated_done,
                                pending: updated_pending.to_vec(),
                                env: env.clone(),
                                next: next.clone(),
                            };
                            State::Eval(expr.clone(), env.clone(), Some(Rc::new(new_frame)))
                        }
                        _ => {
                            let (first, args) = match updated_done.as_slice() {
                                [first, args @ ..] => (first, args),
                                _ => return Err(Error::new("empty application")),
                            };

                            let result = match first {
                                Expr::Procedure(p) => p(args, env.clone())?,
                                e => {
                                    let msg = format!("not a function: {}", e);
                                    dbg!(updated_done);
                                    return Err(Error::Message(msg));
                                }
                            };

                            State::Return(result, next.clone())
                        }
                    }
                }
                Node::If {
                    if_branch: _,
                    else_branch: _,
                    env: _,
                    next: _,
                } => State::Return(value, None),
            },
            State::Return(value, None) => return Ok(value),
        };
    }
}

/// Parse s-expression, evaluate it, and return result.
pub fn parse_and_eval(expr: String, env: EnvRef) -> Result<Expr, Error> {
    let tokens = tokenize(expr);
    let (parsed_exp, _) = parse(&tokens)?;
    let evaled_exp = eval(&parsed_exp, env.clone())?;
    Ok(evaled_exp)
}

/// Evaluate an s-expression.
pub fn old_eval(expr: &Expr, env: EnvRef) -> Result<Expr, Error> {
    match expr {
        Expr::Number(_)
        | Expr::String(_)
        | Expr::Char(_)
        | Expr::Boolean(_)
        | Expr::Vector(_)
        | Expr::ByteVector(_)
        | Expr::Procedure(_)
        | Expr::Closure(_)
        | Expr::Port(_)
        | Expr::Parameter(_) => Ok(expr.clone()),
        Expr::Symbol(k) => env
            .borrow()
            .find_value(k)
            .ok_or(Error::Message(format!("unbound symbol '{}'", k))),
        Expr::Pair(pair) => {
            let list_elements: Vec<Expr> = pair.iter().collect();

            // Return empty list if there are no args.
            let [first, args @ ..] = list_elements.as_slice() else {
                return Ok(Expr::Null);
            };

            // Check for special forms (like define).
            if let Expr::Symbol(s) = first {
                match s.as_str() {
                    "define" => return macros::define(args, env),
                    "set!" => return macros::set(args, env),
                    "begin" => return macros::begin(args, env),
                    "lambda" => return macros::lambda(args, env),
                    "quote" => return macros::quote(args, env),
                    "quasiquote" => return macros::quasiquote(args, env),
                    "if" => return macros::if_statement(args, env),
                    "cond" => return macros::cond(args, env),
                    "and" => return macros::and(args, env),
                    "or" => return macros::or(args, env),
                    "when" => return macros::when(args, env),
                    "unless" => return macros::unless(args, env),
                    "parameterize" => return macros::parameterize(args, env),
                    "let" => return macros::let_binding(args, env),
                    "let*" => return macros::let_star_binding(args, env),
                    "letrec" | "letrec*" => return macros::letrec_binding(args, env),
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
                Expr::Closure(c) => macros::apply_lambda(&c, arg_vals),
                Expr::Parameter(p) => apply_parameter(&p, arg_vals, env),
                e => {
                    let msg = format!("not a function: {}", e);
                    Err(Error::Message(msg))
                }
            }
        }
        Expr::Void() => Ok(Expr::Void()),
        Expr::Eof => Ok(Expr::Eof),
        Expr::Null => Ok(Expr::Null),
    }
}

/// Apply a parameter object.
/// With no args: returns current value
/// With one arg: sets new value (through converter if present)
fn apply_parameter(param: &Parameter, args: Vec<Expr>, env: EnvRef) -> Result<Expr, Error> {
    let key = param.id.to_string();

    match args.as_slice() {
        // Get current value
        [] => env
            .borrow()
            .find_param_id(&key)
            .ok_or_else(|| Error::Message(format!("parameter {} not initialized", param.id))),
        // Set new value
        [new_value] => {
            let value_to_store = if let Some(ref converter) = param.converter {
                // Apply converter
                match converter.as_ref() {
                    Expr::Procedure(f) => f(&[new_value.clone()], env.clone())?,
                    Expr::Closure(c) => macros::apply_lambda(c, vec![new_value.clone()])?,
                    _ => return Err(Error::new("invalid converter")),
                }
            } else {
                new_value.clone()
            };

            // Set in the current environment.
            env.borrow_mut().set_param(&key, &value_to_store);
            Ok(Expr::Void())
        }
        _ => Err(Error::new("parameter: expected 0 or 1 arguments")),
    }
}

/// Parse tokenized s-expressions.
pub fn parse(tokens: &[String]) -> Result<(Expr, &[String]), Error> {
    if tokens.is_empty() {
        return Ok((Expr::Void(), &[]));
    }

    let (token, right_expr) = tokens
        .split_first()
        .ok_or(Error::new("could not parse first token"))?;

    match &token[..] {
        "(" => parse_right_expr(right_expr),
        ")" => Err(Error::new("invalid ')'")),
        "'" => {
            let (quoted_expr, remaining) = parse(right_expr)?;
            let slice = vec![Expr::Symbol("quote".to_string()), quoted_expr];
            Ok((Pair::list(slice.as_slice()), remaining))
        }
        "`" => {
            let (quasiquoted_expr, remaining) = parse(right_expr)?;
            let slice = vec![Expr::Symbol("quasiquote".to_string()), quasiquoted_expr];
            Ok((Pair::list(slice.as_slice()), remaining))
        }
        "#(" => {
            let (vector_expr, remaining) = parse_literal(right_expr, "vector".to_string())?;
            Ok((vector_expr, remaining))
        }
        "#u8(" => {
            let (bytevector_expr, remaining) = parse_literal(right_expr, "bytevector".to_string())?;
            Ok((bytevector_expr, remaining))
        }
        "#!eof" => Ok((Expr::Eof, right_expr)),
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
        let (car, cdr) = tokens_copy
            .split_first()
            .ok_or(Error::new("unable to parse rest of expression"))?;
        if car == ")" {
            return Ok((Pair::list(expressions.as_slice()), cdr));
        }
        let (expr, new_copy) = parse(&tokens_copy)?;
        expressions.push(expr);
        tokens_copy = new_copy;
    }
}

/// Parse a literal. Primarily used to parse `Vector` and `ByteVector` literals.
pub fn parse_literal(tokens: &[String], constructor: String) -> Result<(Expr, &[String]), Error> {
    let mut expressions: Vec<Expr> = vec![];
    let mut tokens_copy = tokens;
    loop {
        let (car, cdr) = tokens_copy
            .split_first()
            .ok_or(Error::new("unable to parse literal"))?;
        if car == ")" {
            let mut vector_form = vec![Expr::Symbol(constructor.to_string())];
            vector_form.extend(expressions);
            return Ok((Pair::list(vector_form.as_slice()), cdr));
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
        _ => Err(Error::new("expected a number")),
    }
}

/// Tokenize a string s-expression.
pub fn tokenize(expression: String) -> Vec<String> {
    let chars: Vec<char> = expression.chars().collect();
    let mut tokens: Vec<String> = Vec::new();

    // Ignore line if it's a comment.
    if chars.starts_with(&[';']) {
        return tokens;
    }

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
            '`' => {
                tokens.push("`".to_string());
                i += 1;
            }
            '#' => {
                // Vector literal: '#('.
                if i + 1 < chars.len() && chars[i + 1] == '(' {
                    tokens.push("#(".to_string());
                    i += 2;
                } else if i + 3 < chars.len() && chars[i + 1..i + 4] == ['u', '8', '('] {
                    tokens.push("#u8(".to_string());
                    i += 4;
                } else if i + 4 < chars.len() && chars[i + 1..i + 5] == ['!', 'e', 'o', 'f'] {
                    tokens.push("#!eof".to_string());
                    i += 5;
                } else {
                    // Atom: '#\char', '#t', or '#f'.
                    let start = i;
                    while i < chars.len() && !is_delimiter(chars[i]) {
                        i += 1;
                    }
                    let atom: String = chars[start..i].iter().collect();
                    tokens.push(atom);
                }
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

    for line in expression
        .lines()
        .filter(|l| !l.trim_start().starts_with(';'))
    {
        for e in line.chars() {
            match e {
                '(' => open_paren += 1,
                ')' => close_paren += 1,
                _ => {}
            }
        }
    }

    // Not a symbolic expression. Covers edge case when an atom contains parentheses.
    // For example, "example string (with parentheses)".
    let not_an_expression = !expression.starts_with('(') && !expression.ends_with(')');
    let paren_are_equal = open_paren == close_paren;
    not_an_expression || paren_are_equal
}
