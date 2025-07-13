use std::collections::HashMap;

use crate::error::Error;
use crate::parser;
use crate::types::Expr;

#[derive(Debug)]
pub struct Env {
    pub data: HashMap<String, Expr>,
}

impl Env {
    pub fn default_env() -> Env {
        let mut data: HashMap<String, Expr> = HashMap::new();
        data.insert(
            "+".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, Error> {
                let numbers = parser::parse_number_list(args)?;
                let sum: i32 = numbers.iter().fold(0, |sum, a| sum + a);
                Ok(Expr::Number(sum))
            }),
        );
        data.insert(
            "-".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, Error> {
                let numbers = parser::parse_number_list(args)?;

                let first = *numbers
                    .first()
                    .ok_or(Error::Message("expected at least one number".to_string()))?;

                let sum_of_rest = numbers[1..].iter().fold(0, |sum, a| sum + a);

                Ok(Expr::Number(first - sum_of_rest))
            }),
        );
        data.insert(
            "*".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, Error> {
                let numbers = parser::parse_number_list(args)?;

                let first = *numbers
                    .first()
                    .ok_or(Error::Message("expected at least one number".to_string()))?;

                let sum_of_rest = numbers[1..].iter().fold(0, |sum, a| sum + a);

                Ok(Expr::Number(first * sum_of_rest))
            }),
        );
        data.insert(
            "/".to_string(),
            Expr::Func(|args: &[Expr]| -> Result<Expr, Error> {
                let numbers = parser::parse_number_list(args)?;

                let first = *numbers
                    .first()
                    .ok_or(Error::Message("expected at least one number".to_string()))?;

                let sum_of_rest = numbers[1..].iter().fold(0, |sum, a| sum + a);

                Ok(Expr::Number(first / sum_of_rest))
            }),
        );

        Env { data }
    }
}
