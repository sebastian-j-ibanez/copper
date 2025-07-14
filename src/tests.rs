// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-11

use num_traits::ToPrimitive;
use crate::types::Number;

#[allow(dead_code)]
#[test]
fn test_add_valid_input() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 1)".to_string();
    let result: Result<Expr, Error> = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_sub_valid_input() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(- 1 1)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(0)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_mult_valid_input() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(* 1 2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_div_valid_input() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(/ 4 2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(2)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_multiline_nested_expression() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1\n  (* 2\n     2)\n)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(5)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_add_multiple_integers() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 2 3 4)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(10)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_add_integer_and_real() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 2.5)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Real(3.5)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_add_rational_and_integer() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_rational::Rational64;
    let env = &mut Env::default_env();
    let input = "(+ 1/2 1)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(Rational64::new(3, 2))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_add_complex_and_real() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_complex::Complex64;
    let env = &mut Env::default_env();
    let input = "(+ 2+3i 1.0)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Complex(Complex64::new(3.0, 3.0))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_sub_single_argument() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(- 5)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(-5)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_sub_mixed_types() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_rational::Rational64;
    let env = &mut Env::default_env();
    let input = "(- 5 1/2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(Rational64::new(9, 2))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_sub_complex_numbers() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_complex::Complex64;
    let env = &mut Env::default_env();
    let input = "(- 2+3i 1+1i)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Complex(Complex64::new(1.0, 2.0))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_mul_multiple_integers() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(* 2 3 4)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_i64(24)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_mul_rational_and_real() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(* 1/2 4.0)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Real(2.0)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_mul_complex_and_integer() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_complex::Complex64;
    let env = &mut Env::default_env();
    let input = "(* 2+3i 2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Complex(Complex64::new(4.0, 6.0))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_div_inexact_integer_division() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_rational::Rational64;
    let env = &mut Env::default_env();
    let input = "(/ 7 2)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(Rational64::new(7, 2))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_div_single_argument_integer() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_rational::Rational64;
    let env = &mut Env::default_env();
    let input = "(/ 5)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(Rational64::new(1, 5))));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_div_by_zero_exact() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
    };
    let env = &mut Env::default_env();
    let input = "(/ 5 0)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Err(Error::Message(_))));
}

#[allow(dead_code)]
#[test]
fn test_div_by_zero_inexact() {
    use crate::{
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    let env = &mut Env::default_env();
    let input = "(/ 5.0 0.0)".to_string();
    let result = parse_eval(input, env);
    if let Ok(Expr::Number(Number::Real(val))) = result {
        assert!(val.is_infinite());
    } else {
        panic!("_expected infinite real result for division by inexact zero");
    }
}

#[allow(dead_code)]
#[test]
fn test_add_bignums() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_bigint::BigInt;
    let env = &mut Env::default_env();
    let input = "(+ 9223372036854775807 1)".to_string();
    let result = parse_eval(input, env);
    let _expected_bignum = BigInt::from(9223372036854775807_i64) + BigInt::from(1);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_bigint(_expected_bignum)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_sub_bignums() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_bigint::BigInt;
    let env = &mut Env::default_env();
    let input = "(- 9223372036854775808 1)".to_string();
    let result = parse_eval(input, env);
    let _expected_bignum = BigInt::from(9223372036854775808_u64) - BigInt::from(1);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_bigint(_expected_bignum)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_mul_bignums() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_bigint::BigInt;
    let env = &mut Env::default_env();
    let input = "(* 1000000000000000000 1000)".to_string();
    let result = parse_eval(input, env);
    let _expected_bignum = BigInt::from(1_000_000_000_000_000_000_i64) * BigInt::from(1000);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_bigint(_expected_bignum)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_div_bignums_exact() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_bigint::BigInt;
    let env = &mut Env::default_env();
    let input = "(/ 1000000000000000000000 1000)".to_string();
    let result = parse_eval(input, env);
    let _expected_bignum = BigInt::from(1_000_000_000_000_000_000_000_i128) / BigInt::from(1000);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::from_bigint(_expected_bignum)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_div_bignums_inexact() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_rational::Rational64;
    let env = &mut Env::default_env();
    let input = "(/ 9223372036854775807 2)".to_string();
    let result = parse_eval(input, env);
    let _expected_rational = Rational64::new(9223372036854775807_i64, 2);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Rational(_expected_rational)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_add_mixed_large_numbers() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_bigint::BigInt;
    use num_complex::Complex64;
    use num_rational::Rational64;

    let env = &mut Env::default_env();
    let input = "(+ 1000000000000000000000 3.5 1/2 2+3i)".to_string();
    let result = parse_eval(input, env);

    let big_int_val = BigInt::from(1_000_000_000_000_000_000_000_i128);
    let real_val = 3.5;
    let rational_val = Rational64::new(1, 2);
    let complex_val = Complex64::new(2.0, 3.0);

    let sum_real_parts = big_int_val.to_f64().unwrap() + real_val + rational_val.to_f64().unwrap() + complex_val.re;
    let sum_imag_parts = complex_val.im;

    let _expected_complex = Complex64::new(sum_real_parts, sum_imag_parts);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Complex(_expected_complex)));
    assert!(matches!(result, _expected));
}

#[allow(dead_code)]
#[test]
fn test_invalid_number_format() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
    };
    let env = &mut Env::default_env();
    let input = "(+ 1 'abc')".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Err(Error::Message(_))));
}

#[allow(dead_code)]
#[test]
fn test_div_by_exact_zero_rational() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
    };
    let env = &mut Env::default_env();
    let input = "(/ 1/2 0)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Err(Error::Message(_))));
}

#[allow(dead_code)]
#[test]
fn test_complex_pure_imaginary_parsing() {
    use crate::{
        error::Error,
        env::Env,
        parser::parse_eval,
        types::Expr,
    };
    use num_complex::Complex64;
    let env = &mut Env::default_env();
    let input = "(+ 1 5i)".to_string();
    let result = parse_eval(input, env);
    let _expected: Result<Expr, Error> = Ok(Expr::Number(Number::Complex(Complex64::new(1.0, 5.0))));
    assert!(matches!(result, _expected));
}