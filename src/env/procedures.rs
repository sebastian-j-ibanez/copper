// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-11-11

use crate::env::{EnvRef, next_parameter_id};
use crate::error::Error;
use crate::macros::apply_lambda;
use crate::types::number::IntVariant::Small;
use crate::types::ports::{BinaryOutputPort, Port};
use crate::types::{
    ByteVector, Expr, Number, Pair, PairIter, Parameter, Result, Vector, format_pair,
};
use crate::{io, parser};
use std::ops::{Add, Deref, Div, Mul, Sub};

// I/O

/// Display raw expression in stdout.
pub fn display(args: &[Expr], _: EnvRef) -> Result {
    match args.first() {
        Some(arg) => {
            print!("{}", arg);
            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected 1 valid expression")),
    }
}

/// Return newline character.
pub fn newline(_: &[Expr], _: EnvRef) -> Result {
    println!();
    Ok(Expr::Void())
}

/// Print formatted value of expression in stdout.
pub fn print(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => print!("{}", s),
            Expr::Char(c) => print!("{}", c),
            Expr::Pair(p) => {
                print!("{}", format_pair(p, "", false));
            }
            _ => print!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::new("expected 1 valid expression"))
}

/// Print formatted value of expression in stdout with a newline.
pub fn println(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => println!("{}", s),
            Expr::Char(c) => println!("{}", c),
            _ => println!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::new("expected 1 valid expression"))
}

/// Evaluate the contents of a file.
pub fn load_file(args: &[Expr], env: EnvRef) -> Result {
    let file = match args.first() {
        Some(Expr::String(f)) => f,
        _ => return Err(Error::new("expected a string path")),
    };

    let expressions = io::file_input(file.to_owned());
    io::process_file_input(expressions, env);

    Ok(Expr::Void())
}

/// End process.
pub fn exit(_: &[Expr], _: EnvRef) -> Result {
    std::process::exit(0);
}

/// Print literal.
pub fn pretty_print(args: &[Expr], _: EnvRef) -> Result {
    match args.first() {
        Some(Expr::Closure(c)) => {
            let c_args = c.parameters.join(" ");
            println!("(lambda ({}) {})", c_args, c.body);
            return Ok(Expr::Void());
        }
        Some(_) => {
            println!("{}", args[0]);
            return Ok(Expr::Void());
        }
        None => return Err(Error::new("expected ")),
    }
}

// Math

/// Add all arguments together.
pub fn add(args: &[Expr], _: EnvRef) -> Result {
    let numbers = parser::parse_number_list(args)?;
    let initial_sum = Number::from_i64(0);
    let sum = numbers
        .into_iter()
        .try_fold(initial_sum, |current_sum, num| current_sum.add(num))?;
    Ok(Expr::Number(sum))
}

/// Subtract all arguments together.
pub fn sub(args: &[Expr], _: EnvRef) -> Result {
    let numbers = parser::parse_number_list(args)?;
    if numbers.is_empty() {
        return Ok(Expr::Number(Number::from_i64(0)));
    }

    let mut iter = numbers.clone().into_iter();
    let first_num = iter.next().unwrap();

    if iter.next().is_none() {
        let zero = Number::from_i64(0);
        let result = zero.sub(first_num)?;
        Ok(Expr::Number(result))
    } else {
        let mut iter = numbers.into_iter();
        let initial_diff = iter.next().unwrap();
        let final_diff = iter.try_fold(initial_diff, |current_diff, num| current_diff.sub(num))?;
        Ok(Expr::Number(final_diff))
    }
}

/// Multiply all arguments together.
pub fn mult(args: &[Expr], _: EnvRef) -> Result {
    let numbers = parser::parse_number_list(args)?;
    if numbers.is_empty() {
        return Err(Error::new("expected at least one number"));
    }
    let initial_value: Number = Number::from_i64(1);
    let product = numbers
        .into_iter()
        .try_fold(initial_value, |current_product, num| {
            current_product.mul(num)
        })?;
    Ok(Expr::Number(product))
}

/// Divide all arguments together.
pub fn div(args: &[Expr], _: EnvRef) -> Result {
    let numbers = parser::parse_number_list(args)?;
    if numbers.is_empty() {
        return Err(Error::new("expected at least one number"));
    }
    let mut length_check_iter = numbers.clone().into_iter();
    length_check_iter.next();
    if length_check_iter.next().is_none() {
        let one = Number::from_i64(1);
        let first_num = numbers.into_iter().next().unwrap();
        let result = one.div(first_num).map_err(Error::from)?;
        Ok(Expr::Number(result))
    } else {
        let mut iter = numbers.into_iter();
        let first_num = iter.next().unwrap();
        let result = iter.try_fold(first_num, |current_quotient, num| {
            current_quotient.div(num).map_err(Error::from)
        })?;
        Ok(Expr::Number(result))
    }
}

/// Apply exponent to number.
pub fn exponent(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(a), Expr::Number(b)] => Ok(Expr::Number(a.pow(b)?)),
        _ => Err(Error::new("expected 2 numbers")),
    }
}

/// Perform modulo to number.
pub fn modulo(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(a), Expr::Number(b)] => {
            let a = a.clone();
            let b = b.clone();
            Ok(Expr::Number((a % b)?))
        }
        _ => Err(Error::new("expected 2 numbers")),
    }
}

/// Get absolute value of number.
pub fn abs(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(n)] => {
            let n = n.clone();
            if n < Number::from_i64(0) {
                return if let Ok(result) = n.clone() * Number::from_i64(-1) {
                    Ok(Expr::Number(result))
                } else {
                    Err(Error::Message(format!(
                        "unable to get absolute value from n: {}",
                        n
                    )))
                };
            }
            Ok(Expr::Number(n))
        }
        _ => Err(Error::new("expected 1 number")),
    }
}

/// Round number up to the nearest integer.
pub fn ceil(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => Err(Error::new("unable to round complex number")),
        [Expr::Number(n)] => {
            if let Some(result) = n.to_f64() {
                return Ok(Expr::Number(Number::from_f64(result.ceil())));
            }
            Err(Error::Message(
                "unable to convert number to float".to_string(),
            ))
        }
        _ => Err(Error::new("expected real number")),
    }
}

