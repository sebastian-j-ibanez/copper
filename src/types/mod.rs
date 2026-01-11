// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Copper data types.

pub mod number;
pub mod ports;

use crate::env::EnvRef;
use crate::error::Error;
use crate::types::ports::Port;
use num_integer::div_floor;
pub(crate) use number::Number;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: u64,
    pub converter: Option<Box<Expr>>,
}

impl Parameter {
    pub fn new(id: u64, converter: Option<Expr>) -> Parameter {
        Parameter {
            id,
            converter: converter.map(Box::new),
        }
    }
}

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

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
    Vector(Vector),
    ByteVector(ByteVector),
    Procedure(Procedure),
    Closure(Box<Closure>),
    Port(Port),
    Parameter(Parameter),
    Eof,
    Void(),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format_string(s),
            Expr::Char(c) => format_char(c),
            Expr::Boolean(b) => format_boolean(b),
            Expr::Symbol(s) => s.clone(),
            Expr::Pair(p) => format_pair(p, " ", true),
            Expr::Null => format_null(),
            Expr::Vector(v) => format_vector(v, true),
            Expr::ByteVector(bv) => format_bytevector(bv, true),
            Expr::Procedure(_) => String::from("#<function {}>"),
            Expr::Closure(_) => String::from("#<procedure {}>"),
            Expr::Port(p) => format_port(p),
            Expr::Parameter(p) => format!("#<parameter {}>", p.id),
            Expr::Eof => String::from("#!eof"),
            Expr::Void() => return Ok(()),
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
pub fn format_pair(pair: &Pair, delim: &str, parenthesis: bool) -> String {
    let (car, cdr) = &*pair.elements.borrow();

    if pair.is_list() {
        let items = pair
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(delim);

        return if parenthesis {
            format!("({items})")
        } else {
            items
        };
    }

    if parenthesis {
        return format!("({car} . {cdr})");
    }

    format!("{car}{cdr}")
}

/// Format vector into literal representation.
fn format_vector(vector: &Vector, literal: bool) -> String {
    let items = vector
        .elements
        .borrow()
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    if literal {
        return format!("#({})", items);
    }

    items
}

/// Format `Bytevector` into literal representation.
fn format_bytevector(vec: &ByteVector, literal: bool) -> String {
    let items = vec
        .buffer
        .borrow()
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    if literal {
        return format!("#u8({})", items);
    }

    items
}

/// Return literal representation of a null list.
pub fn format_null() -> String {
    String::from("()")
}

pub fn format_port(p: &Port) -> String {
    match *p {
        Port::TextInput(_) => "#<input-port (string)>".to_string(),
        Port::TextOutput(_) => "#<output-port (string)>".to_string(),
        Port::BinaryInput(_) => "#<input-port (binary)>".to_string(),
        Port::BinaryOutput(_) => "#<output-port (binary)>".to_string(),
    }
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
}

impl Pair {
    /// Create a new dotted `Pair`.
    pub fn cons(value: (Expr, Expr)) -> Pair {
        Pair {
            elements: Rc::new(RefCell::new(value)),
        }
    }

    /// Create a new list.
    pub fn list(values: &[Expr]) -> Expr {
        values.iter().rev().fold(Expr::Null, |cdr, car| {
            Expr::Pair(Pair::cons((car.clone(), cdr)))
        })
    }

    /// Return an immutable iterator over a `Pair`.
    pub fn iter(&self) -> PairIter {
        PairIter {
            current: Some(self.clone()),
        }
    }

