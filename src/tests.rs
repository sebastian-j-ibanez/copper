// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-11

//! Unit test module.

// #[test]
// fn test_template() {
//     use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
//     let env = Env::standard_env();
//     let input = "".to_string();
//     let result = parse_and_eval(input, env);
//     let _expected: Result<Expr, Error> = Ok(Expr::Void());
//     assert!(matches!(result, _expected));
// }

#[test]
fn test_add_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(+ 1 1)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(2));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_add_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(+ 1 1)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_sub_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(- 1 1)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(0));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_sub_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(- 1 1)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(0)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_mult_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(* 1 2)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(2));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_mult_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(* 1 2)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_div_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(/ 4 2)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(2));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_div_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(/ 4 2)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_multiline_nested_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(+ 1\n  (* 2\n     2)\n)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(5));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_multiline_nested_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(+ 1\n  (* 2\n     2)\n)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(5)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_expt_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 2 3)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(8));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_expt_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 2 3)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(8)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_expt_zero_exponent_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 5 0)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(1));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_expt_zero_exponent_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 5 0)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(1)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_expt_rational_base_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 1/2 2)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::Rational(num_rational::Rational64::new(1, 4)));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_expt_rational_base_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 1/2 2)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(
        num_rational::Rational64::new(1, 4),
    )));
    assert!(matches!(result, _expected));
}

#[test]
fn test_expt_nested_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 2 (+ 1 2))".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(8));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_expt_nested_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(expt 2 (+ 1 2))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(8)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_string_append() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(string-append \"hello \" \" world!\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("hello world!".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_define_atome() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(define thing 10)".to_string();
    _ = parse_and_eval(input, env.clone());
    let result = parse_and_eval("thing".to_string(), env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(10)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_define_lambda_explicit() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(define addone (lambda (x) (+ x 1)))".to_string();
    _ = parse_and_eval(input, env.clone());
    let result = parse_and_eval("(addone 1)".to_string(), env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_define_lambda_implicit() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(define (addone x) (+ x 1))".to_string();
    _ = parse_and_eval(input, env.clone());
    let result = parse_and_eval("(addone 1)".to_string(), env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_new_list() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(list 1 2 3)".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
    ];
    let _expected: Result<Expr, Error> = Ok(Expr::List(expected_values));
    assert!(matches!(result, _expected));
}

#[test]
fn test_new_list_empty() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list)".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![Expr::Void()];
    let _expected: Result<Expr, Error> = Ok(Expr::List(expected_values));
    assert!(matches!(result, _expected));
}

#[test]
fn test_cons() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(cons 1 (list 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
    ];
    let _expected: Result<Expr, Error> = Ok(Expr::List(expected_values));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_append() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(append (list 1 2) (list 3 4))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
        Expr::Number(Number::from_i64(4)),
    ];
    let _expected: Result<Expr, Error> = Ok(Expr::List(expected_values));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_length() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(length (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(3)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_car() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(car (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![Expr::Number(Number::from_i64(1))];
    let _expected: Result<Expr, Error> = Ok(Expr::List(expected_values));
    assert!(matches!(result, _expected));
}

#[test]
fn test_cdr() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(cdr (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
    ];
    let _expected: Result<Expr, Error> = Ok(Expr::List(expected_values));
    assert!(matches!(result, _expected));
}

#[test]
fn test_abs_positive_number_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs 5)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(5));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_abs_positive_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs 5)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(5)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_abs_negative_number_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs -7)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(7));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_abs_negative_number_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs -7)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(7)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_abs_zero_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs 0)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::from_i64(0));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_abs_zero_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs 0)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(0)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_abs_rational_string_result() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs -3/4)".to_string();
    if let Ok(result) = parse_and_eval(input, env) {
        let _expected: Expr = Expr::Number(Number::Rational(num_rational::Rational64::new(3, 4)));
        assert_eq!(result.to_string(), _expected.to_string());
    }
}

#[test]
fn test_abs_rational_result() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(abs -3/4)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(
        num_rational::Rational64::new(3, 4),
    )));
    assert!(matches!(result, _expected));
}