/// Round number down to the nearest integer.
pub fn floor(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => Err(Error::new("unable to round complex number")),
        [Expr::Number(n)] => {
            if let Some(result) = n.to_f64() {
                return Ok(Expr::Number(Number::from_f64(result.floor())));
            }
            Err(Error::Message(
                "unable to convert number to float".to_string(),
            ))
        }
        _ => Err(Error::new("expected real number")),
    }
}

/// Return smallest real number from arguments.
pub fn min(args: &[Expr], _: EnvRef) -> Result {
    if args.is_empty() {
        return Err(Error::new("expected real numbers"));
    }

    let mut min: Option<Number> = None;

    for arg in args {
        match arg {
            Expr::Number(current) => match current {
                Number::Complex(_) => {
                    return Err(Error::new("expected real numbers"));
                }
                _ => match min {
                    None => min = Some(current.clone()),
                    Some(ref current_min) => {
                        if current < current_min {
                            min = Some(current.clone());
                        }
                    }
                },
            },
            _ => {
                return Err(Error::new("expected real numbers"));
            }
        }
    }

    Ok(Expr::Number(min.unwrap()))
}

/// Return largest real number from arguments.
pub fn max(args: &[Expr], _: EnvRef) -> Result {
    if args.is_empty() {
        return Err(Error::new("expected real numbers"));
    }

    let mut min: Option<Number> = None;

    for arg in args {
        match arg {
            Expr::Number(current) => match current {
                Number::Complex(_) => {
                    return Err(Error::new("expected real numbers"));
                }
                _ => match min {
                    None => min = Some(current.clone()),
                    Some(ref current_min) => {
                        if current > current_min {
                            min = Some(current.clone());
                        }
                    }
                },
            },
            _ => {
                return Err(Error::new("expected real numbers"));
            }
        }
    }

    Ok(Expr::Number(min.unwrap()))
}

// Strings

/// Appends two strings together.
pub fn str_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(a), Expr::String(b)] => {
            let c = a.clone() + b;
            Ok(Expr::String(c))
        }
        _ => Err(Error::Message(format!("expected 2 strings"))),
    }
}

/// Returns the size of a string as an `Expr::Number` (more specifically an `IntVariant::Small`).
pub fn str_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Number(Number::from_usize(s.len()))),
        _ => Err(Error::new("expected string")),
    }
}

/// Create either a new empty string or a string from a char.
pub fn new_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [] => Ok(Expr::String(String::new())),
        [Expr::Char(c)] => Ok(Expr::String(String::from(*c))),
        _ => Err(Error::new("expected character")),
    }
}

/// Convert string to upper case.
pub fn string_to_upcase(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => {
            let upcase = s
                .chars()
                .map(|c| c.to_ascii_uppercase())
                .collect::<String>();
            return Ok(Expr::String(upcase));
        }
        _ => Err(Error::new("expected string")),
    }
}

/// Convert string to lowercase.
pub fn string_to_downcase(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => {
            let upcase = s
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>();
            return Ok(Expr::String(upcase));
        }
        _ => Err(Error::new("expected string")),
    }
}

// Boolean

/// Returns the opposite value of a `bool`.
pub fn not(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Boolean(false)] => Ok(Expr::Boolean(true)),
        _ => Ok(Expr::Boolean(false)),
    }
}

/// Returns `true` if any arguments are `false`.
pub fn and(args: &[Expr], _: EnvRef) -> Result {
    let contains_false = args.iter().all(|arg| !matches!(arg, Expr::Boolean(false)));
    Ok(Expr::Boolean(contains_false))
}

/// Returns `false` if any arguments are `true`.
pub fn or(args: &[Expr], _: EnvRef) -> Result {
    let contains_true = args.iter().all(|arg| !matches!(arg, Expr::Boolean(true)));
    Ok(Expr::Boolean(contains_true))
}

// Pairs & Lists

/// Construct a new pair from 2 expressions.
pub fn cons_proc(args: &[Expr], _: EnvRef) -> Result {
    use crate::types::Pair;
    match args {
        [a, b] => Ok(Expr::Pair(Pair::cons((a.clone(), b.clone())))),
        _ => Err(Error::new("expected 2 arguments")),
    }
}

/// Make a new list.
pub fn new_list(args: &[Expr], _: EnvRef) -> Result {
    Ok(Pair::list(args))
}

/// Append 2 lists together.
pub fn list_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(list_a), Expr::Pair(list_b)] if list_a.is_list() && list_b.is_list() => {
            let result = list_a.clone().append(Expr::Pair(list_b.clone()))?;
            Ok(result)
        }
        _ => Err(Error::new("expected 2 lists")),
    }
}

/// Get length of list.
pub fn list_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(p)] => Ok(Expr::Number(Number::from_usize(p.len()))),
        _ => Err(Error::new("expected list")),
    }
}

/// Return first element from `Pair`.
pub fn car(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.car()),
        _ => Err(Error::new("expected pair")),
    }
}

/// Return second element from `Pair`.
pub fn cdr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.cdr()),
        _ => Err(Error::new("expected pair")),
    }
}

/// Return car of cdr.
pub fn cadr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(p)] => {
            let cdr = p.cdr();
            match cdr {
                Expr::Pair(p) => Ok(p.car()),
                _ => Err(Error::Message(
                    "expected list of at least 2 items".to_string(),
                )),
            }
        }
        _ => Err(Error::new("expected list")),
    }
}

/// Reverse list.
pub fn list_reverse(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => {
            let items: Vec<Expr> = PairIter::new(pair).map(|e| e.clone()).collect();
            let reversed: Vec<Expr> = items.into_iter().rev().collect::<Vec<_>>();
            Ok(Pair::list(&reversed))
        }
        _ => Err(Error::new("expected list")),
    }
}

// Vectors

/// Create a new vector containing the given arguments.
pub fn new_vector(args: &[Expr], _: EnvRef) -> Result {
    Ok(Expr::Vector(Vector::from(args)))
}

/// Create a new vector with an optional pre-allocated size.
pub fn make_vector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(n)] => match n.to_usize() {
            Some(size) => {
                let vector = Vector::new();
                vector.alloc_size(size, None);
                Ok(Expr::Vector(vector))
            }
            _ => Err(Error::Message(
                "invalid size, expected int or float".to_string(),
            )),
        },
        _ => Ok(Expr::Vector(Vector::new())),
    }
}

