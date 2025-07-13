// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

use std::fmt::{self};

use crate::error::Error;

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Symbol(String),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, Error>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Expr::Symbol(s) => s.clone(),
            Expr::Number(n) => n.to_string(),
            Expr::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            Expr::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", s)
    }
}

