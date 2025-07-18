// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

pub mod operators;
pub mod predicates;

use crate::env::predicates::{
    is_boolean, is_complex, is_integer, is_list, is_number, is_procedure, is_rational, is_real,
    is_string,
};
use crate::types::Expr;
pub(crate) use operators::{add, div, mult, sub};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    pub data: HashMap<String, Expr>,
}

impl Env {
    pub fn default_env() -> Env {
        let mut data: HashMap<String, Expr> = HashMap::new();
        data.insert("+".to_string(), Expr::Func(add));
        data.insert("-".to_string(), Expr::Func(sub));
        data.insert("*".to_string(), Expr::Func(mult));
        data.insert("/".to_string(), Expr::Func(div));
        data.insert("number?".to_string(), Expr::Func(is_number));
        data.insert("real?".to_string(), Expr::Func(is_real));
        data.insert("rational?".to_string(), Expr::Func(is_rational));
        data.insert("complex?".to_string(), Expr::Func(is_complex));
        data.insert("integer?".to_string(), Expr::Func(is_integer));
        data.insert("string?".to_string(), Expr::Func(is_string));
        data.insert("boolean?".to_string(), Expr::Func(is_boolean));
        data.insert("list?".to_string(), Expr::Func(is_list));
        data.insert("procedure?".to_string(), Expr::Func(is_procedure));
        Env { data }
    }
}