/// Return contents of vector at specified index.
pub fn vector_ref(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(v), Expr::Number(n)] => match n.to_usize() {
            Some(size) => match v.get(size) {
                Some(e) => Ok(e.clone()),
                _ => Err(Error::new("invalid index")),
            },
            _ => Err(Error::Message(
                "invalid length, expected int or float".to_string(),
            )),
        },
        _ => Ok(Expr::Vector(Vector::new())),
    }
}

/// Set contents of vector at specified index.
pub fn vector_set(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(v), Expr::Number(n), expr] => match n.to_usize() {
            Some(index) => {
                v.set(index, expr.clone())?;
                Ok(Expr::Void())
            }
            _ => Err(Error::new("invalid index")),
        },
        _ => Err(Error::Message(
            "expected vector, index, and new value".to_string(),
        )),
    }
}

/// Return length of vector.
pub fn vector_len(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(v)] => Ok(Expr::Number(Number::from_usize(v.elements.borrow().len()))),
        _ => Err(Error::new("expected vector")),
    }
}

/// Return a newly allocated copy of `&self`. Accepts optional start and end indexes.
pub fn vector_copy(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(v)] => Ok(Expr::Vector(v.clone())),
        [Expr::Vector(vec), Expr::Number(start)] => {
            if *start == Number::from_usize(vec.len()) {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match vec.sub_vector(start, vec.len()) {
                Some(v) => Ok(Expr::Vector(v.clone())),
                None => Err(Error::new("out of range")),
            }
        }
        [Expr::Vector(vec), Expr::Number(start), Expr::Number(end)] => {
            let v_len = Number::from_usize(vec.len());
            if *start == v_len && *end == v_len {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            let end = match end.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match vec.sub_vector(start, end) {
                Some(v) => Ok(Expr::Vector(v.clone())),
                None => Err(Error::new("out of range")),
            }
        }
        _ => Err(Error::new("expected vector")),
    }
}

/// Copy elements from one `ByteVector` into another using range indexes.
pub fn vector_copy_from(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(dest), Expr::Number(at), Expr::Vector(from)] => {
            let at = at
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if at > dest.len() {
                return Err(Error::new("out of range"));
            }

            for (i, expr) in from.elements.borrow().iter().enumerate() {
                dest.set(i + at, expr.clone())?;
            }

            Ok(Expr::Void())
        }
        [
            Expr::Vector(dest),
            Expr::Number(at),
            Expr::Vector(from),
            Expr::Number(start),
        ] => {
            let at = at
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if at > dest.len() || (dest.len() - at) < (from.len() - start) {
                return Err(Error::new("out of range"));
            }

            let from_ref = from.elements.borrow();
            for (i, expr) in from_ref.as_slice()[start..].iter().enumerate() {
                dest.set(i + at, expr.clone())?;
            }

            Ok(Expr::Void())
        }
        [
            Expr::Vector(dest),
            Expr::Number(at),
            Expr::Vector(from),
            Expr::Number(start),
            Expr::Number(end),
        ] => {
            let at = at
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if at > dest.len()
                || end > from.len()
                || start > from.len()
                || (dest.len() - at) < (end - start)
            {
                return Err(Error::new("out of range"));
            }

            let from_ref = from.elements.borrow();
            for (i, expr) in from_ref.as_slice()[start..end].iter().enumerate() {
                dest.set(i + at, expr.clone())?;
            }

            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected a vector, index, and vector")),
    }
}

/// Fill a vector with the given argument.
pub fn vector_fill(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(vec), new_value] => {
            vec.fill(new_value, 0, vec.len())?;
            Ok(Expr::Void())
        }
        [Expr::Vector(vec), new_value, Expr::Number(start)] => {
            if *start == Number::from_usize(vec.len()) {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            vec.fill(new_value, start, vec.len())?;
            Ok(Expr::Void())
        }
        [
            Expr::Vector(vec),
            new_value,
            Expr::Number(start),
            Expr::Number(end),
        ] => {
            let v_len = Number::from_usize(vec.len());
            if *start == v_len && *end == v_len {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            let end = match end.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            vec.fill(new_value, start, end)?;
            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected vector and new value")),
    }
}

/// Append two `Vector` and return resulting `Vector`.
pub fn vector_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(a), Expr::Vector(b)] => {
            let new_vec = a.deep_copy();
            new_vec.append(b.deep_copy());
            Ok(Expr::Vector(new_vec))
        }
        _ => Err(Error::new("expected 2 vectors")),
    }
}

/// Bytevectors

/// Return a newly allocated `ByteVector` filled with all `u8` arguments.
pub fn new_bytevector(args: &[Expr], _: EnvRef) -> Result {
    let vector = ByteVector::new(args.len());
    for (i, arg) in args.iter().enumerate() {
        match arg {
            Expr::Number(n) if n.is_byte() => {
                let byte = n.to_u8().expect("value should have been converted to byte");
                vector.set(i, byte).expect("index should be in bounds");
            }
            _ => return Err(Error::new("expected byte")),
        }
    }
    Ok(Expr::ByteVector(vector))
}

/// Return a newly allocated `ByteVector` of a given size and an optional value. Defaults to 0 if no value is provided.
pub fn make_bytevector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(size), Expr::Number(default)] if size.is_usize() && default.is_byte() => {
            let byte = default
                .to_u8()
                .expect("value should have converted to byte");
            let size = size
                .to_usize()
                .expect("value should have converted to usize");
            let vec = vec![byte.clone(); size];
            Ok(Expr::ByteVector(ByteVector::from(vec.as_slice())))
        }
        _ => Err(Error::new("expected size and optional byte value")),
    }
}

/// Return the length of a `ByteVector`.
pub fn bytevector_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(bv)] => Ok(Expr::Number(Number::from_usize(bv.len()))),
        _ => Err(Error::new("expected bytevector")),
    }
}

/// Return byte at given index.
pub fn bytevector_ref(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(bv), Expr::Number(n_index)] if n_index.is_usize() => {
            let index = n_index
                .to_usize()
                .expect("value should have been converted to usize");
            match bv.get(index) {
                Some(byte) => Ok(Expr::Number(Number::from_u8(byte))),
                None => Err(Error::new("index out of range")),
            }
        }
        _ => Err(Error::new("expected bytevector")),
    }
}

