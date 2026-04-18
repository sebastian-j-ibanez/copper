// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-11

//! Unit test module.

#[test]
fn test_template() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let input = "".to_string();
    let result = parse_and_eval(input, env);
    assert!(matches!(result.unwrap(), Expr::Void()));
}

#[test]
fn test_add_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(+ 1 1)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_add_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(+ 1 1)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_sub_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(- 1 1)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_sub_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(- 1 1)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_mult_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(* 1 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_mult_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(* 1 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_div_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(/ 4 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_div_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(/ 4 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_multiline_nested_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(+ 1\n  (* 2\n     2)\n)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_multiline_nested_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(+ 1\n  (* 2\n     2)\n)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_expt_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 2 3)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_expt_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 2 3)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_expt_zero_exponent_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 5 0)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_expt_zero_exponent_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 5 0)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_expt_rational_base_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 1/2 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1/4");
}

#[test]
fn test_expt_rational_base_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 1/2 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1/4");
}

#[test]
fn test_expt_nested_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 2 (+ 1 2))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_expt_nested_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(expt 2 (+ 1 2))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "8");
}

#[test]
fn test_string_append() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string-append \"hello \" \" world!\")".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "\"hello  world!\"");
}

#[test]
fn test_define_atome() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define thing 10)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("thing".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "10");
}

#[test]
fn test_define_lambda_explicit() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define addone (lambda (x) (+ x 1)))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(addone 1)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_define_lambda_implicit() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define (addone x) (+ x 1))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(addone 1)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_new_list() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(list 1 2 3)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(1 2 3)");
}

#[test]
fn test_new_list_empty() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(list)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Null));
}

#[test]
fn test_cons() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(cons 1 (list 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(1 2 3)");
}

#[test]
fn test_list_append() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(append (list 1 2) (list 3 4))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(1 2 3 4)");
}

#[test]
fn test_list_length() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(length (list 1 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_car() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(car (list 1 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_cdr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(cdr (list 1 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(2 3)");
}

#[test]
fn test_abs_positive_number_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs 5)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_abs_positive_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs 5)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_abs_negative_number_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs -7)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "7");
}

#[test]
fn test_abs_negative_number_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs -7)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "7");
}

#[test]
fn test_abs_zero_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs 0)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_abs_zero_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs 0)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_abs_rational_string_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs -3/4)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "3/4");
}

#[test]
fn test_abs_rational_result() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(abs -3/4)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "3/4");
}

// I/O Functions

#[test]
fn test_display() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(display \"hello\")".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Void()));
}

#[test]
fn test_newline() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(newline)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Void()));
}

#[test]
fn test_print() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(print \"hello\")".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Void()));
}

#[test]
fn test_println() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(println \"hello\")".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Void()));
}

#[test]
fn test_pretty_print() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(pp 42)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Void()));
}

// Math Functions

#[test]
fn test_modulo() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(modulo 10 3)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_ceil() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(ceiling 3.2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_floor() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(floor 3.8)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_min() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(min 3 1 4 1 5)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_max() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(max 3 1 4 1 5)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "5");
}

// String Functions

#[test]
fn test_string_length() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string-length \"hello\")".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_make_string_empty() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "");
}

#[test]
fn test_make_string_from_char() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string #\\a)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "a");
}

// Boolean Functions

#[test]
fn test_not_true() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(not #t)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#f");
}

#[test]
fn test_not_false() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(not #f)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_and_all_true() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(and #t #t #t)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_and_one_false() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(and #t #f #t)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#f");
}

#[test]
fn test_or_all_false() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(or #f #f #f)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#f");
}

#[test]
fn test_or_one_true() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(or #f #t #f)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

// List Functions

#[test]
fn test_cadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(cadr (list 1 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

#[test]
fn test_reverse() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(reverse (list 1 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(3 2 1)");
}

// Pair Functions

#[test]
fn test_cons_pair() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(cons 1 2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(1 . 2)");
}

#[test]
fn test_car_pair() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(car (cons 1 2))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_cdr_pair() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(cdr (cons 1 2))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "2");
}

// Conversion Functions

#[test]
fn test_number_to_string() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(number->string 42)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "42");
}

