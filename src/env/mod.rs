// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Types and functions for the Copper runtime environment.

mod bool;
mod convert;
mod io;
mod lists;
mod math;
mod operators;
mod pairs;
mod predicates;
mod strings;

use crate::env::bool::{and, not, or};
use crate::env::convert::{
    num_to_string, string_to_list, string_to_num, string_to_symbol, symbol_to_string,
};
use crate::env::io::{display, exit, load_file, newline, pretty_print, print, println};
use crate::env::lists::{cadr, car, cdr, list_append, list_length, list_reverse, new_list};
use crate::env::math::{abs, ceil, exponent, floor, max, min, modulo};
pub use crate::env::operators::{add, div, mult, sub};
use crate::env::pairs::cons;
use crate::env::predicates::{
    is_boolean, is_char, is_char_alphabetic, is_char_lowercase, is_char_numeric, is_char_uppercase,
    is_char_whitespace, is_complex, is_even, is_integer, is_list, is_number, is_odd, is_pair,
    is_procedure, is_rational, is_real, is_string, is_symbol,
};
use crate::env::strings::{
    new_string, str_append, str_length, string_to_downcase, string_to_upcase,
};
use crate::macros::quote;
use crate::types::Expr;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A reference counted pointer to Env. Allows nested scoped environments.
pub type EnvRef = Rc<RefCell<Env>>;

#[derive(Debug, Clone)]
pub struct Env {
    pub data: HashMap<String, Expr>,
    pub outer: Option<EnvRef>,
}

impl Env {
    pub fn standard_env() -> EnvRef {
        let mut data: HashMap<String, Expr> = HashMap::new();
        // Booleans
        data.insert("not".to_string(), Expr::Procedure(not));
        data.insert("and".to_string(), Expr::Procedure(and));
        data.insert("or".to_string(), Expr::Procedure(or));
        // Conversions
        data.insert("number->string".to_string(), Expr::Procedure(num_to_string));
        data.insert(
            "symbol->string".to_string(),
            Expr::Procedure(symbol_to_string),
        );
        data.insert("string->number".to_string(), Expr::Procedure(string_to_num));
        data.insert(
            "string->symbol".to_string(),
            Expr::Procedure(string_to_symbol),
        );
        data.insert("string->list".to_string(), Expr::Procedure(string_to_list));
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
        data.insert("min".to_string(), Expr::Procedure(min));
        data.insert("max".to_string(), Expr::Procedure(max));
        // Predicates
        data.insert("symbol?".to_string(), Expr::Procedure(is_symbol));
        data.insert("string?".to_string(), Expr::Procedure(is_string));
        data.insert("char?".to_string(), Expr::Procedure(is_char));
        data.insert(
            "char-alphabetic?".to_string(),
            Expr::Procedure(is_char_alphabetic),
        );
        data.insert(
            "char-numeric?".to_string(),
            Expr::Procedure(is_char_numeric),
        );
        data.insert(
            "char-whitespace?".to_string(),
            Expr::Procedure(is_char_whitespace),
        );
        data.insert(
            "char-upper-case?".to_string(),
            Expr::Procedure(is_char_uppercase),
        );
        data.insert(
            "char-lower-case?".to_string(),
            Expr::Procedure(is_char_lowercase),
        );
        data.insert("boolean?".to_string(), Expr::Procedure(is_boolean));
        data.insert("list?".to_string(), Expr::Procedure(is_list));
        data.insert("pair?".to_string(), Expr::Procedure(is_pair));
        data.insert("procedure?".to_string(), Expr::Procedure(is_procedure));
        data.insert("number?".to_string(), Expr::Procedure(is_number));
        data.insert("real?".to_string(), Expr::Procedure(is_real));
        data.insert("rational?".to_string(), Expr::Procedure(is_rational));
        data.insert("complex?".to_string(), Expr::Procedure(is_complex));
        data.insert("integer?".to_string(), Expr::Procedure(is_integer));
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
        data.insert("string".to_string(), Expr::Procedure(new_string));
        data.insert("string-append".to_string(), Expr::Procedure(str_append));
        data.insert("string-length".to_string(), Expr::Procedure(str_length));
        data.insert(
            "string-upcase".to_string(),
            Expr::Procedure(string_to_upcase),
        );
        data.insert(
            "string-downcase".to_string(),
            Expr::Procedure(string_to_downcase),
        );
        // Lists & Pairs
        data.insert("cons".to_string(), Expr::Procedure(cons));
        data.insert("list".to_string(), Expr::Procedure(new_list));
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
    pub fn local_env(outer: EnvRef) -> EnvRef {
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
