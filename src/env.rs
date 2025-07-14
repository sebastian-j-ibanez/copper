use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

use crate::error::Error;
use crate::parser;
use crate::types::{Expr, Number};

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
        Env { data }
    }
}

fn add(args: &[Expr]) -> Result<Expr, Error> {
    let numbers = parser::parse_number_list(args)?;
    let initial_sum = Number::from_i64(0);
    let sum = numbers
        .into_iter()
        .try_fold(initial_sum, |current_sum, num| current_sum.add(num))?;
    Ok(Expr::Number(sum))
}

fn sub(args: &[Expr]) -> Result<Expr, Error> {
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

fn mult(args: &[Expr]) -> Result<Expr, Error> {
    let numbers = parser::parse_number_list(args)?;
    if numbers.is_empty() {
        return Err(Error::Message("expected at least one number".to_string()));
    }
    let initial_value: Number = Number::from_i64(1);
    let product = numbers
        .into_iter()
        .try_fold(initial_value, |current_product, num| current_product.mul(num))?;
    Ok(Expr::Number(product))
}

fn div(args: &[Expr]) -> Result<Expr, Error> {
    let numbers = parser::parse_number_list(args)?;
    if numbers.is_empty() {
        return Err(Error::Message("expected at least one number".to_string()));
    }
    let mut iter = numbers.clone().into_iter();
    let first_num = iter.next().unwrap();
    if iter.next().is_none() {
        let one = Number::from_i64(1);
        let result = one.div(first_num).map_err(Error::from)?;
        Ok(Expr::Number(result))
    } else {
        let mut iter = numbers.into_iter();
        let result = iter.try_fold(first_num, |current_quotient, num| current_quotient.div(num))?;
        Ok(Expr::Number(result))
    }
}