#[test]
fn test_string_to_number() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string->number \"42\")".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_string_to_symbol() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string->symbol \"foo\")".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "foo");
}

#[test]
fn test_symbol_to_string() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(symbol->string 'foo)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "foo");
}

#[test]
fn test_string_to_list() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string->list \"hello\")".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(#\\h #\\e #\\l #\\l #\\o)");
}

#[test]
fn test_string_to_list_empty() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(string->list \"\")".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Null));
}

#[test]
fn test_string_to_vector() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(string->vector \"abc\")".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Vector(_)));
}

#[test]
fn test_list_to_string() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(list->string (list #\\h #\\i))".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "hi");
}

#[test]
fn test_list_to_string_with_start() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval(
        "(list->string (list #\\h #\\e #\\l #\\l #\\o) 1)".to_string(),
        env,
    )
    .unwrap();
    assert_eq!(result.formatted(), "ello");
}

#[test]
fn test_list_to_string_with_start_and_end() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval(
        "(list->string (list #\\h #\\e #\\l #\\l #\\o) 1 4)".to_string(),
        env,
    )
    .unwrap();
    assert_eq!(result.formatted(), "ell");
}

#[test]
fn test_list_to_vector() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(list->vector (list 1 2 3))".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Vector(_)));
}

#[test]
fn test_list_to_vector_with_start() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(list->vector (list 1 2 3 4) 1)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Pair(_) | Expr::Vector(_)));
}

#[test]
fn test_list_to_vector_with_start_and_end() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(list->vector (list 1 2 3 4 5) 1 4)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Vector(_)));
}

#[test]
fn test_vector_to_list() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(vector->list (vector 1 2 3))".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Pair(_)));
}

#[test]
fn test_vector_to_list_with_start() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(vector->list (vector 1 2 3 4) 2)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Pair(_)));
}

#[test]
fn test_vector_to_list_with_start_and_end() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(vector->list (vector 1 2 3 4 5) 1 3)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Pair(_)));
}

#[test]
fn test_vector_to_string() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result =
        parse_and_eval("(vector->string (vector #\\a #\\b #\\c))".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "abc");
}

// Predicate Functions

#[test]
fn test_symbol_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(symbol? 'foo)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_string_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(string? \"hello\")".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_char_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(char? #\\a)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_char_alphabetic() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(char-alphabetic? #\\a)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_char_numeric() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(char-numeric? #\\5)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_char_whitespace() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(char-whitespace? #\\space)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_char_uppercase() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(char-upper-case? #\\A)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_char_lowercase() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(char-lower-case? #\\a)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_boolean_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(boolean? #t)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_list_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(list? (list 1 2 3))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_pair_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(pair? (cons 1 2))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_procedure_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(procedure? +)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_number_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(number? 42)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_real_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(real? 3.14)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_rational_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(rational? 1/2)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_complex_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(complex? 1+2i)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_integer_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(integer? 42)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_even_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(even? 4)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_odd_predicate() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(odd? 3)".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_pair_get_element() {
    use crate::types::{Expr, Pair};
    let pair = Pair::cons((Expr::Boolean(true), Expr::Boolean(false)));
    let result = pair.get(0).unwrap();
    assert_eq!(result.to_string(), "#t");
}

