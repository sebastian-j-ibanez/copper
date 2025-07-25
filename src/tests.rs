// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-11

//! Unit test module.

#[test]
fn test_add_string_result() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 1)".to_string();
    if let Ok(result) = parse_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(2));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_add_number_result() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 1)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_sub_string_result() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(- 1 1)".to_string();
    if let Ok(result) = parse_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(0));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_sub_number_result() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(- 1 1)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(0)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_mult_string_result() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(* 1 2)".to_string();
    if let Ok(result) = parse_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(2));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_mult_number_result() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(* 1 2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_div_string_result() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(/ 4 2)".to_string();
    if let Ok(result) = parse_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(2));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_div_number_result() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(/ 4 2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_multiline_nested_string_result() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1\n  (* 2\n     2)\n)".to_string();
    if let Ok(result) = parse_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(5));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_multiline_nested_number_result() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
        types::Number,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1\n  (* 2\n     2)\n)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(5)));
    assert!(matches!(result, _expected));
}