/// Set byte at index to new value.
pub fn bytevector_set(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [
            Expr::ByteVector(bv),
            Expr::Number(n_index),
            Expr::Number(n_byte),
        ] if n_index.is_usize() && n_byte.is_byte() => {
            let index = n_index
                .to_usize()
                .expect("value should have been converted to usize");
            let byte = n_byte
                .to_u8()
                .expect("value should have been converted to byte");
            if index < bv.len() {
                bv.set(index, byte)?;
                return Ok(Expr::Void());
            }
            Err(Error::new("index out of range"))
        }
        _ => Err(Error::new("expected bytevector")),
    }
}

/// Return a newly allocated copy of a `ByteVector`
pub fn bytevector_copy(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(bv)] => Ok(Expr::ByteVector(bv.clone())),
        [Expr::ByteVector(vec), Expr::Number(start)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if start == vec.len() {
                return Ok(Expr::ByteVector(ByteVector::new(0)));
            }

            match vec.sub_bytevector(start, vec.len()) {
                Some(v) => Ok(Expr::ByteVector(v.clone())),
                None => Err(Error::new("out of range")),
            }
        }
        [
            Expr::ByteVector(vec),
            Expr::Number(start),
            Expr::Number(end),
        ] => {
            let v_len = Number::from_usize(vec.len());
            if *start == v_len && *end == v_len {
                return Ok(Expr::Null);
            }
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            match vec.sub_bytevector(start, end) {
                Some(v) => Ok(Expr::ByteVector(v.clone())),
                None => Err(Error::new("out of range")),
            }
        }
        _ => Err(Error::new("expected vector")),
    }
}

/// Copy elements from one `ByteVector` into another using range indexes.
pub fn bytevector_copy_from(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [
            Expr::ByteVector(dest),
            Expr::Number(at),
            Expr::ByteVector(from),
        ] => {
            let at = at
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if at > from.len() {
                return Err(Error::new("out of range"));
            }

            for (i, byte) in from.to_slice().iter().enumerate() {
                dest.set(i + at, *byte)?;
            }

            Ok(Expr::Void())
        }
        [
            Expr::ByteVector(dest),
            Expr::Number(at),
            Expr::ByteVector(from),
            Expr::Number(start),
        ] => {
            let at = at
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if at > dest.len() || (dest.len() - at) < (from.len() - start) {
                return Err(Error::new("out of range"));
            }

            let from_slice = from.to_slice();
            for (i, byte) in from_slice[start..].iter().enumerate() {
                dest.set(i + at, *byte)?;
            }

            Ok(Expr::Void())
        }
        [
            Expr::ByteVector(dest),
            Expr::Number(at),
            Expr::ByteVector(from),
            Expr::Number(start),
            Expr::Number(end),
        ] => {
            let at = at
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            if at > dest.len()
                || end > from.len()
                || start > from.len()
                || (dest.len() - at) < (end - start)
            {
                return Err(Error::new("out of range"));
            }

            let from_slice = from.to_slice();
            for (i, byte) in from_slice[start..end].iter().enumerate() {
                dest.set(i + at, *byte)?;
            }

            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected a bytevector, index, and bytevector")),
    }
}

/// Return a newly allocated `ByteVector` created from concatenating 2 `ByteVector`.
pub fn bytevector_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(a), Expr::ByteVector(b)] => {
            let new_slice = [a.to_slice(), b.to_slice()];
            let bytevector = ByteVector::from(new_slice.concat().as_slice());
            Ok(Expr::ByteVector(bytevector))
        }
        _ => Err(Error::new("expected 2 bytevectors")),
    }
}

// Ports

/// Open textual input file `Port`.
pub fn open_input_file(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(path)] => Ok(Expr::Port(Port::text_input_file(path)?)),
        _ => Err(Error::new("expected file path string")),
    }
}

/// Open textual output file `Port`.
pub fn open_output_file(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(path)] => Ok(Expr::Port(Port::text_output_file(path)?)),
        _ => Err(Error::new("expected file path string")),
    }
}

/// Open textual string input `Port`.
pub fn open_input_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Port(Port::text_input_string(s.clone()))),
        _ => Err(Error::new("expected string")),
    }
}

/// Open textual string output `Port`.
pub fn open_output_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [] => Ok(Expr::Port(Port::text_output_string())),
        _ => Err(Error::new("expected no arguments")),
    }
}

/// Get string output of `Port`.
pub fn get_output_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(Port::TextOutput(p))] => match p.borrow().get_output_string() {
            Some(s) => Ok(Expr::String(s.to_string())),
            None => Err(Error::new("not a string output port")),
        },
        _ => Err(Error::new("expected string output port")),
    }
}

/// Open binary input file `Port`.
pub fn open_binary_input_file(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(path)] => Ok(Expr::Port(Port::binary_input_file(path)?)),
        _ => Err(Error::new("expected file path string")),
    }
}

/// Open binary output file `Port`.
pub fn open_binary_output_file(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(path)] => Ok(Expr::Port(Port::binary_output_file(path)?)),
        _ => Err(Error::new("expected file path string")),
    }
}

/// Open binary input `Port` from bytevector.
pub fn open_input_bytevector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(bv)] => Ok(Expr::Port(Port::binary_input_bytevector(bv)?)),
        _ => Err(Error::new("expected file path string")),
    }
}

/// Open binary output `Port` from bytevector.
pub fn open_output_bytevector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(bv)] => Ok(Expr::Port(Port::binary_output_bytevector(bv)?)),
        _ => Err(Error::new("expected file path string")),
    }
}

/// Return `ByteVector` from bytes read in output port.
pub fn get_output_bytevector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(Port::BinaryOutput(output))] => match output.borrow().deref() {
            BinaryOutputPort::ByteVector(bv) => {
                let new_byte_vec = match bv.get_bytes() {
                    Some(b) => b,
                    None => ByteVector::new(0),
                };
                Ok(Expr::ByteVector(new_byte_vec))
            }
            _ => Err(Error::new(
                "expected binary output port created from bytevector",
            )),
        },
        _ => Err(Error::new("expected file path string")),
    }
}

/// Close `Port`.
pub fn close_port(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(port)] => {
            port.close();
            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected port")),
    }
}

// Ports input

