// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

//! Simple error struct.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(m) => write!(f, "{}", m),
        }
    }
}
