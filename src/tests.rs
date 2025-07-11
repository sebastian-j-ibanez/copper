use crate::types::Env;

#[allow(dead_code)]
#[test]
fn test_add_valid_input() {
    use crate::{parser::parse_eval, types::Expr};
    let env = &mut Env::default_env();
    let input = "(+ 1 1)".to_string();
    let result = parse_eval(input, env);
    assert!(matches!(result, Ok(Expr::Number(2))));
}