/// Read a char from a `Port`.
/// Defaults to `current-input-port` if port is not specified.
pub fn read_char(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::TextInput(port)) = port {
                let mut port = port.borrow_mut();
                return match port.read_char()? {
                    Some(c) => Ok(Expr::Char(c)),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::Port(Port::TextInput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            match port.read_char()? {
                Some(c) => Ok(Expr::Char(c)),
                None => Ok(Expr::Eof),
            }
        }
        _ => Err(Error::new("expected textual input port")),
    }
}

/// Peek a char from a `Port` without modifying the buffer.
/// Defaults to `current-input-port` if port is not specified.
pub fn peek_char(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::TextInput(port)) = port {
                let mut port = port.borrow_mut();
                return match port.peek_char()? {
                    Some(c) => Ok(Expr::Char(c)),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::Port(Port::TextInput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            match port.peek_char()? {
                Some(c) => Ok(Expr::Char(c)),
                None => Ok(Expr::Eof),
            }
        }
        _ => Err(Error::new("expected textual file input port")),
    }
}

/// Read a string from a `Port`.
/// Defaults to `current-input-port` if port is not specified.
pub fn read_string(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::TextInput(port)) = port {
                let mut port = port.borrow_mut();
                return match port.read_string()? {
                    Some(s) => Ok(Expr::String(s)),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::Port(Port::TextInput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            return match port.read_string()? {
                Some(s) => Ok(Expr::String(s)),
                None => Ok(Expr::Eof),
            };
        }
        _ => Err(Error::new("expected textual file input port")),
    }
}

/// Read a line from a `Port`.
/// Defaults to `current-input-port` if port is not specified.
pub fn read_line(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::TextInput(port)) = port {
                return match port.borrow_mut().read_line()? {
                    Some(line) => Ok(Expr::String(line)),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::Port(Port::TextInput(port))] => match port.borrow_mut().read_line()? {
            Some(line) => Ok(Expr::String(line)),
            None => Ok(Expr::Eof),
        },
        _ => Err(Error::new("expected textual input port")),
    }
}

/// Read a byte from a `Port`.
/// Defaults to `current-input-port` if port is not specified.
pub fn read_u8(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let input_port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::BinaryInput(port)) = input_port {
                let mut port = port.borrow_mut();
                return match port.read_byte()? {
                    Some(byte) => Ok(Expr::Number(Number::from_u8(byte))),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected binary input port"))
        }
        [Expr::Port(Port::BinaryInput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            match port.read_byte() {
                Ok(Some(byte)) => Ok(Expr::Number(Number::from_u8(byte))),
                Ok(None) => Ok(Expr::Eof),
                Err(e) => Err(e),
            }
        }
        _ => Err(Error::new("expected binary file input port")),
    }
}

/// Read a byte from a `Port` without modifying the buffer.
/// Defaults to `current-input-port` if port is not specified.
pub fn peek_u8(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let input_port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::BinaryInput(port)) = input_port {
                let mut port = port.borrow_mut();
                return match port.peek_byte()? {
                    Some(byte) => Ok(Expr::Number(Number::from_u8(byte))),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected binary input port"))
        }
        [Expr::Port(Port::BinaryInput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            match port.peek_byte()? {
                Some(byte) => Ok(Expr::Number(Number::from_u8(byte))),
                None => Ok(Expr::Eof),
            }
        }
        _ => Err(Error::new("expected binary file input port")),
    }
}

/// Read bytes from `Port` into new bytevector.
/// Defaults to `current-input-port` if port is not specified.
pub fn read_bytevector(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [] => {
            let input_port = env
                .borrow()
                .find_param("current-input-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::BinaryInput(port)) = input_port {
                let mut port = port.borrow_mut();
                return match port.read_bytevector(None)? {
                    Some(bv) => Ok(Expr::ByteVector(bv)),
                    None => Ok(Expr::Eof),
                };
            }

            Err(Error::new("expected binary input port"))
        }
        [Expr::Port(Port::BinaryInput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            match port.read_bytevector(None)? {
                Some(bv) => Ok(Expr::ByteVector(bv)),
                None => Ok(Expr::Eof),
            }
        }
        _ => Err(Error::new("expected binary file input port")),
    }
}

/// Read bytes from `Port` into given bytevector.
/// Defaults to `current-input-port` if port is not specified.
pub fn read_into_bytevector(args: &[Expr], env: EnvRef) -> Result {
    // Get bytevector.
    let bv = match args.first() {
        Some(Expr::ByteVector(bv)) => bv,
        _ => return Err(Error::new("expected bytevector as first argument")),
    };

    // Get port.
    let port_expr = match args.get(1) {
        Some(p @ Expr::Port(Port::BinaryInput(_))) => p.clone(),
        Some(_) => return Err(Error::new("expected binary input port")),
        None => env
            .borrow()
            .find_param("current-input-port")
            .ok_or_else(|| Error::new("current-input-port is not initialized"))?,
    };
    let port = match &port_expr {
        Expr::Port(Port::BinaryInput(p)) => p,
        _ => return Err(Error::new("expected binary input port")),
    };

    // Get start and end indexes.
    let start = match args.get(2) {
        Some(Expr::Number(n)) => Some(
            n.to_usize()
                .ok_or_else(|| Error::new("invalid start index"))?,
        ),
        Some(_) => return Err(Error::new("expected number for start index")),
        None => None,
    };
    let end = match args.get(3) {
        Some(Expr::Number(n)) => Some(
            n.to_usize()
                .ok_or_else(|| Error::new("invalid end index"))?,
        ),
        Some(_) => return Err(Error::new("expected number for end index")),
        None => None,
    };

    // Copy bytes from port into bytevector.
    if let Some(input_vec) = port.borrow_mut().read_bytevector(Some(bv.len()))? {
        let bytes_read = bv.copy_into(input_vec, start, end)?;
        return Ok(Expr::Number(Number::from_usize(bytes_read)));
    }
    Ok(Expr::Eof)
}

// Ports output

