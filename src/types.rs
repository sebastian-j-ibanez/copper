// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Copper data types.

pub mod number;

use crate::error::Error;
pub(crate) use number::Number;
use std::fmt;

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    String(String),
    Boolean(bool),
    Symbol(String),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, Error>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format_string(s),
            Expr::Boolean(b) => format_boolean(b),
            Expr::Symbol(s) => s.clone(),
            Expr::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            Expr::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", s)
    }
}

fn format_string(s: &String) -> String {
    "\"".to_string() + s + "\""
}

fn format_boolean(b: &bool) -> String {
    match *b {
        true => BOOLEAN_TRUE_STR.to_string(),
        false => BOOLEAN_FALSE_STR.to_string(),
    }
}
