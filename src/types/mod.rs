// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Copper data types.

pub mod number;

use crate::env::EnvRef;
use crate::error::Error;
pub(crate) use number::Number;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

pub type Result = std::result::Result<Expr, Error>;
pub type List = Rc<RefCell<Vec<Expr>>>;
pub type Pair = Rc<RefCell<(Expr, Expr)>>;
pub type Procedure = fn(&[Expr], EnvRef) -> Result;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    String(String),
    Char(char),
    Boolean(bool),
    Symbol(String),
    List(List),
    Pair(Pair),
    Void(),
    Procedure(Procedure),
    Closure(Box<Closure>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format_string(s),
            Expr::Char(c) => format_char(c),
            Expr::Boolean(b) => format_boolean(b),
            Expr::Symbol(s) => s.clone(),
            Expr::List(list) => format_list(list, " ", true),
            Expr::Pair(pair) => format_pair(pair),
            Expr::Void() => return Ok(()),
            Expr::Procedure(_) => "#<function {}".to_string(),
            Expr::Closure(_) => "#<procedure {}>".to_string(),
        };
        write!(f, "{}", s)
    }
}

/// Format string into its literal representation.
fn format_string(s: &String) -> String {
    format!("\"{}\"", s)
}

/// Format char into its literal representation.
fn format_char(c: &char) -> String {
    format!("{}{}", "#\\", c)
}

/// Format boolean into its literal representation.
fn format_boolean(b: &bool) -> String {
    match *b {
        true => format!("{}", BOOLEAN_TRUE_STR),
        false => format!("{}", BOOLEAN_FALSE_STR),
    }
}

/// Format list, optional delimeter and parenthesis.
pub fn format_list(list: &List, delim: &str, parenthesis: bool) -> String {
    let items: String = list
        .borrow()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(delim);

    match parenthesis {
        true => format!("({})", items),
        false => items,
    }
}

/// Format pair into literal representation.
pub fn format_pair(pair: &Pair) -> String {
    format!("({} . {})", pair.borrow().0, pair.borrow().1)
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub env: EnvRef,
    pub parameters: Vec<String>,
    pub body: Expr,
}

impl Closure {
    pub fn init(env: EnvRef, parameters: Vec<String>, body: Expr) -> Closure {
        Closure {
            env,
            parameters,
            body,
        }
    }
}