/// Write `char` to a textual `Port`.
/// Defaults to `current-output-port` if port is not specified.
pub fn write_char(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [Expr::Char(c)] => {
            let port = env
                .borrow()
                .find_param("current-output-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::TextOutput(port)) = port {
                let mut port = port.borrow_mut();
                port.write_char(*c)?;
                return Ok(Expr::Void());
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::Char(ch), Expr::Port(Port::TextOutput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            port.write_char(*ch)?;
            port.flush().map_err(|e| e)?;
            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected binary file output port")),
    }
}

/// Write `String` to textual `Port`.
/// Defaults to `current-output-port` if port is not specified.
pub fn write_string(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => {
            let port = env
                .borrow()
                .find_param("current-output-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;

            if let Expr::Port(Port::TextOutput(port)) = port {
                let mut port = port.borrow_mut();
                port.write_string(s)?;
                return Ok(Expr::Void());
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::String(s), Expr::Port(Port::TextOutput(input))] => {
            let mut port = input.borrow_mut();
            port.write_string(s)?;
            Ok(Expr::Void())
        }
        [
            Expr::String(s),
            Expr::Port(Port::TextOutput(input)),
            Expr::Number(start),
        ] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start >= s.len() {
                return Err(Error::new("index out of range"));
            }
            let mut port = input.borrow_mut();
            port.write_string(&s[start..])?;
            Ok(Expr::Void())
        }
        [
            Expr::String(s),
            Expr::Port(Port::TextOutput(input)),
            Expr::Number(start),
            Expr::Number(end),
        ] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start > end || start >= s.len() || end >= s.len() {
                return Err(Error::new("index out of range"));
            }
            let mut port = input.borrow_mut();
            port.write_string(&s[start..end + 1])?;
            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected string and text output port")),
    }
}

/// Write `u8` to a binary `Port`.
/// Defaults to `current-output-port` if port is not specified.
pub fn write_u8(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [Expr::Number(byte)] => {
            let port = env
                .borrow()
                .find_param("current-output-port")
                .ok_or_else(|| Error::new("current-input-port is not initialized"))?;
            let byte = byte
                .to_u8()
                .ok_or_else(|| Error::new("unable to convert num to byte"))?;

            if let Expr::Port(Port::BinaryOutput(port)) = port {
                let mut port = port.borrow_mut();
                port.write_byte(byte)?;
                return Ok(Expr::Void());
            }

            Err(Error::new("expected textual input port"))
        }
        [Expr::Number(byte), Expr::Port(Port::BinaryOutput(port_ref))] => {
            let mut port = port_ref.borrow_mut();
            let byte = byte
                .to_u8()
                .ok_or_else(|| Error::new("unable to convert num to byte"))?;
            port.write_byte(byte)?;
            port.flush().map_err(|e| e)?;
            Ok(Expr::Void())
        }
        _ => Err(Error::new("expected byte and binary port")),
    }
}

/// Execute procedure with `Port`.
pub fn call_with_port(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [port @ Expr::Port(_), proc @ Expr::Procedure(_)] => {
            let expr = Pair::list(&[proc.clone(), port.clone()]);
            parser::eval(&expr, env)
        }
        _ => Err(Error::new("expected port and procedure")),
    }
}

/// Run procedure on new input port.
pub fn call_with_input_file(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [s @ Expr::String(_), proc @ Expr::Procedure(_)] => {
            let port = open_input_file(&[s.clone()], env.clone())?;
            let expr = Pair::list(&[proc.clone(), port]);
            parser::eval(&expr, env)
        }
        _ => Err(Error::new("expected port and procedure")),
    }
}

/// Run procedure on new input port.
pub fn call_with_output_file(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [s @ Expr::String(_), proc @ Expr::Procedure(_)] => {
            let port = open_output_file(&[s.clone()], env.clone())?;
            let expr = Pair::list(&[proc.clone(), port]);
            parser::eval(&expr, env)
        }
        _ => Err(Error::new("expected port and procedure")),
    }
}

/// Return a new `Eof` object.
pub fn eof_object(_: &[Expr], _: EnvRef) -> Result {
    Ok(Expr::Eof)
}

// Conversion

/// Convert a `Number` into a `String`.
pub fn num_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(num)] => Ok(Expr::String(String::from(num.to_string()))),
        _ => Err(Error::new("expected string")),
    }
}

/// Convert a `String` into a `Number`.
pub fn string_to_num(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(num_str)] => match Number::from_token(&num_str) {
            Ok(n) => Ok(Expr::Number(n)),
            Err(e) => Err(e),
        },
        _ => Err(Error::new("expected string")),
    }
}

/// Convert a `String` into a `Symbol`.
pub fn string_to_symbol(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Symbol(s.to_owned())),
        _ => Err(Error::new("expected string")),
    }
}

/// Convert a `String` into a `Pair` list.
pub fn string_to_list(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => {
            let chars: Vec<Expr> = s.chars().map(|c| Expr::Char(c)).collect::<Vec<Expr>>();
            Ok(Pair::list(chars.as_slice()))
        }
        [Expr::String(s), Expr::Number(start)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start < s.len() {
                let chars: Vec<Expr> = s[start..].chars().map(|c| Expr::Char(c)).collect();
                return Ok(Pair::list(chars.as_slice()));
            }
            Err(Error::new("out of range"))
        }
        [Expr::String(s), Expr::Number(start), Expr::Number(end)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start < s.len() && end < s.len() {
                let chars: Vec<Expr> = s[start..end].chars().map(|c| Expr::Char(c)).collect();
                return Ok(Pair::list(chars.as_slice()));
            }
            Err(Error::new("out of range"))
        }
        _ => Err(Error::new("expected string")),
    }
}

/// Convert a `String` into a `Vector`.
pub fn string_to_vector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Vector(Vector::from_string(s.clone()))),
        [Expr::String(s), Expr::Number(start)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start < s.len() {
                let chars: Vec<Expr> = s[start..].chars().map(|c| Expr::Char(c)).collect();
                return Ok(Expr::Vector(Vector::from(chars.as_slice())));
            }
            Err(Error::new("out of range"))
        }
        [Expr::String(s), Expr::Number(start), Expr::Number(end)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start < s.len() && end < s.len() {
                let chars: Vec<Expr> = s[start..end].chars().map(|c| Expr::Char(c)).collect();
                return Ok(Expr::Vector(Vector::from(chars.as_slice())));
            }
            Err(Error::new("out of range"))
        }
        _ => Err(Error::new("expected string")),
    }
}