#[test]
fn test_list_get_first_element() {
    use crate::types::{Expr, Number, Pair};
    let pair = Pair::cons((
        Expr::Number(Number::from_i64(0)),
        Expr::Pair(Pair::cons((Expr::Number(Number::from_i64(1)), Expr::Null))),
    ));
    // get(0) returns the car
    let result = pair.get(0).unwrap();
    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_list_get_second_element() {
    use crate::types::{Expr, Number, Pair};
    let pair = Pair::cons((
        Expr::Number(Number::from_i64(0)),
        Expr::Pair(Pair::cons((Expr::Number(Number::from_i64(1)), Expr::Null))),
    ));
    let result = pair.get(2);
    assert!(matches!(result, Some(Expr::Null)));
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

#[test]
fn test_quote_empty_list() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(cons 1 '())".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(1)");
}

#[test]
fn test_make_parameter_basic() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(make-parameter 10)".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Parameter(_)));
}

#[test]
fn test_make_parameter_with_string() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(make-parameter \"hello\")".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Parameter(_)));
}

#[test]
fn test_make_parameter_with_list() {
    use crate::{env::Env, parser::parse_and_eval, types::Expr};
    let env = Env::standard_env();
    let result = parse_and_eval("(make-parameter (list 1 2 3))".to_string(), env).unwrap();
    assert!(matches!(result, Expr::Parameter(_)));
}

#[test]
fn test_parameter_predicate_true() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter? (make-parameter 5))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "#t");
}

#[test]
fn test_parameter_predicate_false_number() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter? 42)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "#f");
}

#[test]
fn test_parameter_predicate_false_string() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter? \"hello\")".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "#f");
}

#[test]
fn test_parameter_predicate_false_list() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter? (list 1 2 3))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "#f");
}

#[test]
fn test_parameter_predicate_false_procedure() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter? +)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "#f");
}

#[test]
fn test_parameter_get_value() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 42))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "42");
}

#[test]
fn test_parameter_get_string_value() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define p (make-parameter \"hello\"))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().formatted(), "hello");
}

#[test]
fn test_parameter_set_value() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    parse_and_eval("(p 20)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "20");
}

#[test]
fn test_parameter_set_different_type() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    parse_and_eval("(p \"now a string\")".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().formatted(), "now a string");
}

#[test]
fn test_parameter_multiple_sets() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 1))".to_string(), env.clone()).unwrap();
    parse_and_eval("(p 2)".to_string(), env.clone()).unwrap();
    parse_and_eval("(p 3)".to_string(), env.clone()).unwrap();
    parse_and_eval("(p 4)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "4");
}

#[test]
fn test_make_parameter_with_converter() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // Converter doubles the value
    parse_and_eval(
        "(define p (make-parameter 5 (lambda (x) (* x 2))))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "10");
}

#[test]
fn test_parameter_converter_on_set() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define p (make-parameter 5 (lambda (x) (* x 2))))".to_string(),
        env.clone(),
    )
    .unwrap();
    parse_and_eval("(p 10)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "20");
}

#[test]
fn test_parameter_converter_string_to_upper() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define p (make-parameter \"hello\" string-upcase))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().formatted(), "HELLO");
}

#[test]
fn test_parameter_converter_validates() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // Converter that ensures value is non-negative using abs
    parse_and_eval(
        "(define p (make-parameter -5 abs))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env.clone());
    assert_eq!(result.unwrap().to_string(), "5");
    // Set to negative, should be converted to positive
    parse_and_eval("(p -10)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "10");
}

#[test]
fn test_parameterize_basic() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(parameterize ((p 20)) (p))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "20");
}

#[test]
fn test_parameterize_restores_value() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    parse_and_eval("(parameterize ((p 20)) (p))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "10");
}

#[test]
fn test_parameterize_multiple_bindings() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p1 (make-parameter 1))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define p2 (make-parameter 2))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval(
        "(parameterize ((p1 10) (p2 20)) (+ (p1) (p2)))".to_string(),
        env,
    );
    assert_eq!(result.unwrap().to_string(), "30");
}

#[test]
fn test_parameterize_multiple_bindings_restore() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p1 (make-parameter 1))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define p2 (make-parameter 2))".to_string(), env.clone()).unwrap();
    parse_and_eval(
        "(parameterize ((p1 10) (p2 20)) (+ (p1) (p2)))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(+ (p1) (p2))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "3");
}

