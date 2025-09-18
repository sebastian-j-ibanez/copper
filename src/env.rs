// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Types and functions for the Copper runtime environment.

mod io;
mod lists;
mod math;
mod operators;
mod predicates;
mod strings;

// Internal crate imports.
use crate::env::io::{display, exit, load_file, newline, pretty_print, print, println};
use crate::env::lists::{cadr, car, cdr, cons, list_append, list_length, list_reverse, new_list};
use crate::env::math::{abs, ceil, exponent, floor, modulo};
pub use crate::env::operators::{add, div, mult, sub};
use crate::env::predicates::{
    is_boolean, is_complex, is_even, is_integer, is_list, is_number, is_odd, is_procedure,
    is_rational, is_real, is_string,
};
use crate::env::strings::{str_append, str_length};
use crate::macros::quote;
use crate::types::Expr;

// Std imports.
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
        data.insert("+".to_string(), Expr::Procedure(add));
        data.insert("-".to_string(), Expr::Procedure(sub));
        data.insert("*".to_string(), Expr::Procedure(mult));
        data.insert("/".to_string(), Expr::Procedure(div));
        // Math
        data.insert("modulo".to_string(), Expr::Procedure(modulo));
        data.insert("expt".to_string(), Expr::Procedure(exponent));
        data.insert("abs".to_string(), Expr::Procedure(abs));
        data.insert("ceiling".to_string(), Expr::Procedure(ceil));
        data.insert("floor".to_string(), Expr::Procedure(floor));
        // Predicates
        data.insert("number?".to_string(), Expr::Procedure(is_number));
        data.insert("real?".to_string(), Expr::Procedure(is_real));
        data.insert("rational?".to_string(), Expr::Procedure(is_rational));
        data.insert("complex?".to_string(), Expr::Procedure(is_complex));
        data.insert("integer?".to_string(), Expr::Procedure(is_integer));
        data.insert("string?".to_string(), Expr::Procedure(is_string));
        data.insert("boolean?".to_string(), Expr::Procedure(is_boolean));
        data.insert("list?".to_string(), Expr::Procedure(is_list));
        data.insert("procedure?".to_string(), Expr::Procedure(is_procedure));
        data.insert("even?".to_string(), Expr::Procedure(is_even));
        data.insert("odd?".to_string(), Expr::Procedure(is_odd));
        // IO
        data.insert("load".to_string(), Expr::Procedure(load_file));
        data.insert("display".to_string(), Expr::Procedure(display));
        data.insert("newline".to_string(), Expr::Procedure(newline));
        data.insert("print".to_string(), Expr::Procedure(print));
        data.insert("println".to_string(), Expr::Procedure(println));
        data.insert("pp".to_string(), Expr::Procedure(pretty_print));
        // Strings
        data.insert("string-append".to_string(), Expr::Procedure(str_append));
        data.insert("string-length".to_string(), Expr::Procedure(str_length));
        // Lists
        data.insert("list".to_string(), Expr::Procedure(new_list));
        data.insert("cons".to_string(), Expr::Procedure(cons));
        data.insert("append".to_string(), Expr::Procedure(list_append));
        data.insert("length".to_string(), Expr::Procedure(list_length));
        data.insert("car".to_string(), Expr::Procedure(car));
        data.insert("cdr".to_string(), Expr::Procedure(cdr));
        data.insert("cadr".to_string(), Expr::Procedure(cadr));
        data.insert("reverse".to_string(), Expr::Procedure(list_reverse));
        // Misc
        data.insert("exit".to_string(), Expr::Procedure(exit));
        data.insert("quote".to_string(), Expr::Procedure(quote));
        Rc::new(RefCell::new(Env { data, outer: None }))
    }

    /// Initialize an empty environment.
    pub fn local_env(outer: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env {
            data: HashMap::new(),
            outer: Some(outer),
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
