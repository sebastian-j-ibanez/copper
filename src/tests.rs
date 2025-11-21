// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-11

//! Unit test module.

#[test]
fn test_template() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Void());
    assert!(matches!(result, _expected));
}

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
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(list 1 2 3)".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
    ];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
}

#[test]
fn test_new_list_empty() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Pair};
    let env = Env::standard_env();
    let input = "(list)".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![Expr::Void()];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
}

#[test]
fn test_cons() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(cons 1 (list 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
    ];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
}

#[test]
fn test_list_append() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(append (list 1 2) (list 3 4))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
        Expr::Number(Number::from_i64(4)),
    ];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
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
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(car (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![Expr::Number(Number::from_i64(1))];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
}

#[test]
fn test_cdr() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(cdr (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env.clone());
    let expected_values = vec![
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(3)),
    ];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
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

// I/O Functions

#[test]
fn test_display() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(display \"hello\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Void());
    assert!(matches!(result, _expected));
}

#[test]
fn test_newline() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(newline)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Void());
    assert!(matches!(result, _expected));
}

#[test]
fn test_print() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(print \"hello\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Void());
    assert!(matches!(result, _expected));
}

#[test]
fn test_println() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(println \"hello\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Void());
    assert!(matches!(result, _expected));
}

#[test]
fn test_pretty_print() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(pretty-print 42)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Void());
    assert!(matches!(result, _expected));
}

// Math Functions

#[test]
fn test_modulo() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(modulo 10 3)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(1)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_ceil() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(ceil 3.2)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_f64(4.0)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_floor() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(floor 3.8)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_f64(3.0)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_min() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(min 3 1 4 1 5)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(1)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_max() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(max 3 1 4 1 5)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(5)));
    assert!(matches!(result, _expected));
}

// String Functions

#[test]
fn test_string_length() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(string-length \"hello\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_usize(5)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_make_string_empty() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(make-string)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String(String::new()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_make_string_from_char() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(make-string #\\a)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("a".to_string()));
    assert!(matches!(result, _expected));
}

// Boolean Functions

#[test]
fn test_not_true() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(not #t)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(false));
    assert!(matches!(result, _expected));
}

#[test]
fn test_not_false() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(not #f)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_and_all_true() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(and #t #t #t)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_and_one_false() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(and #t #f #t)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(false));
    assert!(matches!(result, _expected));
}

#[test]
fn test_or_all_false() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(or #f #f #f)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(false));
    assert!(matches!(result, _expected));
}

#[test]
fn test_or_one_true() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(or #f #t #f)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

// List Functions

#[test]
fn test_cadr() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(cadr (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_reverse() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(reverse (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env);
    let expected_values = vec![
        Expr::Number(Number::from_i64(3)),
        Expr::Number(Number::from_i64(2)),
        Expr::Number(Number::from_i64(1)),
    ];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
}

// Pair Functions

#[test]
fn test_cons_pair() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Number, types::Pair};
    let env = Env::standard_env();
    let input = "(cons 1 2)".to_string();
    let result = parse_and_eval(input, env);
    let _expected = Pair::cons((
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
    ));
    assert!(matches!(result, Ok(_expected)));
}

#[test]
fn test_car_pair() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(car (cons 1 2))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(1)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_cdr_pair() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(cdr (cons 1 2))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

// Conversion Functions

#[test]
fn test_number_to_string() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(number->string 42)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("42".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_string_to_number() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr, types::Number};
    let env = Env::standard_env();
    let input = "(string->number \"42\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(42)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_string_to_symbol() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(string->symbol \"foo\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Symbol("foo".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_symbol_to_string() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(symbol->string 'foo)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("foo".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_string_to_list() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr, types::Pair};
    let env = Env::standard_env();
    let input = "(string->list \"hello\")".to_string();
    let result = parse_and_eval(input, env);
    let expected_values = vec![
        Expr::Char('h'),
        Expr::Char('e'),
        Expr::Char('l'),
        Expr::Char('l'),
        Expr::Char('o'),
    ];
    let _expected = Pair::list(expected_values.as_slice());
    assert!(matches!(result, Ok(_expected)));
}

#[test]
fn test_string_to_list_empty() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(string->list \"\")".to_string();
    let result = parse_and_eval(input, env);
    assert!(matches!(result, Ok(Expr::Null)));
}

#[test]
fn test_string_to_vector() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let input = "(string->vector \"abc\")".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, crate::types::Expr::Vector(_)));
    }
}

#[test]
fn test_list_to_string() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list->string (list #\\h #\\i))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("hi".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_to_string_with_start() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list->string (list #\\h #\\e #\\l #\\l #\\o) 1)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("ello".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_to_string_with_start_and_end() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list->string (list #\\h #\\e #\\l #\\l #\\o) 1 4)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("ell".to_string()));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_to_vector() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list->vector (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, Expr::Vector(_)));
    }
}

