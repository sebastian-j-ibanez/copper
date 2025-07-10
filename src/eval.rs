// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Operator(Operator),
    DataType(DataType),
}

impl Symbol {
    /// Create Symbol from &str, returns None if no match.
    pub fn from_str(s: &str) -> Option<Symbol> {
        if let Some(op) = Operator::from_str(s) {
            return Some(Symbol::Operator(op));
        }
        
        if let Some(dt) = DataType::from_str(s) {
            return Some(Symbol::DataType(dt))
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Divide,
}

impl Operator {
    /// Create Operator from &str, returns None if no match.
    pub fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mult),
            "/" => Some(Operator::Divide),
            _ => None
        } 
    }

    /// Evaluate an operation, returning an i32 result.
    pub fn eval_operator(&self, a: &DataType, b: &DataType) -> Result<i32, crate::error::Error> {
        match (a, b) {
            (a_value, b_value)
            if !matches!(a_value, DataType::Number(_))
            && !matches!(b_value, DataType::Number(_)) => {
                let message = format!(
                    "unable to evaluate operator: {:?} and {:?} are not numbers",
                    a_value,
                    b_value
                );
                let e = crate::error::Error::init(&message);
                return Err(e);
            },
            (DataType::Number(a_value), DataType::Number(b_value)) => {
                match self {
                    Operator::Add => { return Ok(a_value + b_value) },
                    Operator::Sub => { return Ok(a_value - b_value) },
                    Operator::Mult => { return Ok(a_value * b_value) },
                    Operator::Divide => { return Ok(a_value / b_value) }
                }
            },
            (DataType::Number(_), b) => {
                let message = format!("unable to evaluate operator: {:?} is not a number", b);
                let e = crate::error::Error::init(&message);
                return Err(e);
            },
            (a, DataType::Number(_)) => {
                let message = format!("unable to evaluate operator: {:?} is not a number", a);
                let e = crate::error::Error::init(&message);
                return Err(e);
            },
            _ => {
                let message = format!("unable to evaluate operator: something has gone wrong");
                let e = crate::error::Error::init(&message);
                return Err(e);
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataType{
    Number(i32),
    String(String),
    Boolean(bool),
}

impl DataType {
    /// Create DataType from &str, returns None if no match.
    pub fn from_str(s: &str) -> Option<DataType> {
        let mut chars = s.chars();
        while let Some(character) = chars.next() {
            if !character.is_ascii_digit() {
                return None
            }
        }

        if let Ok(num) = FromStr::from_str(s) {
            return Some(DataType::Number(num))
        }

        None
    }
}

