// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-11-11

use crate::env::EnvRef;
use crate::error::Error;
use crate::types::number::IntVariant::Small;
use crate::types::{Expr, Number, Result, format_list};
use crate::{io, parser};
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

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
            Expr::List(l) => {
                print!("{}", format_list(l, "", false));
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
            Expr::List(l) => {
                println!("{}", format_list(l, "", false));
            }
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

// Lists

/// Make a new list with an unbound number of expressions.
pub fn new_list(args: &[Expr], _: EnvRef) -> Result {
    let mut list = Vec::new();
    for arg in args {
        list.push(arg.to_owned());
    }

    Ok(Expr::List(Rc::new(RefCell::new(list))))
}

/// Append 2 lists together.
pub fn list_append(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(list_a_ref), Expr::List(list_b_ref)] => {
            let list_a = list_a_ref.borrow_mut();
            let list_b = list_b_ref.borrow_mut();
            let result: Vec<Expr> = list_a.iter().chain(list_b.iter()).cloned().collect();
            Ok(Expr::List(Rc::new(RefCell::new(result))))
        }
        _ => Err(Error::Message("expected 2 lists".to_string())),
    }
}

/// Get length of list.
pub fn list_length(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(l)] => Ok(Expr::Number(Number::from_usize(l.borrow().len()))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Get first element from list or pair.
pub fn car(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.as_ref().0.clone()),
        [Expr::List(l)] => Ok(Expr::List(Rc::new(RefCell::new(
            l.borrow().iter().cloned().take(1).collect::<Vec<Expr>>(),
        )))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Return list without first element or return second element from pair.
pub fn cdr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Pair(pair)] => Ok(pair.as_ref().1.clone()),
        [Expr::List(l)] => Ok(Expr::List(Rc::new(RefCell::new(
            l.borrow().iter().cloned().skip(1).collect::<Vec<Expr>>(),
        )))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Get second item.
pub fn cadr(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(l)] => {
            if l.borrow().len() < 2 {
                return Err(Error::Message(
                    "expected list of at least 2 items".to_string(),
                ));
            }

            Ok(Expr::from(l.borrow()[1].clone()))
        }
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// Reverse list.
pub fn list_reverse(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(l)] => Ok(Expr::List(Rc::new(RefCell::new(
            l.borrow().iter().cloned().rev().collect::<Vec<Expr>>(),
        )))),
        _ => Err(Error::Message("expected list".to_string())),
    }
}

/// WIP! Sets the first element in a list or pair.
pub fn set_car(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::List(list_ref), new_value] => {
            let mut list = list_ref.borrow_mut();
            if list.len() <= 0 {
                list.push(new_value.clone());
            }
            Ok(Expr::Void())
        }
        [Expr::Pair(pair), new_value] => Ok(Expr::Pair(Box::new((
            new_value.clone(),
            pair.as_ref().1.clone(),
        )))),
        [a, b] => Ok(Expr::Pair(Box::new((a.clone(), b.clone())))),
        _ => Err(Error::Message("expected 2 arguments".to_string())),
    }
}

// Pairs

/// Construct a new pair from 2 expressions.
pub fn cons(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [a, Expr::List(list)] => {
            let new_list = std::iter::once(a.clone())
                .chain(list.borrow().iter().cloned())
                .collect();
            Ok(Expr::List(Rc::new(RefCell::new(new_list))))
        }
        [a, b] => Ok(Expr::Pair(Box::new((a.clone(), b.clone())))),
        _ => Err(Error::Message("expected 2 arguments".to_string())),
    }
}

// Conversion

/// Convert a number into a string.
pub fn num_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Number(num)] => Ok(Expr::String(String::from(num.to_string()))),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a string into a number.
pub fn string_to_num(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(num_str)] => match Number::from_token(&num_str) {
            Ok(n) => Ok(Expr::Number(n)),
            Err(e) => Err(e),
        },
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a string into a symbol.
pub fn string_to_symbol(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => Ok(Expr::Symbol(s.to_owned())),
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a string into a list of `Expr::Char`
pub fn string_to_list(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::String(s)] => {
            return Ok(Expr::List(Rc::new(RefCell::new(
                s.chars().map(|c| Expr::Char(c)).collect::<Vec<Expr>>(),
            ))));
        }
        _ => Err(Error::Message("expected string".to_string())),
    }
}

/// Convert a string into a symbol.
pub fn symbol_to_string(args: &[Expr], _: EnvRef) -> Result {
    match args {
        [Expr::Symbol(s)] => Ok(Expr::String(s.to_owned())),
        _ => Err(Error::Message("expected string".to_string())),
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
    match args {
        [Expr::List(_)] => Ok(Expr::Boolean(true)),
        [_] => Ok(Expr::Boolean(false)),
        _ => Err(Error::Message(format!(
            "expected 1 argument, got {}",
            args.len()
        ))),
    }
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