#[test]
fn test_list_to_vector_with_start() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list->vector (list 1 2 3 4) 1)".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, Expr::Pair(_)));
    }
}

#[test]
fn test_list_to_vector_with_start_and_end() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list->vector (list 1 2 3 4 5) 1 4)".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, Expr::Vector(_)));
    }
}

#[test]
fn test_vector_to_list() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(vector->list (vector 1 2 3))".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, Expr::Pair(_)));
    }
}

#[test]
fn test_vector_to_list_with_start() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(vector->list (vector 1 2 3 4) 2)".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, Expr::Vector(_)));
    }
}

#[test]
fn test_vector_to_list_with_start_and_end() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(vector->list (vector 1 2 3 4 5) 1 3)".to_string();
    let result = parse_and_eval(input, env);
    assert!(result.is_ok());
    if let Ok(expr) = result {
        assert!(matches!(expr, Expr::Vector(_)));
    }
}

#[test]
fn test_vector_to_string() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(vector->string (vector #\\a #\\b #\\c))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::String("abc".to_string()));
    assert!(matches!(result, _expected));
}

// Predicate Functions

#[test]
fn test_symbol_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(symbol? 'foo)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_string_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(string? \"hello\")".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_char_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(char? #\\a)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_char_alphabetic() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(char-alphabetic? #\\a)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_char_numeric() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(char-numeric? #\\5)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_char_whitespace() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(char-whitespace? #\\space)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_char_uppercase() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(char-uppercase? #\\A)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_char_lowercase() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(char-lowercase? #\\a)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_boolean_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(boolean? #t)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(list? (list 1 2 3))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_pair_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(pair? (cons 1 2))".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_procedure_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(procedure? +)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_number_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(number? 42)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_real_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(real? 3.14)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_rational_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(rational? 1/2)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_complex_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(complex? 1+2i)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_integer_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(integer? 42)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_even_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(even? 4)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_odd_predicate() {
    use crate::{env::Env, error::Error, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "(odd? 3)".to_string();
    let result = parse_and_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_pair_get_element() {
    use crate::types::{Expr, Pair};
    let pair = Pair::cons((Expr::Boolean(true), Expr::Boolean(false)));
    let result = pair.get(0);
    let _expected = Some(Expr::Boolean(true));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_get_first_element() {
    use crate::types::{Expr, Number, Pair};
    let pair = Pair::cons((
        Expr::Number(Number::from_i64(0)),
        Expr::Pair(Pair::cons((Expr::Number(Number::from_i64(1)), Expr::Null))),
    ));
    let result = pair.get(1);
    let _expected = Some(Expr::Number(Number::from_i64(1)));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_get_second_element() {
    use crate::types::{Expr, Number, Pair};
    let pair = Pair::cons((
        Expr::Number(Number::from_i64(0)),
        Expr::Pair(Pair::cons((Expr::Number(Number::from_i64(1)), Expr::Null))),
    ));
    let result = pair.get(1);
    let _expected = Some(Expr::Boolean(false));
    assert!(matches!(result, _expected));
}

#[test]
fn test_list_get_invalid_element() {
    use crate::types::{Expr, Number, Pair};
    let pair = Pair::cons((
        Expr::Number(Number::from_i64(0)),
        Expr::Pair(Pair::cons((Expr::Number(Number::from_i64(1)), Expr::Null))),
    ));
    let result = pair.get(2);
    assert!(matches!(result, Some(Expr::Null)));
}

#[test]
fn test_list_get_last_element() {
    use crate::types::{Expr, Number, Pair};
    let pair = Pair::cons((
        Expr::Number(Number::from_i64(0)),
        Expr::Pair(Pair::cons((
            Expr::Number(Number::from_i64(1)),
            Expr::Pair(Pair::cons((Expr::Number(Number::from_i64(2)), Expr::Null))),
        ))),
    ));
    let result = pair.get(4);
    assert!(matches!(result, Some(Expr::Null)));
}

#[test]
fn test_create_list() {
    use crate::types::{Expr, Number, Pair};
    let expr = [
        Expr::Number(Number::from_i64(0)),
        Expr::Number(Number::from_i64(1)),
        Expr::Number(Number::from_i64(2)),
    ];
    let list = Pair::list(&expr);
    assert!(matches!(list, Expr::Pair(_)));
}

#[test]
fn test_empty_list_format() {
    use crate::types::{Expr, Pair};
    let empty = Pair::list(&[]);
    assert_eq!(format!("{}", empty), "()");
    assert!(matches!(empty, Expr::Null));
}