/// Convert a `String` into a `ByteVector`.
pub fn string_to_utf8(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::ByteVector(ByteVector::from_string(s.clone()))),
        [Expr::String(s), Expr::Number(start)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            if start < s.len() {
                return Ok(Expr::ByteVector(ByteVector::from_string(
                    s[start..].to_string(),
                )));
            }
            Err(Error::new("out of range"))
        }
        [Expr::String(s), Expr::Number(start), Expr::Number(end)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;
            let len = s.len();
            if start < len && end < len {
                return Ok(Expr::ByteVector(ByteVector::from_string(
                    s[start..end].to_string(),
                )));
            }
            Err(Error::new("out of range"))
        }
        _ => Err(Error::new("expected string")),
    }
}

/// Convert a `String` into a `Symbol`.
pub fn symbol_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Symbol(s)] => Ok(Expr::String(s.to_owned())),
        _ => Err(Error::new("expected string")),
    }
}

/// Convert `Pair` list to `String`.
pub fn list_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(p)] if p.is_list() => p.to_expr_string(),
        [Expr::Pair(list), Expr::Number(n_start)] if list.is_list() => {
            if *n_start == Number::from_usize(list.len()) {
                return Ok(Expr::Null);
            }
            let start = match n_start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match list.sub_list(start, list.len()) {
                Some(list) => match list {
                    Expr::Pair(p) => p.to_expr_string(),
                    _ => unreachable!(),
                },
                None => Err(Error::new("out of range")),
            }
        }
        [Expr::Pair(list), Expr::Number(n_start), Expr::Number(n_end)] if list.is_list() => {
            if *n_start == Number::from_usize(list.len()) {
                return Ok(Expr::Null);
            }
            let start = match n_start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            let end = match n_end.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match list.sub_list(start, end) {
                Some(list) => match list {
                    Expr::Pair(p) => p.to_expr_string(),
                    _ => unreachable!(),
                },
                None => Err(Error::new("out of range")),
            }
        }
        _ => Err(Error::new("expected proper list")),
    }
}

/// Convert `Pair` list to `Vector`.
pub fn list_to_vector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(list)] if list.is_list() => Ok(list.to_expr_vector()),
        [Expr::Pair(list), Expr::Number(n_start)] if list.is_list() => {
            if *n_start == Number::from_usize(list.len()) {
                return Ok(Expr::Null);
            }
            let start = match n_start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match list.sub_list(start, list.len()) {
                Some(list) => Ok(list),
                None => Err(Error::new("out of range")),
            }
        }
        [Expr::Pair(list), Expr::Number(n_start), Expr::Number(n_end)] if list.is_list() => {
            if *n_start == Number::from_usize(list.len()) {
                return Ok(Expr::Null);
            }
            let start = match n_start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            let end = match n_end.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match list.sub_list(start, end) {
                Some(sub_list) => match sub_list {
                    Expr::Pair(sub_list) => Ok(sub_list.to_expr_vector()),
                    _ => Err(Error::Message(
                        "unable to convert sub list to vector".to_string(),
                    )),
                },
                None => Err(Error::new("out of range")),
            }
        }
        _ => Err(Error::new("expected proper list")),
    }
}

/// Convert `Vector` to `Pair` list.
pub fn vector_to_list(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(vec)] => Ok(vec.to_expr_list()),
        [Expr::Vector(vec), Expr::Number(start)] => {
            if *start == Number::from_usize(vec.len()) {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match vec.sub_vector(start, vec.len()) {
                Some(v) => Ok(v.to_expr_list()),
                None => Err(Error::new("out of range")),
            }
        }
        [Expr::Vector(vec), Expr::Number(start), Expr::Number(end)] => {
            let v_len = Number::from_usize(vec.len());
            if *start == v_len && *end == v_len {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            let end = match end.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match vec.sub_vector(start, end) {
                Some(v) => Ok(v.to_expr_list()),
                None => Err(Error::new("out of range")),
            }
        }
        _ => Err(Error::new("expected vector")),
    }
}

/// Convert `Vector` to `String`.
pub fn vector_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(v)] => v.to_expr_string(),
        [Expr::Vector(vec), Expr::Number(start)] => {
            if *start == Number::from_usize(vec.len()) {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match vec.sub_vector(start, vec.len()) {
                Some(v) => Ok(v.to_expr_string()?),
                None => Err(Error::new("out of range")),
            }
        }
        [Expr::Vector(vec), Expr::Number(start), Expr::Number(end)] => {
            let v_len = Number::from_usize(vec.len());
            if *start == v_len && *end == v_len {
                return Ok(Expr::Null);
            }
            let start = match start.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            let end = match end.to_usize() {
                Some(s) => s,
                None => {
                    return Err(Error::Message(
                        "invalid index, expected int or float".to_string(),
                    ));
                }
            };
            match vec.sub_vector(start, end) {
                Some(v) => Ok(v.to_expr_string()?),
                None => Err(Error::new("out of range")),
            }
        }
        _ => Err(Error::new("expected vector")),
    }
}

/// Convert `ByteVector` into `String`. Converts non-printable UTF-8 values into their hex value.
pub fn utf8_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(b)] => {
            let hex_str = b
                .to_slice()
                .iter()
                .map(|byte| ByteVector::utf8_to_hex_str(*byte))
                .collect::<String>();

            Ok(Expr::String(hex_str))
        }
        [Expr::ByteVector(b), Expr::Number(start)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let len = b.len();
            if start > len {
                return Err(Error::new("index out of bounds"));
            }

            let hex_str = b
                .to_slice()
                .iter()
                .skip(start)
                .map(|byte| ByteVector::utf8_to_hex_str(*byte))
                .collect::<String>();

            Ok(Expr::String(hex_str))
        }
        [Expr::ByteVector(b), Expr::Number(start), Expr::Number(end)] => {
            let start = start
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let end = end
                .to_usize()
                .ok_or_else(|| Error::new("invalid index, expected int or float"))?;

            let len = b.len();
            if start > len || end > len {
                return Err(Error::new("index out of bounds"));
            }

            let hex_str = b
                .to_slice()
                .iter()
                .skip(start)
                .take(end - start)
                .map(|byte| ByteVector::utf8_to_hex_str(*byte))
                .collect::<String>();

            Ok(Expr::String(hex_str))
        }
        _ => Err(Error::new("expected bytevector")),
    }
}

// Predicates

