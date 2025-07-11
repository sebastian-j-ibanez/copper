// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

use std::collections::HashMap;
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
                let xs: Vec<String> = list
                    .iter()
                    .map(|x| x.to_string())
                    .collect();
                format!("({})", xs.join(","))
            },
            Expr::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Env {
    pub data: HashMap<String, Expr>
}

impl Env {
    pub fn default_env() -> Env {
        let mut data: HashMap<String, Expr> = HashMap::new();
        data.insert(
            "+".to_string(), 
            Expr::Func(
                |args: &[Expr]| -> Result<Expr, Error> {
                    let numbers = parse_number_list(args)?;
                    let sum: i32 = numbers
                        .iter()
                        .fold(0, |sum, a| sum + a);
                    Ok(Expr::Number(sum))
                }
            )
        );
        data.insert(
            "-".to_string(), 
            Expr::Func(
                |args: &[Expr]| -> Result<Expr, Error> {
                    let numbers = parse_number_list(args)?; 

                    let first = *numbers
                        .first()
                        .ok_or(
                            Error::Message("expected at least one number".to_string())
                        )?;

                    let sum_of_rest = numbers[1..].iter().fold(0, |sum, a| sum + a);

                    Ok(Expr::Number(first - sum_of_rest))
                }
            )
        );

        Env {data}
    }
}

fn parse_number_list(expressions: &[Expr]) -> Result<Vec<i32>, Error> {
    expressions
        .iter()
        .map(|e| parse_number(e))
        .collect()
}

fn parse_number(expr: &Expr) -> Result<i32, Error> {
    match expr {
        Expr::Number(num) => Ok(*num),
        _ => Err(Error::Message("expected a number".to_string()))
    } 
}