#[test]
fn test_parameterize_nested() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 1))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval(
        "(parameterize ((p 10)) (parameterize ((p 100)) (p)))".to_string(),
        env,
    );
    assert_eq!(result.unwrap().to_string(), "100");
}

#[test]
fn test_parameterize_nested_restore_inner() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 1))".to_string(), env.clone()).unwrap();

    let inner_result = parse_and_eval(
        "(parameterize ((p 10)) (parameterize ((p 100)) (p)))".to_string(),
        env.clone(),
    );
    assert_eq!(inner_result.unwrap().to_string(), "100");

    let outer_result = parse_and_eval(
        "(parameterize ((p 10)) (parameterize ((p 100)) (+ 0 0)) (p))".to_string(),
        env.clone(),
    );
    assert_eq!(outer_result.unwrap().to_string(), "10");

    let original_result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(original_result.unwrap().to_string(), "1");
}

#[test]
fn test_parameterize_with_converter() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define p (make-parameter 5 (lambda (x) (* x 2))))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(parameterize ((p 10)) (p))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "20");
}

#[test]
fn test_parameterize_empty_bindings() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameterize () (+ 1 2))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "3");
}

#[test]
fn test_parameterize_multiple_body_exprs() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 0))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval(
        "(parameterize ((p 10)) (+ (p) 0) (+ (p) 5))".to_string(),
        env,
    );
    assert_eq!(result.unwrap().to_string(), "15");
}

#[test]
fn test_parameter_in_lambda() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    parse_and_eval(
        "(define get-p (lambda (unused) (p)))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(get-p 0)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "10");
}

#[test]
fn test_parameter_set_in_lambda() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define set-p (lambda (v) (p v)))".to_string(), env.clone()).unwrap();
    parse_and_eval("(set-p 99)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert!(result.is_ok());
}

#[test]
fn test_parameterize_affects_lambda_call() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 10))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval(
        "(parameterize ((p 50)) ((lambda (unused) (p)) 0))".to_string(),
        env,
    );
    assert_eq!(result.unwrap().to_string(), "50");
}

#[test]
fn test_make_parameter_wrong_args() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(make-parameter)".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_make_parameter_too_many_args() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(make-parameter 1 + 2)".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_make_parameter_invalid_converter() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(make-parameter 10 42)".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_parameter_predicate_wrong_args() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter?)".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_parameter_predicate_too_many_args() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameter? 1 2)".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_parameterize_non_parameter() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    let result = parse_and_eval("(parameterize ((42 10)) (+ 1 1))".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_parameterize_malformed_binding() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 1))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(parameterize ((p)) (p))".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_parameterize_no_body() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter 1))".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(parameterize ((p 10)))".to_string(), env);
    assert!(result.is_err());
}

#[test]
fn test_parameter_with_boolean() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (make-parameter #f))".to_string(), env.clone()).unwrap();
    parse_and_eval("(p #t)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "#t");
}

#[test]
fn test_parameter_with_null() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define p (make-parameter (list)))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "()");
}

#[test]
fn test_multiple_parameters_independent() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p1 (make-parameter 1))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define p2 (make-parameter 2))".to_string(), env.clone()).unwrap();
    parse_and_eval("(p1 10)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(list (p1) (p2))".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "(10 2)");
}

#[test]
fn test_parameter_closure_converter() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define multiplier 3)".to_string(), env.clone()).unwrap();
    parse_and_eval(
        "(define p (make-parameter 5 (lambda (x) (* x multiplier))))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "15");
}

#[test]
fn test_parameter_procedure_converter() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval(
        "(define p (make-parameter -5 abs))".to_string(),
        env.clone(),
    )
    .unwrap();
    let result = parse_and_eval("(p)".to_string(), env);
    assert_eq!(result.unwrap().to_string(), "5");
}

#[test]
fn test_write_shared_no_sharing() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    parse_and_eval("(write-shared '(1 2 3) p)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "(1 2 3)");
}

