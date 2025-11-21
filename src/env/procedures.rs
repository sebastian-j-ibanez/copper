// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-11-11

use crate::env::EnvRef;
use crate::error::Error;
use crate::types::number::IntVariant::Small;
use crate::types::{Expr, Number, Pair, PairIter, Result, Vector, format_pair};
use crate::{io, parser};
use std::ops::{Add, Div, Mul, Sub};

// I/O

/// Display raw expression in stdout.
pub fn display(args: &[Expr], _: EnvRef) -> Result {
    match args.first() {
        Some(arg) => {
            print!("{}", arg);
            Ok(Expr::Void())
        }
        _ => Err(Error::Message("expected 1 valid expression".to_string())),
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

    Err(Error::Message("expected 1 valid expression".to_string()))
}

/// Print formatted value of expression in stdout with a newline.
pub fn println(args: &[Expr], _: EnvRef) -> Result {
    if let Some(arg) = args.first() {
        match arg {
            Expr::String(s) => println!("{}", s),
            Expr::Char(c) => println!("{}", c),
            // Expr::List(l) => {
            //     println!("{}", format_list(l, "", false));
            // }
            _ => println!("{}", arg),
        }
        return Ok(Expr::Void());
    }

    Err(Error::Message("expected 1 valid expression".to_string()))
}

/// Evaluate the contents of a file.
pub fn load_file(args: &[Expr], env: EnvRef) -> Result {
    let file = match args.first() {
        Some(Expr::String(f)) => f,
        _ => return Err(Error::Message("expected a string path".to_string())),
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
        None => return Err(Error::Message("expected ".to_string())),
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
        return Err(Error::Message("expected at least one number".to_string()));
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
        return Err(Error::Message("expected at least one number".to_string()));
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
        _ => Err(Error::Message("expected 2 numbers".to_string())),
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
        _ => Err(Error::Message("expected 2 numbers".to_string())),
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
        _ => Err(Error::Message("expected 1 number".to_string())),
    }
}

/// Round number up to the nearest integer.
pub fn ceil(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => {
            Err(Error::Message("unable to round complex number".to_string()))
        }
        [Expr::Number(n)] => {
            if let Some(result) = n.to_f64() {
                return Ok(Expr::Number(Number::from_f64(result.ceil())));
            }
            Err(Error::Message(
                "unable to convert number to float".to_string(),
            ))
        }
        _ => Err(Error::Message("expected real number".to_string())),
    }
}

/// Round number down to the nearest integer.
pub fn floor(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Complex(_))] => {
            Err(Error::Message("unable to round complex number".to_string()))
        }
        [Expr::Number(n)] => {
            if let Some(result) = n.to_f64() {
                return Ok(Expr::Number(Number::from_f64(result.floor())));
            }
            Err(Error::Message(
                "unable to convert number to float".to_string(),
            ))
        }
        _ => Err(Error::Message("expected real number".to_string())),
    }
}

/// Return smallest real number from arguments.
pub fn min(args: &[Expr], _: EnvRef) -> Result {
    if args.is_empty() {
        return Err(Error::Message("expected real numbers".to_string()));
    }

    let mut min: Option<Number> = None;

    for arg in args {
        match arg {
            Expr::Number(current) => match current {
                Number::Complex(_) => {
                    return Err(Error::Message("expected real numbers".to_string()));
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
                return Err(Error::Message("expected real numbers".to_string()));
            }
        }
    }

    Ok(Expr::Number(min.unwrap()))
}

/// Return largest real number from arguments.
pub fn max(args: &[Expr], _: EnvRef) -> Result {
    if args.is_empty() {
        return Err(Error::Message("expected real numbers".to_string()));
    }

    let mut min: Option<Number> = None;

    for arg in args {
        match arg {
            Expr::Number(current) => match current {
                Number::Complex(_) => {
                    return Err(Error::Message("expected real numbers".to_string()));
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
                return Err(Error::Message("expected real numbers".to_string()));
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
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Create either a new empty string or a string from a char.
pub fn new_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [] => Ok(Expr::String(String::new())),
        [Expr::Char(c)] => Ok(Expr::String(String::from(*c))),
        _ => Err(Error::Message("expected character".to_string())),
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
        _ => Err(Error::Message("expected string".to_string())),
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
        _ => Err(Error::Message("expected string".to_string())),
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
        _ => Err(Error::Message("expected 2 arguments".to_string())),
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
        _ => Err(Error::Message("expected 2 lists".to_string())),
    }
}

/// Get length of list.
pub fn list_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(p)] => Ok(Expr::Number(Number::from_usize(p.len()))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Return first element from `Pair`.
pub fn car(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.car()),
        _ => Err(Error::Message("expected pair".to_string())),
    }
}

/// Return second element from `Pair`.
pub fn cdr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.cdr()),
        _ => Err(Error::Message("expected pair".to_string())),
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
        _ => Err(Error::Message("expected list".to_string())),
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
        _ => Err(Error::Message("expected list".to_string())),
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
                _ => Err(Error::Message("invalid index".to_string())),
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
            _ => Err(Error::Message("invalid index".to_string())),
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
        _ => Err(Error::Message("expected vector".to_string())),
    }
}

