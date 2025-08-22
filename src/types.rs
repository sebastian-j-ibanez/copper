// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Copper data types.

pub mod number;

use crate::error::Error;
pub(crate) use number::Number;
use std::fmt;
use crate::env::Env;

use std::rc::Rc;
use std::cell::RefCell;

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    String(String),
    Boolean(bool),
    Symbol(String),
    List(Vec<Expr>),
    Void(),
    Func(fn(&[Expr], Rc<RefCell<Env>>) -> Result<Expr, Error>),
    Closure(Box<Closure>)
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
            Expr::Void() => return Ok(()),
            Expr::Func(_) => "#<function {}".to_string(),
            Expr::Closure(_) => "#<procedure {}>".to_string(),
        };
        write!(f, "{}", s)
    }
}

fn format_string(s: &String) -> String {
    format!("\"{}\"", s)
}

fn format_boolean(b: &bool) -> String {
    match *b {
        true => BOOLEAN_TRUE_STR.to_string(),
        false => BOOLEAN_FALSE_STR.to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub env: Rc<RefCell<Env>>,
    pub parameters: Vec<String>,
    pub body: Expr,
}

impl Closure {
    pub fn init(env: Rc<RefCell<Env>>, parameters: Vec<String>, body: Expr) -> Closure {
        Closure { env, parameters, body }
    }    
}
