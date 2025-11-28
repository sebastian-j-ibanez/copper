// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Types and functions for the Copper runtime environment.

mod procedures;

use crate::env::procedures::{
    abs, add, and, bytevector_append, bytevector_copy, bytevector_copy_from, bytevector_length,
    bytevector_ref, bytevector_set, cadr, car, cdr, ceil, cons_proc, display, div, exit, exponent,
    floor, is_boolean, is_bytevector, is_char, is_char_alphabetic, is_char_lowercase,
    is_char_numeric, is_char_uppercase, is_char_whitespace, is_complex, is_even, is_exact,
    is_exact_integer, is_inexact, is_integer, is_list, is_number, is_odd, is_pair, is_procedure,
    is_rational, is_real, is_string, is_symbol, is_vector, list_append, list_length, list_reverse,
    list_to_string, list_to_vector, load_file, make_bytevector, make_vector, max, min, modulo,
    mult, new_bytevector, new_list, new_string, new_vector, newline, not, num_to_string,
    open_binary_input_file, open_binary_output_file, open_input_file, open_output_file, or,
    peek_char, peek_u8, pretty_print, print, println, read_char, read_u8, str_append, str_length,
    string_to_downcase, string_to_list, string_to_num, string_to_symbol, string_to_upcase,
    string_to_utf8, string_to_vector, sub, symbol_to_string, utf8_to_string, vector_append,
    vector_copy, vector_copy_from, vector_fill, vector_len, vector_ref, vector_set, vector_to_list,
    vector_to_string, write_char, write_u8,
};
use crate::macros::{quote, set_car, set_cdr};
use crate::types::{Expr, Procedure};

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
    pub fn new() -> EnvRef {
        let map: HashMap<String, Expr> = HashMap::new();
        Rc::new(RefCell::new(Env {
            data: map,
            outer: None,
        }))
    }

    pub fn standard_env() -> EnvRef {
        let env_ref = Env::new();
        {
            let mut env = env_ref.borrow_mut();
            // IO
            env.insert_proc("load", load_file);
            env.insert_proc("load", load_file);
            env.insert_proc("display", display);
            env.insert_proc("newline", newline);
            env.insert_proc("print", print);
            env.insert_proc("println", println);
            env.insert_proc("pp", pretty_print);
            // Math
            env.insert_proc("+", add);
            env.insert_proc("-", sub);
            env.insert_proc("*", mult);
            env.insert_proc("/", div);
            env.insert_proc("modulo", modulo);
            env.insert_proc("expt", exponent);
            env.insert_proc("abs", abs);
            env.insert_proc("ceiling", ceil);
            env.insert_proc("floor", floor);
            env.insert_proc("min", min);
            env.insert_proc("max", max);
            // Strings
            env.insert_proc("string", new_string);
            env.insert_proc("string-append", str_append);
            env.insert_proc("string-length", str_length);
            env.insert_proc("string-upcase", string_to_upcase);
            env.insert_proc("string-downcase", string_to_downcase);
            // Booleans
            env.insert_proc("not", not);
            env.insert_proc("and", and);
            env.insert_proc("or", or);
            // Lists & Pairs
            env.insert_proc("cons", cons_proc);
            env.insert_proc("list", new_list);
            env.insert_proc("append", list_append);
            env.insert_proc("length", list_length);
            env.insert_proc("car", car);
            env.insert_proc("cdr", cdr);
            env.insert_proc("cadr", cadr);
            env.insert_proc("set-car!", set_car);
            env.insert_proc("set-cdr!", set_cdr);
            env.insert_proc("reverse", list_reverse);
            // Vectors
            env.insert_proc("vector", new_vector);
            env.insert_proc("make-vector", make_vector);
            env.insert_proc("vector-ref", vector_ref);
            env.insert_proc("vector-set!", vector_set);
            env.insert_proc("vector-length", vector_len);
            env.insert_proc("vector-copy", vector_copy);
            env.insert_proc("vector-copy!", vector_copy_from);
            env.insert_proc("vector-fill!", vector_fill);
            env.insert_proc("vector-append", vector_append);
            // Bytevectors
            env.insert_proc("bytevector", new_bytevector);
            env.insert_proc("make-bytevector", make_bytevector);
            env.insert_proc("bytevector-length", bytevector_length);
            env.insert_proc("bytevector-u8-ref", bytevector_ref);
            env.insert_proc("bytevector-u8-set!", bytevector_set);
            env.insert_proc("bytevector-copy", bytevector_copy);
            env.insert_proc("bytevector-append", bytevector_append);
            env.insert_proc("bytevector-copy!", bytevector_copy_from);
            // Ports
            env.insert_proc("open-input-file", open_input_file);
            env.insert_proc("open-binary-input-file", open_binary_input_file);
            env.insert_proc("open-output-file", open_output_file);
            env.insert_proc("open-binary-output-file", open_binary_output_file);
            env.insert_proc("read-char", read_char);
            env.insert_proc("peek-char", peek_char);
            env.insert_proc("write-char", write_char);
            env.insert_proc("read-u8", read_u8);
            env.insert_proc("peek-u8", peek_u8);
            env.insert_proc("write-u8", write_u8);
            // Conversions
            env.insert_proc("number->string", num_to_string);
            env.insert_proc("symbol->string", symbol_to_string);
            env.insert_proc("string->number", string_to_num);
            env.insert_proc("string->symbol", string_to_symbol);
            env.insert_proc("string->list", string_to_list);
            env.insert_proc("string->vector", string_to_vector);
            env.insert_proc("string->utf8", string_to_utf8);
            env.insert_proc("list->string", list_to_string);
            env.insert_proc("list->vector", list_to_vector);
            env.insert_proc("vector->list", vector_to_list);
            env.insert_proc("vector->string", vector_to_string);
            env.insert_proc("utf8->string", utf8_to_string);
            // Predicates
            env.insert_proc("number?", is_number);
            env.insert_proc("real?", is_real);
            env.insert_proc("rational?", is_rational);
            env.insert_proc("complex?", is_complex);
            env.insert_proc("integer?", is_integer);
            env.insert_proc("even?", is_even);
            env.insert_proc("odd?", is_odd);
            env.insert_proc("exact?", is_exact);
            env.insert_proc("inexact?", is_inexact);
            env.insert_proc("exact-integer?", is_exact_integer);
            env.insert_proc("symbol?", is_symbol);
            env.insert_proc("string?", is_string);
            env.insert_proc("char?", is_char);
            env.insert_proc("char-alphabetic?", is_char_alphabetic);
            env.insert_proc("char-numeric?", is_char_numeric);
            env.insert_proc("char-whitespace?", is_char_whitespace);
            env.insert_proc("char-upper-case?", is_char_uppercase);
            env.insert_proc("char-lower-case?", is_char_lowercase);
            env.insert_proc("boolean?", is_boolean);
            env.insert_proc("list?", is_list);
            env.insert_proc("pair?", is_pair);
            env.insert_proc("vector?", is_vector);
            env.insert_proc("procedure?", is_procedure);
            env.insert_proc("bytevector?", is_bytevector);
            // Misc
            env.insert_proc("exit", exit);
            env.insert_proc("quote", quote);
        }
        env_ref
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

    /// Insert a new `Procedure` into `HashMap<String, Expr>`. Only used to clean up boilerplate in `env::standard_env()`.
    fn insert_proc(&mut self, name: &str, function: Procedure) {
        self.data
            .insert(name.to_string(), Expr::Procedure(function));
    }

    /// Insert a new `Expr` into `&self`.
    pub fn insert_expr(&mut self, name: &str, value: Expr) {
        self.data.insert(name.to_string(), value);
    }
}
