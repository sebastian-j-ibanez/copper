// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Types and functions for the Copper runtime environment.

mod io;
mod math;
mod operators;
mod predicates;
mod strings;

use crate::env::io::{display, newline, print, println};
use crate::env::math::{exponent, modulo};
pub use crate::env::operators::{add, div, mult, sub};
use crate::env::predicates::{
    is_boolean, is_complex, is_even, is_integer, is_list, is_number, is_odd, is_procedure,
    is_rational, is_real, is_string,
};
use crate::env::strings::{str_append, str_length};
use crate::types::Expr;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Env {
    pub data: HashMap<String, Expr>,
    pub outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn standard_env() -> Rc<RefCell<Env>> {
        let mut data: HashMap<String, Expr> = HashMap::new();
        // Operators
        data.insert("+".to_string(), Expr::Func(add));
        data.insert("-".to_string(), Expr::Func(sub));
        data.insert("*".to_string(), Expr::Func(mult));
        data.insert("/".to_string(), Expr::Func(div));
        // Math
        data.insert("modulo".to_string(), Expr::Func(modulo));
        data.insert("expt".to_string(), Expr::Func(exponent));
        // Predicates
        data.insert("number?".to_string(), Expr::Func(is_number));
        data.insert("real?".to_string(), Expr::Func(is_real));
        data.insert("rational?".to_string(), Expr::Func(is_rational));
        data.insert("complex?".to_string(), Expr::Func(is_complex));
        data.insert("integer?".to_string(), Expr::Func(is_integer));
        data.insert("string?".to_string(), Expr::Func(is_string));
        data.insert("boolean?".to_string(), Expr::Func(is_boolean));
        data.insert("list?".to_string(), Expr::Func(is_list));
        data.insert("procedure?".to_string(), Expr::Func(is_procedure));
        data.insert("even?".to_string(), Expr::Func(is_even));
        data.insert("odd?".to_string(), Expr::Func(is_odd));
        // IO
        data.insert("display".to_string(), Expr::Func(display));
        data.insert("newline".to_string(), Expr::Func(newline));
        data.insert("print".to_string(), Expr::Func(print));
        data.insert("println".to_string(), Expr::Func(println));
        // Strings
        data.insert("string-append".to_string(), Expr::Func(str_append));
        data.insert("string-length".to_string(),Expr::Func(str_length));
        Rc::new(RefCell::new(Env { data, outer: None }))
    }

    /// Initialize an empty environment.
    pub fn local_env(outer: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env {
            data: HashMap::new(),
            outer: Some(outer)
        }))
    }

    /// Find var in env. Checks self before outer env.
    pub fn find_var(&self, var: &str) -> Option<Expr> {
        if let Some(val) = self.data.get(var) {
            Some(val.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().find_var(var)
        } else {
            None
        }
    }
}
