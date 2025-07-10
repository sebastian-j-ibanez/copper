// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

pub const BOOLEAN_TRUE_STR: &str = "#t";
pub const BOOLEAN_FALSE_STR: &str = "#f";

#[derive(Debug, PartialEq)]
pub enum Type{
    Number(i32),
    Boolean(String),
    String(String),
    Symbol(String),
}

impl Type {
    /// Create Type from &str, returns None if there is no match.
    pub fn from_str(s: &str) -> Option<Type> {
        if let Ok(num) = s.parse::<i32>() {
            return Some(Type::Number(num))
        }

        if Type::is_boolean(s) {
            return Some(Type::Boolean(s.to_string()))
        } 

        if Type::is_string(s) {
            return Some(Type::String(s.to_string()))
        }

        if Type::is_symbol(s) {
            return Some(Type::Symbol(s.to_string()))
        }

        None
    }

    pub fn to_str(&self) -> String {
        match self {
            Type::Number(n) => n.to_string(),
            Type::Boolean(b) => b.to_string(),
            Type::String(s) => s.to_string(),
            Type::Symbol(s) => s.to_string(),
        }
    }

    pub fn get_type_name(&self) -> &'static str {
        match self {
            Type::Number(_) => "Number",
            Type::Boolean(_) => "Boolean",
            Type::String(_) => "String",
            Type::Symbol(_) => "Symbol",
        }
    }

    pub fn is_number(s: &str) -> bool {
        s.parse::<i32>().is_ok()
    }

    pub fn is_boolean(s: &str) -> bool {
            s == BOOLEAN_TRUE_STR || s == BOOLEAN_FALSE_STR
    }

    pub fn is_string(s: &str) -> bool {
        s.starts_with("\"") && s.ends_with("\"")
    }

    pub fn is_symbol(s: &str) -> bool {
        !Type::is_number(s) && !Type::is_string(s)
    }
}


