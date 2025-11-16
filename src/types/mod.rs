// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Copper data types.

pub mod number;

use crate::env::EnvRef;
use crate::error::Error;
use num_integer::div_floor;
pub(crate) use number::Number;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

// pub type List = Rc<RefCell<Vec<Expr>>>;
// pub type Pair = Rc<RefCell<(Expr, Expr)>>;

pub type Result = std::result::Result<Expr, Error>;
pub type Procedure = fn(&[Expr], EnvRef) -> Result;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Number),
    String(String),
    Char(char),
    Boolean(bool),
    Symbol(String),
    Pair(Pair),
    Null,
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
            Expr::Pair(pair) => format_pair(pair),
            Expr::Null => format_null(),
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

/// Format pair into literal representation.
pub fn format_pair(pair: &Pair) -> String {
    format!(
        "({} . {})",
        pair.elements.borrow().0,
        pair.elements.borrow().1
    )
}

/// Return literal representation of a null list.
pub fn format_null() -> String {
    String::from("()")
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

#[derive(Debug, Clone)]
pub struct Pair {
    elements: Rc<RefCell<(Expr, Expr)>>,
    index: usize,
}

impl Pair {
    /// Create a new `Expr::Pair`.
    pub fn cons(value: (Expr, Expr)) -> Pair {
        Pair {
            elements: Rc::new(RefCell::new(value)),
            index: 0,
        }
    }

    /// Create a new list.
    pub fn list(values: &[Expr]) -> Pair {
        values
            .iter()
            .rev()
            .fold(Pair::cons((Expr::Null, Expr::Null)), |cdr, car| {
                Pair::cons((car.clone(), Expr::Pair(cdr)))
            })
    }

    /// Returns if `&self` is a list.
    pub fn is_list(&self) -> bool {
        let items: Vec<Expr> = PairIter::new(self).into_iter().collect();
        match items.last() {
            Some(Expr::Null) => true,
            _ => false,
        }
    }

    /// Return if cdr is an `Expr::Pair`.
    pub fn cdr_is_pair(&self) -> bool {
        match self.elements.borrow().1 {
            Expr::Pair(_) => true,
            _ => false,
        }
    }

    /// Get first element.
    pub fn car(&self) -> Expr {
        self.elements.borrow().0.clone()
    }

    /// Get last element.
    pub fn cdr(&self) -> Expr {
        self.elements.borrow().1.clone()
    }

    /// Get element from list.
    pub fn get(&self, index: usize) -> Option<Expr> {
        if index == 0 {
            return Some(self.elements.borrow().0.clone());
        }
        let mut curr_pair = self.clone();
        let even = index % 2;
        let depth = div_floor(index, 2);
        for _ in 0..(depth + 1) {
            let next_pair = {
                let borrowed = curr_pair.elements.borrow();
                match &borrowed.1 {
                    Expr::Pair(p) => match p.elements.borrow().1 {
                        Expr::Null => return None,
                        _ => p.clone(),
                    },
                    _ => return None,
                }
            };
            curr_pair = next_pair;
        }

        let curr_element = curr_pair.elements.borrow();
        if even == 0 {
            return Some(curr_element.1.clone());
        } else {
            return Some(curr_element.0.clone());
        }
    }
}

pub struct PairIter {
    current: Option<Pair>,
}

impl PairIter {
    pub fn new(pair: &Pair) -> PairIter {
        PairIter {
            current: Some(pair.clone()),
        }
    }
}

impl Iterator for PairIter {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.current.clone()?;
        let car = pair.car();
        let cdr = pair.cdr();
        self.current = match cdr {
            Expr::Pair(next) => Some(next),
            _ => None,
        };
        Some(car)
    }
}