    /// Return an mutable iterator over a `Pair`.
    pub fn iter_mut(&mut self) -> PairIterMut {
        PairIterMut {
            current: Some(self.clone()),
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

    /// Set the first element in the pair.
    pub fn set_car(&self, value: Expr) {
        self.elements.borrow_mut().0 = value
    }

    /// Set the last element in the pair.
    pub fn set_cdr(&self, value: Expr) {
        self.elements.borrow_mut().1 = value
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
                        Expr::Null => return Some(Expr::Null),
                        _ => p.clone(),
                    },
                    _ => return None,
                }
            };
            curr_pair = next_pair;
        }

        let curr_element = curr_pair.elements.borrow();
        return if even == 0 {
            Some(curr_element.1.clone())
        } else {
            Some(curr_element.0.clone())
        };
    }

    /// Set element from list.
    pub fn set(&self, value: Expr, mut index: usize) -> std::result::Result<(), Error> {
        let mut current = self.clone();
        let even = index % 2;
        let depth = index / 2;

        for _ in 0..depth {
            match current.cdr() {
                Expr::Pair(next) => current = next,
                _ => {
                    return Err(Error::Message(
                        "pair is not a null terminated list".to_string(),
                    ));
                }
            }
            index -= 1;
        }

        let mut borrowed_pair = current.elements.borrow_mut();
        if even == 0 {
            borrowed_pair.1 = value;
        } else {
            borrowed_pair.0 = value;
        }

        Ok(())
    }

    /// Append element and return new list. Does not mutate `&self`.
    pub fn append(&self, new_element: Expr) -> Result {
        let mut elements: Vec<Expr> = self.iter().collect();
        elements.push(new_element);
        Ok(Pair::list(elements.as_slice()))
    }

    /// Append element to list, mutating `&self`.
    pub fn append_mut(&mut self, element: Expr) -> std::result::Result<(), Error> {
        let mut current = self.clone();
        loop {
            match current.cdr() {
                Expr::Pair(next) => current = next,
                Expr::Null => break,
                _ => {
                    return Err(Error::Message(
                        "pair is not a null terminated list".to_string(),
                    ));
                }
            }
        }

        // Update last element.
        let mut tail = current.elements.borrow_mut();
        match element {
            Expr::Pair(ref p) if p.is_list() => {
                tail.1 = Expr::Pair(p.clone());
            }
            _ => {
                let new_tail = Pair::cons((element, Expr::Null));
                tail.1 = Expr::Pair(new_tail);
            }
        }

        Ok(())
    }

    /// Return the number of elements in the `Pair` or list.
    pub fn len(&self) -> usize {
        let mut len: usize = 1;
        let mut current = self.clone();

        loop {
            match current.cdr() {
                Expr::Pair(next) => current = next,
                _ => break,
            }
            len += 1;
        }

        len
    }

    /// Returns if `&self` is a list.
    pub fn is_list(&self) -> bool {
        let mut current = Some(self.clone());
        while let Some(pair) = current {
            match pair.cdr() {
                Expr::Pair(next) => current = Some(next),
                Expr::Null => return true,
                _ => return false,
            }
        }
        false
    }

    /// Return if cdr is an `Expr::Pair`.
    pub fn cdr_is_pair(&self) -> bool {
        match self.elements.borrow().1 {
            Expr::Pair(_) => true,
            _ => false,
        }
    }

    /// Return `Vector` created from `&self` elements.
    pub fn to_expr_vector(&self) -> Expr {
        let pair_elements: Vec<Expr> = self.iter().collect();
        Expr::Vector(Vector::from(pair_elements.as_slice()))
    }

    /// Return `String` created from `&self` elements.
    pub fn to_expr_string(&self) -> Result {
        let pair_elements = self
            .iter()
            .map(|e| match e {
                Expr::Char(c) => Ok(c),
                _ => return Err(Error::new("expected char")),
            })
            .collect::<std::result::Result<String, Error>>()?;
        Ok(Expr::String(pair_elements))
    }

    /// Return a new sub `Vector` with the given indices. `start` is inclusive and `end` is exclusive. Return `None` if `&self` is not a list.
    pub fn sub_list(&self, start: usize, end: usize) -> Option<Expr> {
        let len = self.len();
        if start < len && end <= len {
            let vector = self.iter().collect::<Vec<Expr>>();
            let sub_list = &vector.as_slice()[start..end];
            return Some(Pair::list(sub_list));
        }

        None
    }
}

/// Immutable iterator wrapper for `Pair`.
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

/// Proxy that refers to a single cons cell and can read or write its car.
pub struct PairElemMut {
    node: Pair,
}

impl PairElemMut {
    /// Get a cloned copy of the car.
    pub fn get(&self) -> Expr {
        self.node.car()
    }

    /// Set the `Pair` car a new value.
    pub fn set(&self, new_value: Expr) {
        let mut borrow = self.node.elements.borrow_mut();
        borrow.0 = new_value;
    }

    pub fn update<F: FnOnce(&mut Expr)>(&self, f: F) {
        let mut borrow = self.node.elements.borrow_mut();
        f(&mut borrow.0);
    }
}

/// Mutable iterator wrapper for `Pair`.
pub struct PairIterMut {
    current: Option<Pair>,
}

impl Iterator for PairIterMut {
    type Item = PairElemMut;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current.take()?;
        let handle = PairElemMut { node: node.clone() };
        let cdr = node.cdr();
        self.current = match cdr {
            Expr::Pair(next) => Some(next),
            _ => None,
        };
        Some(handle)
    }
}

#[derive(Debug, Clone)]
pub struct Vector {
    pub elements: Rc<RefCell<Vec<Expr>>>,
}

