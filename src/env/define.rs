// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-03

//! Define functions and variables.

use crate::env::Env;
use crate::{error::Error, types::Expr};

pub fn define(args: &[Expr], env: &mut Env) -> Result<Expr, Error> {
    let mut iter = args.iter();
    let name = match iter.next() {
        Some(Expr::Symbol(n)) => n.to_owned(),
        _ => return Err(Error::Message("expected symbol name".to_string())),
    };

    let data = match iter.next() {
        Some(e) => e.clone(),
        _ => return Err(Error::Message("invalid data".to_string())),
    };

    env.data.insert(name, data);

    // println!("{}", name);
    // for arg in args.iter() {
    //     println!("{}", arg);
    // }

    Ok(Expr::Void())
}