/// Returns true if arg is a number.
pub fn is_number(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a real number.
pub fn is_real(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Float(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a rational number.
pub fn is_rational(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Rational(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a complex number.
pub fn is_complex(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is an integer.
pub fn is_integer(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Int(_))] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if number is even.
pub fn is_even(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(n)] => {
            let remainder = (n.clone() % Number::Int(Small(2)))?;
            Ok(Expr::Boolean(remainder == Number::Int(Small(0))))
        }
        _ => Err(Error::new("expected one argument")),
    }
}

/// Returns true if number is odd.
pub fn is_odd(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(n)] => {
            let remainder = (n.clone() % Number::Int(Small(2)))?;
            Ok(Expr::Boolean(remainder == Number::Int(Small(1))))
        }
        _ => Err(Error::new("expected a number")),
    }
}

/// Returns true if number is exact.
pub fn is_exact(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Int(_))] | [Expr::Number(Number::Rational(_))] => {
            Ok(Expr::Boolean(true))
        }
        [Expr::Number(Number::Float(_))] | [Expr::Number(Number::Complex(_))] => {
            Ok(Expr::Boolean(false))
        }
        _ => Err(Error::new("expected a number")),
    }
}

/// Returns false if number is inexact.
pub fn is_inexact(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Float(_))] | [Expr::Number(Number::Complex(_))] => {
            Ok(Expr::Boolean(true))
        }
        [Expr::Number(Number::Int(_))] | [Expr::Number(Number::Rational(_))] => {
            Ok(Expr::Boolean(false))
        }
        _ => Err(Error::new("expected a number")),
    }
}

/// Returns true if number is an exact integer.
pub fn is_exact_integer(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Int(_))] => Ok(Expr::Boolean(true)),
        [Expr::Number(_)] => Ok(Expr::Boolean(false)),
        _ => Err(Error::new("expected a number")),
    }
}

/// Returns true if arg is a symbol.
pub fn is_symbol(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Symbol(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a string.
pub fn is_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a character.
pub fn is_char(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Char(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if char is alphabetic.
pub fn is_char_alphabetic(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_alphabetic())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is numeric.
pub fn is_char_numeric(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_numeric())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is whitespace.
pub fn is_char_whitespace(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_whitespace())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is uppercase.
pub fn is_char_uppercase(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_uppercase())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if char is lowercase.
pub fn is_char_lowercase(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        return match arg {
            Expr::Char(c) => Ok(Expr::Boolean(c.is_lowercase())),
            _ => Ok(Expr::Boolean(false)),
        };
    }
    let msg = format!("expected 1 argument, got {}", args.len());
    Err(Error::Message(msg))
}

/// Returns true if arg is a boolean.
pub fn is_boolean(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Boolean(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a list.
pub fn is_list(args: &[Expr], _: EnvRef) -> Result {
    let result = match args {
        [Expr::Pair(p)] => p.is_list(),
        [_] => false,
        _ => {
            return Err(Error::Message(format!(
                "expected 1 argument, got {}",
                args.len()
            )));
        }
    };
    Ok(Expr::Boolean(result))
}

/// Return true is arg is pair.
pub fn is_pair(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

pub fn is_vector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a procedure.
pub fn is_procedure(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Procedure(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if arg is a `ByteVector`.
pub fn is_bytevector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::ByteVector(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if arg is an output `Port`.
pub fn is_output_port(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(p)] => Ok(Expr::Boolean(p.is_output())),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if arg is an input `Port`.
pub fn is_input_port(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(p)] => Ok(Expr::Boolean(p.is_input())),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}
/// Return true if arg is a textual `Port`.
pub fn is_textual_port(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(p)] => Ok(Expr::Boolean(p.is_textual())),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if arg is a binary `Port`.
pub fn is_binary_port(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(p)] => Ok(Expr::Boolean(p.is_binary())),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if `Port` has a readable character.
pub fn is_char_ready(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(Port::TextInput(p))] => match p.borrow_mut().peek_char().map_err(|e| e) {
            Ok(None) => Ok(Expr::Boolean(false)),
            Ok(_) => Ok(Expr::Boolean(true)),
            Err(e) => Err(e),
        },
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if `Port` has a readable byte.
pub fn is_byte_ready(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(Port::BinaryInput(p))] => match p.borrow_mut().peek_byte().map_err(|e| e) {
            Ok(None) => Ok(Expr::Boolean(false)),
            Ok(_) => Ok(Expr::Boolean(true)),
            Err(e) => Err(e),
        },
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if input port is open.
pub fn is_input_port_open(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(p)] => Ok(Expr::Boolean(p.is_open())),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if output port is open.
pub fn is_output_port_open(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Port(p)] => Ok(Expr::Boolean(p.is_open())),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Return true if arg is `Eof`.
pub fn is_eof_object(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Eof] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

/// Returns true if arg is a parameter object.
pub fn is_parameter(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Parameter(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
}

// Parameters

/// Apply a converter function to a value.
fn apply_converter(converter: &Expr, value: &Expr, env: EnvRef) -> Result {
    match converter {
        Expr::Procedure(f) => f(&[value.clone()], env),
        Expr::Closure(c) => apply_lambda(c, vec![value.clone()]),
        _ => Err(Error::new("converter must be a procedure")),
    }
}

/// Create a new parameter object.
/// (make-parameter init) or (make-parameter init converter)
pub fn make_parameter(args: &[Expr], env: EnvRef) -> Result {
    match args {
        [init] => {
            let id = next_parameter_id();
            let param = Parameter::new(id, None);

            // Store initial value in the environment's params
            env.borrow_mut().set_param(&id.to_string(), init);

            Ok(Expr::Parameter(param))
        }
        [init, converter] => {
            // Validate converter is callable
            match converter {
                Expr::Procedure(_) | Expr::Closure(_) => {}
                _ => return Err(Error::new("make-parameter: converter must be a procedure")),
            }

            let id = next_parameter_id();

            // Apply converter to initial value
            let converted_init = apply_converter(converter, init, env.clone())?;

            let param = Parameter::new(id, Some(converter.clone()));

            // Store converted initial value
            env.borrow_mut().set_param(&id.to_string(), &converted_init);

            Ok(Expr::Parameter(param))
        }
        _ => Err(Error::new("make-parameter: expected 1 or 2 arguments")),
    }
}
