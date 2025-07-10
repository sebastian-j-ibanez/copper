// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

use crate::eval::Symbol;

/// Parse a line into a vector of symbols.
pub fn parse_line(line: &str) -> Option<Vec<Symbol>> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut split_line = line.split_whitespace();
    while let Some(symbol_str) = split_line.next() {
        match Symbol::from_str(symbol_str) {
            Some(symbol) => {
                symbols.push(symbol);
            },
            None => {
                eprintln!("unable to parse line: symbol is not valid");
                return None
            }
        }
    }

    Some(symbols)
}