impl Vector {
    /// Create a new vector;
    pub fn new() -> Vector {
        Vector {
            elements: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Create a new vector from expressions.
    pub fn from(expressions: &[Expr]) -> Vector {
        let vector = Vector::new();
        for expr in expressions {
            vector.elements.borrow_mut().push(expr.clone());
        }
        vector
    }

    /// Allocate the vector to a specific size. Sets each element to `Expr::Void` by default.
    /// Will overwrite all data in the vector.
    pub fn alloc_size(&self, size: usize, default_value: Option<Expr>) {
        let mut vec_ref = self.elements.borrow_mut();
        let mut value = Expr::Void();
        if let Some(default) = default_value {
            value = default;
        }
        *vec_ref = vec![value; size];
    }

    /// Set element at `index` to `new_value`.
    pub fn set(&self, index: usize, new_value: Expr) -> std::result::Result<(), Error> {
        let mut vec_ref = self.elements.borrow_mut();
        match vec_ref.get(index) {
            Some(_) => {
                vec_ref[index] = new_value;
                Ok(())
            }
            None => Err(Error::new("index out of range")),
        }
    }

    /// Get element at `index`.
    pub fn get(&self, index: usize) -> Option<Expr> {
        let vec_ref = self.elements.borrow();
        match vec_ref.get(index) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    /// Return new `Expr::Pair` list created from `&self`.
    pub fn to_expr_list(&self) -> Expr {
        let vec_ref = self.elements.borrow();
        Pair::list(vec_ref.as_slice())
    }

    /// Return new `Expr::String` created from `&self`.
    pub fn to_expr_string(&self) -> Result {
        let str_elements = self
            .elements
            .borrow()
            .iter()
            .map(|e| match e {
                Expr::Char(c) => Ok(*c),
                _ => return Err(Error::new("expected char")),
            })
            .collect::<std::result::Result<String, Error>>()?;
        Ok(Expr::String(str_elements))
    }

    /// Create new `Vector` from a `String`.
    pub fn from_string(s: String) -> Vector {
        let chars = s.chars().map(|c| Expr::Char(c)).collect::<Vec<Expr>>();
        Vector::from(chars.as_slice())
    }

    /// Return a new sub `Vector` with the given indices. `start` is inclusive and `end` is exclusive.
    pub fn sub_vector(&self, start: usize, end: usize) -> Option<Vector> {
        let vec_ref = self.elements.borrow();
        let len = self.len();
        if start < len && end <= len {
            let sub_slice = &vec_ref[start..end];
            return Some(Vector::from(sub_slice));
        }

        None
    }

    /// Return length of `Vector`.
    pub fn len(&self) -> usize {
        self.elements.borrow().len()
    }

    /// Set each element from `start` to `end` as `new_value`.
    pub fn fill(
        &self,
        new_value: &Expr,
        start: usize,
        end: usize,
    ) -> std::result::Result<(), Error> {
        let len = self.len();
        if start < len && end <= len {
            self.elements
                .borrow_mut()
                .iter_mut()
                .enumerate()
                .skip(start)
                .take(end - start)
                .for_each(|(_, e)| {
                    *e = new_value.clone();
                });

            return Ok(());
        }

        Err(Error::new("out of range"))
    }

    /// Append elements from `other` into `&self`, leaving `other` empty.
    pub fn append(&self, other: Vector) {
        let mut other_elements = other.elements.borrow_mut();
        self.elements.borrow_mut().append(&mut other_elements)
    }

    /// Return a deep copy of `&self.elements`.
    pub fn deep_copy(&self) -> Vector {
        let copied_elements: Vec<Expr> = self.elements.borrow().iter().map(|e| e.clone()).collect();
        Vector::from(copied_elements.as_slice())
    }
}

#[derive(Debug, Clone)]
pub struct ByteVector {
    buffer: Rc<RefCell<Box<[u8]>>>,
}

impl ByteVector {
    /// Create new `ByteVector` with the given `size`.
    pub fn new(size: usize) -> Self {
        let buffer = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
        Self { buffer }
    }

    /// Create new `ByteVector` from a slice of `u8` bytes.
    pub fn from(bytes: &[u8]) -> Self {
        let vec = ByteVector::new(bytes.len());
        bytes
            .iter()
            .enumerate()
            .for_each(|(i, byte)| vec.buffer.borrow_mut()[i] = *byte);
        vec
    }

    /// Create new `ByteVector` from a String.
    pub fn from_string(s: String) -> ByteVector {
        let chars = s.chars().map(|c| c as u8).collect::<Vec<u8>>();
        ByteVector::from(chars.as_slice())
    }

    /// Set element at `index` to `value`.
    pub fn set(&self, index: usize, value: u8) -> std::result::Result<(), Error> {
        let mut buffer = self.buffer.borrow_mut();
        if index < buffer.len() {
            buffer[index] = value;
            return Ok(());
        }
        Err(Error::new("index out of range"))
    }

    /// Get byte at index. Returns `None` if index is outside vector bounds.
    pub fn get(&self, index: usize) -> Option<u8> {
        self.buffer.borrow().get(index).copied()
    }

    /// Return size of `buffer`.
    pub fn len(&self) -> usize {
        self.buffer.borrow().len()
    }

    /// Return copy of `ByteVector` from `start` and `end` indexes.
    pub fn sub_bytevector(&self, start: usize, end: usize) -> Option<ByteVector> {
        let vec_ref = self.buffer.borrow();
        let len = self.len();
        if start < len && end <= len {
            let sub_slice = &vec_ref[start..end];
            return Some(ByteVector::from(sub_slice));
        }

        None
    }

    /// Return contents of `&self` buffer as a `&[u8]`.
    pub fn to_slice(&self) -> Box<[u8]> {
        self.buffer.borrow().clone()
    }

    /// Convert a `u8` into a hex value `String`.
    pub fn utf8_to_hex_str(byte: u8) -> String {
        if byte > 31 {
            return format!("{}", char::from(byte));
        }

        format!("\\x{};", byte)
    }
}