// Conversion

/// Convert a `Number` into a `String`.
pub fn num_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(num)] => Ok(Expr::String(String::from(num.to_string()))),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a `String` into a `Number`.
pub fn string_to_num(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(num_str)] => match Number::from_token(&num_str) {
            Ok(n) => Ok(Expr::Number(n)),
            Err(e) => Err(e),
        },
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a `String` into a `Symbol`.
pub fn string_to_symbol(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Symbol(s.to_owned())),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a `String` into a `Pair` list.
pub fn string_to_list(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => {
            let chars: Vec<Expr> = s.chars().map(|c| Expr::Char(c)).collect::<Vec<Expr>>();
            Ok(Pair::list(chars.as_slice()))
        }
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a `String` into a `Vector`.
pub fn string_to_vector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Vector(Vector::from_string(s.clone()))),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a `String` into a `Symbol`.
pub fn symbol_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Symbol(s)] => Ok(Expr::String(s.to_owned())),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// WIP! Convert `Pair` list to `String`.
pub fn list_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(p)] if p.is_list() => p.to_string(),
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
                    Expr::Pair(p) => p.to_string(),
                    _ => unreachable!(),
                },
                None => Err(Error::Message("out of range".to_string())),
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
                    Expr::Pair(p) => p.to_string(),
                    _ => unreachable!(),
                },
                None => Err(Error::Message("out of range".to_string())),
            }
        }
        _ => Err(Error::Message("expected proper list".to_string())),
    }
}

/// Convert `Pair` list to `Vector`.
pub fn list_to_vector(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(list)] if list.is_list() => Ok(list.to_vector()),
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
                None => Err(Error::Message("out of range".to_string())),
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
                    Expr::Pair(sub_list) => Ok(sub_list.to_vector()),
                    _ => Err(Error::Message(
                        "unable to convert sub list to vector".to_string(),
                    )),
                },
                None => Err(Error::Message("out of range".to_string())),
            }
        }
        _ => Err(Error::Message("expected proper list".to_string())),
    }
}

/// Convert `Vector` to `Pair` list.
pub fn vector_to_list(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(vec)] => Ok(vec.to_list()),
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
                Some(v) => Ok(Expr::Vector(v)),
                None => Err(Error::Message("out of range".to_string())),
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
                Some(v) => Ok(Expr::Vector(v)),
                None => Err(Error::Message("out of range".to_string())),
            }
        }
        _ => Err(Error::Message("expected vector".to_string())),
    }
}

/// Convert `Vector` to `String`.
pub fn vector_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Vector(v)] => v.to_string(),
        _ => Err(Error::Message("expected vector".to_string())),
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
        _ => Err(Error::Message("expected one argument".to_string())),
    }
}

/// Returns true if number is odd.
pub fn is_odd(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(n)] => {
            let remainder = (n.clone() % Number::Int(Small(2)))?;
            Ok(Expr::Boolean(remainder == Number::Int(Small(1))))
        }
        _ => Err(Error::Message("expected a number".to_string())),
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
        _ => Err(Error::Message("expected a number".to_string())),
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
        _ => Err(Error::Message("expected a number".to_string())),
    }
}

/// Returns true if number is an exact integer.
pub fn is_exact_integer(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(Number::Int(_))] => Ok(Expr::Boolean(true)),
        [Expr::Number(_)] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message("expected a number".to_string())),
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