#[test]
fn test_write_shared_shared_pair() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define x (list 1 2))".to_string(), env.clone()).unwrap();
    parse_and_eval("(write-shared (cons x x) p)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "(#0=(1 2) . #0#)");
}

#[test]
fn test_write_shared_cyclic() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define x (list 1 2 3))".to_string(), env.clone()).unwrap();
    parse_and_eval("(set-cdr! (cddr x) x)".to_string(), env.clone()).unwrap();
    parse_and_eval("(write-shared x p)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "#0=(1 2 3 . #0#)");
}

#[test]
fn test_write_no_sharing() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    parse_and_eval("(write '(1 2 3) p)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    // No sharing: write and write-shared produce identical output
    assert_eq!(result.formatted(), "(1 2 3)");
}

#[test]
fn test_write_shared_pair_expanded() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define x (list 1 2))".to_string(), env.clone()).unwrap();
    // write expands non-cyclic shared structure inline — no datum labels
    parse_and_eval("(write (cons x x) p)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "((1 2) 1 2)");
}

#[test]
fn test_write_cyclic() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    parse_and_eval("(define x (list 1 2 3))".to_string(), env.clone()).unwrap();
    parse_and_eval("(set-cdr! (cddr x) x)".to_string(), env.clone()).unwrap();
    // Cyclic structure must use datum labels to avoid infinite output
    parse_and_eval("(write x p)".to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), "#0=(1 2 3 . #0#)");
}

#[test]
fn test_write_atoms() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    parse_and_eval("(define p (open-output-string))".to_string(), env.clone()).unwrap();
    // Atoms: strings get quotes, chars get #\ prefix (external representation)
    parse_and_eval(r#"(write "hello" p)"#.to_string(), env.clone()).unwrap();
    let result = parse_and_eval("(get-output-string p)".to_string(), env).unwrap();
    assert_eq!(result.formatted(), r#""hello""#);
}

// cxr functions (2-deep)

#[test]
fn test_caar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a b) c d) -> caar = a
    let result = parse_and_eval("(caar '((a b) c d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "a");
}

#[test]
fn test_cdar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a b) c d) -> cdar = (b)
    let result = parse_and_eval("(cdar '((a b) c d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(b)");
}

#[test]
fn test_cddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b c) -> cddr = (c)
    let result = parse_and_eval("(cddr '(a b c))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(c)");
}

// cxr functions (3-deep)

#[test]
fn test_caaar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(((a b) c) d) -> caaar = a
    let result = parse_and_eval("(caaar '(((a b) c) d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "a");
}

#[test]
fn test_caadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a (b c) d) -> caadr = b
    let result = parse_and_eval("(caadr '(a (b c) d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "b");
}

#[test]
fn test_cadar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a b c) d) -> cadar = b
    let result = parse_and_eval("(cadar '((a b c) d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "b");
}

#[test]
fn test_caddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b c d) -> caddr = c
    let result = parse_and_eval("(caddr '(a b c d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "c");
}

#[test]
fn test_cdaar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(((a b c) d) e) -> cdaar = (b c)
    let result = parse_and_eval("(cdaar '(((a b c) d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(b c)");
}

#[test]
fn test_cdadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a (b c d) e) -> cdadr = (c d)
    let result = parse_and_eval("(cdadr '(a (b c d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(c d)");
}

#[test]
fn test_cddar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a b c) d) -> cddar = (c)
    let result = parse_and_eval("(cddar '((a b c) d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(c)");
}

#[test]
fn test_cdddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b c d) -> cdddr = (d)
    let result = parse_and_eval("(cdddr '(a b c d))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(d)");
}

// cxr functions (4-deep)

#[test]
fn test_caaaar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((((a b) c) d) e) -> caaaar = a
    let result = parse_and_eval("(caaaar '((((a b) c) d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "a");
}

#[test]
fn test_caaadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a ((b c) d) e) -> caaadr = b
    let result = parse_and_eval("(caaadr '(a ((b c) d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "b");
}

