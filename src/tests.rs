// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-11

#[allow(dead_code)]
#[test]
fn test_add_valid_input() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 1)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}

#[allow(dead_code)]
#[test]
fn test_sub_valid_input() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(- 1 1)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(0))));
}

#[allow(dead_code)]
#[test]
fn test_mult_valid_input() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(* 1 2)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}

#[allow(dead_code)]
#[test]
fn test_div_valid_input() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(/ 4 2)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}

#[allow(dead_code)]
#[test]
fn test_multiline_nested_expression() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1\n  (* 2\n     2)\n)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(5))));
}
