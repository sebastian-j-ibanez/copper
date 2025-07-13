#[allow(dead_code)]
#[test]
fn test_add_valid_input() {
    use crate::{parser::parse_eval, types::{Env, Expr}};
    let env = &mut Env::default_env();
    let input = "(+ 1 1)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}

#[allow(dead_code)]
#[test]
fn test_sub_valid_input() {
    use crate::{parser::parse_eval, types::{Env, Expr}};
    let env = &mut Env::default_env();
    let input = "(- 1 1)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(0))));
}

#[allow(dead_code)]
#[test]
fn test_mult_valid_input() {
    use crate::{parser::parse_eval, types::{Env, Expr}};
    let env = &mut Env::default_env();
    let input = "(* 1 2)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}

#[allow(dead_code)]
#[test]
fn test_div_valid_input() {
    use crate::{parser::parse_eval, types::{Env, Expr}};
    let env = &mut Env::default_env();
    let input = "(/ 4 2)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}
