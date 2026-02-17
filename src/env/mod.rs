// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-17

//! Types and functions for the Copper runtime environment.

mod procedures;

use crate::macros::{quote, set_car, set_cdr};
use crate::types::ports::Port;
use crate::types::{Expr, Parameter, Procedure};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global counter for generating unique parameter IDs.
static PARAMETER_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a unique parameter ID.
pub fn next_parameter_id() -> u64 {
    PARAMETER_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// A reference counted pointer to `Env`. Allows nested scoped environments.
pub type EnvRef = Rc<RefCell<Env>>;

#[derive(Debug, Clone)]
pub struct Env {
    pub data: HashMap<String, Expr>,
    params: HashMap<String, Expr>,
    pub outer: Option<EnvRef>,
}

impl Env {
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Env {
            data: HashMap::new(),
            params: HashMap::new(),
            outer: None,
        }))
    }

    pub fn standard_env() -> EnvRef {
        let env_ref = Env::new();
        {
            let mut env = env_ref.borrow_mut();
            // IO
            env.insert_proc("load", procedures::load_file);
            env.insert_proc("load", procedures::load_file);
            env.insert_proc("display", procedures::display);
            env.insert_proc("newline", procedures::newline);
            env.insert_proc("print", procedures::print);
            env.insert_proc("println", procedures::println);
            env.insert_proc("pp", procedures::pretty_print);
            // Math
            env.insert_proc("+", procedures::add);
            env.insert_proc("-", procedures::sub);
            env.insert_proc("*", procedures::mult);
            env.insert_proc("/", procedures::div);
            env.insert_proc("modulo", procedures::modulo);
            env.insert_proc("expt", procedures::exponent);
            env.insert_proc("abs", procedures::abs);
            env.insert_proc("ceiling", procedures::ceil);
            env.insert_proc("floor", procedures::floor);
            env.insert_proc("min", procedures::min);
            env.insert_proc("max", procedures::max);
            // Strings
            env.insert_proc("string", procedures::new_string);
            env.insert_proc("string-append", procedures::str_append);
            env.insert_proc("string-length", procedures::str_length);
            env.insert_proc("string-upcase", procedures::string_to_upcase);
            env.insert_proc("string-downcase", procedures::string_to_downcase);
            // Booleans
            env.insert_proc("not", procedures::not);
            env.insert_proc("and", procedures::and);
            env.insert_proc("or", procedures::or);
            // Lists & Pairs
            env.insert_proc("cons", procedures::cons_proc);
            env.insert_proc("list", procedures::new_list);
            env.insert_proc("append", procedures::list_append);
            env.insert_proc("length", procedures::list_length);
            env.insert_proc("car", procedures::car);
            env.insert_proc("cdr", procedures::cdr);
            env.insert_proc("cadr", procedures::cadr);
            env.insert_proc("set-car!", set_car);
            env.insert_proc("set-cdr!", set_cdr);
            env.insert_proc("reverse", procedures::list_reverse);
            // Vectors
            env.insert_proc("vector", procedures::new_vector);
            env.insert_proc("make-vector", procedures::make_vector);
            env.insert_proc("vector-ref", procedures::vector_ref);
            env.insert_proc("vector-set!", procedures::vector_set);
            env.insert_proc("vector-length", procedures::vector_len);
            env.insert_proc("vector-copy", procedures::vector_copy);
            env.insert_proc("vector-copy!", procedures::vector_copy_from);
            env.insert_proc("vector-fill!", procedures::vector_fill);
            env.insert_proc("vector-append", procedures::vector_append);
            // Bytevectors
            env.insert_proc("bytevector", procedures::new_bytevector);
            env.insert_proc("make-bytevector", procedures::make_bytevector);
            env.insert_proc("bytevector-length", procedures::bytevector_length);
            env.insert_proc("bytevector-u8-ref", procedures::bytevector_ref);
            env.insert_proc("bytevector-u8-set!", procedures::bytevector_set);
            env.insert_proc("bytevector-copy", procedures::bytevector_copy);
            env.insert_proc("bytevector-append", procedures::bytevector_append);
            env.insert_proc("bytevector-copy!", procedures::bytevector_copy_from);
            // Ports
            env.insert_proc("open-input-file", procedures::open_input_file);
            env.insert_proc("open-output-file", procedures::open_output_file);
            env.insert_proc("open-input-string", procedures::open_input_string);
            env.insert_proc("open-output-string", procedures::open_output_string);
            env.insert_proc("get-output-string", procedures::get_output_string);
            env.insert_proc("open-binary-input-file", procedures::open_binary_input_file);
            env.insert_proc(
                "open-binary-output-file",
                procedures::open_binary_output_file,
            );
            env.insert_proc("open-input-bytevector", procedures::open_input_bytevector);
            env.insert_proc("open-output-bytevector", procedures::open_output_bytevector);
            env.insert_proc("get-output-bytevector", procedures::get_output_bytevector);
            env.insert_proc("close-port", procedures::close_port);
            env.insert_proc("read-char", procedures::read_char);
            env.insert_proc("peek-char", procedures::peek_char);
            env.insert_proc("read-string", procedures::read_string);
            env.insert_proc("read-line", procedures::read_line);
            env.insert_proc("read-u8", procedures::read_u8);
            env.insert_proc("peek-u8", procedures::peek_u8);
            env.insert_proc("write-char", procedures::write_char);
            env.insert_proc("write-string", procedures::write_string);
            env.insert_proc("write-u8", procedures::write_u8);
            env.insert_proc("write-simple", procedures::write_simple);
            env.insert_proc("read-bytevector", procedures::read_bytevector);
            env.insert_proc("read-bytevector!", procedures::read_into_bytevector);
            env.insert_proc("flush-output-port", procedures::flush_output_port);
            env.insert_proc("call-with-port", procedures::call_with_port);
            env.insert_proc("call-with-input-file", procedures::call_with_input_file);
            env.insert_proc("call-with-output-file", procedures::call_with_output_file);
            env.insert_proc("eof-object", procedures::eof_object);
            // Conversions
            env.insert_proc("number->string", procedures::num_to_string);
            env.insert_proc("symbol->string", procedures::symbol_to_string);
            env.insert_proc("string->number", procedures::string_to_num);
            env.insert_proc("string->symbol", procedures::string_to_symbol);
            env.insert_proc("string->list", procedures::string_to_list);
            env.insert_proc("string->vector", procedures::string_to_vector);
            env.insert_proc("string->utf8", procedures::string_to_utf8);
            env.insert_proc("list->string", procedures::list_to_string);
            env.insert_proc("list->vector", procedures::list_to_vector);
            env.insert_proc("vector->list", procedures::vector_to_list);
            env.insert_proc("vector->string", procedures::vector_to_string);
            env.insert_proc("utf8->string", procedures::utf8_to_string);
            // Predicates
            env.insert_proc("number?", procedures::is_number);
            env.insert_proc("real?", procedures::is_real);
            env.insert_proc("rational?", procedures::is_rational);
            env.insert_proc("complex?", procedures::is_complex);
            env.insert_proc("integer?", procedures::is_integer);
            env.insert_proc("even?", procedures::is_even);
            env.insert_proc("odd?", procedures::is_odd);
            env.insert_proc("exact?", procedures::is_exact);
            env.insert_proc("inexact?", procedures::is_inexact);
            env.insert_proc("exact-integer?", procedures::is_exact_integer);
            env.insert_proc("symbol?", procedures::is_symbol);
            env.insert_proc("string?", procedures::is_string);
            env.insert_proc("char?", procedures::is_char);
            env.insert_proc("char-alphabetic?", procedures::is_char_alphabetic);
            env.insert_proc("char-numeric?", procedures::is_char_numeric);
            env.insert_proc("char-whitespace?", procedures::is_char_whitespace);
            env.insert_proc("char-upper-case?", procedures::is_char_uppercase);
            env.insert_proc("char-lower-case?", procedures::is_char_lowercase);
            env.insert_proc("boolean?", procedures::is_boolean);
            env.insert_proc("list?", procedures::is_list);
            env.insert_proc("pair?", procedures::is_pair);
            env.insert_proc("vector?", procedures::is_vector);
            env.insert_proc("procedure?", procedures::is_procedure);
            env.insert_proc("bytevector?", procedures::is_bytevector);
            env.insert_proc("input-port?", procedures::is_input_port);
            env.insert_proc("output-port?", procedures::is_output_port);
            env.insert_proc("textual-port?", procedures::is_textual_port);
            env.insert_proc("binary-port?", procedures::is_binary_port);
            env.insert_proc("char-ready?", procedures::is_char_ready);
            env.insert_proc("u8-ready?", procedures::is_byte_ready);
            env.insert_proc("input-port-open?", procedures::is_input_port_open);
            env.insert_proc("output-port-open?", procedures::is_output_port_open);
            env.insert_proc("eof-object?", procedures::is_eof_object);
            // Parameters
            env.insert_proc("make-parameter", procedures::make_parameter);
            env.insert_proc("parameter?", procedures::is_parameter);
            // Misc
            env.insert_proc("exit", procedures::exit);
            env.insert_proc("quote", quote);

            // Setup ports
            env.init_default_ports();
        }
        env_ref
    }

    /// Initialize an empty environment.
    pub fn local_env(outer: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Env {
            data: HashMap::new(),
            params: HashMap::new(),
            outer: Some(outer),
        }))
    }

    /// Find var in environment. Checks self before outer environment.
    pub fn find_var(&self, var: &str) -> Option<Expr> {
        if let Some(val) = self.data.get(var) {
            Some(val.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().find_var(var)
        } else {
            None
        }
    }

    /// Insert a new `Procedure` into `HashMap<String, Expr>`. Created to clean up boilerplate in `env::standard_env()`.
    fn insert_proc(&mut self, name: &str, function: Procedure) {
        self.data
            .insert(name.to_string(), Expr::Procedure(function));
    }

    /// Insert a new `Expr` into `&self`.
    pub fn insert_expr(&mut self, name: &str, value: Expr) {
        self.data.insert(name.to_string(), value);
    }

    /// Initialize parameter and set in environment. Created to clean up boilerplate in `env::standard_env()`.
    fn new_param(&mut self, name: &str, value: &Expr) {
        let id = next_parameter_id();
        let env_placeholder = Expr::Parameter(Parameter::new(id, None));
        self.data.insert(name.to_string(), env_placeholder);
        self.set_param(&id.to_string(), value);
    }

    /// Find parameter value by name. Looks up the `Parameter` in `self.data`,
    /// then uses its ID to retrieve the value from `self.params`.
    /// Checks outer env if name not found in current env.
    pub fn find_param(&self, param_name: &str) -> Option<Expr> {
        if let Some(Expr::Parameter(param)) = self.data.get(param_name) {
            let id = param.id.to_string();
            if let Some(val) = self.params.get(&id) {
                return Some(val.clone());
            }
        }

        if let Some(outer) = &self.outer {
            outer.borrow().find_param(param_name)
        } else {
            None
        }
    }

    /// Find parameter value by numeric ID.
    /// Checks outer env if parameter not found in current env.
    pub fn find_param_id(&self, id: &str) -> Option<Expr> {
        if let Some(val) = self.params.get(id) {
            Some(val.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().find_param_id(id)
        } else {
            None
        }
    }

    /// Set parameter in environment.
    pub fn set_param(&mut self, param: &str, value: &Expr) {
        self.params.insert(param.to_string(), value.clone());
    }

    /// Initialize default ports:
    /// - current-input-port
    /// - current-output-port
    fn init_default_ports(&mut self) {
        self.new_param("current-input-port", &Expr::Port(Port::text_input_stdin()));
        self.new_param(
            "current-output-port",
            &Expr::Port(Port::text_output_stdout()),
        );
    }
}