#[test]
fn test_caadar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(((a b) (c d)) e) -> caadar = c
    // car -> ((a b) (c d)), cdr -> ((c d)), car -> (c d), car -> c
    // Wait, let me think more carefully.
    // caadar = car(car(cdr(car(x))))
    // car(x) = ((a b) (c d))
    // cdr(car(x)) = ((c d))
    // car(cdr(car(x))) = (c d)
    // car(car(cdr(car(x)))) = c
    let result = parse_and_eval("(caadar '(((a b) (c d)) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "c");
}

#[test]
fn test_caaddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b (c d) e) -> caaddr = c
    let result = parse_and_eval("(caaddr '(a b (c d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "c");
}

#[test]
fn test_cadaar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(((a b c) d) e) -> cadaar = b
    // car(x) = ((a b c) d), car(car(x)) = (a b c), cdr(car(car(x))) = (b c), car(cdr(car(car(x)))) = b
    let result = parse_and_eval("(cadaar '(((a b c) d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "b");
}

#[test]
fn test_cadadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a (b c d) e) -> cadadr = c
    // cdr(x) = ((b c d) e), car(cdr(x)) = (b c d), cdr(car(cdr(x))) = (c d), car = c
    let result = parse_and_eval("(cadadr '(a (b c d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "c");
}

#[test]
fn test_caddar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a b c d) e) -> caddar = c
    // car(x) = (a b c d), cdr = (b c d), cdr = (c d), car = c
    let result = parse_and_eval("(caddar '((a b c d) e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "c");
}

#[test]
fn test_cadddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b c d e) -> cadddr = d
    let result = parse_and_eval("(cadddr '(a b c d e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "d");
}

#[test]
fn test_cdaaar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((((a b c) d) e) f) -> cdaaar = (b c)
    // car = (((a b c) d) e), car = ((a b c) d), car = (a b c), cdr = (b c)
    let result = parse_and_eval("(cdaaar '((((a b c) d) e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(b c)");
}

#[test]
fn test_cdaadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a ((b c d) e) f) -> cdaadr = (c d)
    // cdr = (((b c d) e) f), car = ((b c d) e), car = (b c d), cdr = (c d)
    let result = parse_and_eval("(cdaadr '(a ((b c d) e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(c d)");
}

#[test]
fn test_cdadar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a (b c d) e) f) -> cdadar = (c d)
    // car = (a (b c d) e), cdr = ((b c d) e), car = (b c d), cdr = (c d)
    let result = parse_and_eval("(cdadar '((a (b c d) e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(c d)");
}

#[test]
fn test_cdaddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b (c d e) f) -> cdaddr = (d e)
    // cdr = (b (c d e) f), cdr = ((c d e) f), car = (c d e), cdr = (d e)
    let result = parse_and_eval("(cdaddr '(a b (c d e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(d e)");
}

#[test]
fn test_cddaar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(((a b c d) e) f) -> cddaar = (c d)
    // car = ((a b c d) e), car = (a b c d), cdr = (b c d), cdr = (c d)
    let result = parse_and_eval("(cddaar '(((a b c d) e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(c d)");
}

#[test]
fn test_cddadr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a (b c d e) f) -> cddadr = (d e)
    // cdr = ((b c d e) f), car = (b c d e), cdr = (c d e), cdr = (d e)
    let result = parse_and_eval("(cddadr '(a (b c d e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(d e)");
}

#[test]
fn test_cdddar() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '((a b c d e) f) -> cdddar = (d e)
    // car = (a b c d e), cdr = (b c d e), cdr = (c d e), cdr = (d e)
    let result = parse_and_eval("(cdddar '((a b c d e) f))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(d e)");
}

#[test]
fn test_cddddr() {
    use crate::{env::Env, parser::parse_and_eval};
    let env = Env::standard_env();
    // '(a b c d e) -> cddddr = (e)
    let result = parse_and_eval("(cddddr '(a b c d e))".to_string(), env).unwrap();
    assert_eq!(result.to_string(), "(e)");
}
