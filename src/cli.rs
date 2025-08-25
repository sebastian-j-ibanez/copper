// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-08-24

//! Parse and handle CLI arguments.

pub enum Flag {
    File(String),
    Help,
    Version,
}

/// Parse CLI args and return appropriate `ui::Flag`.
pub fn parse_args(args: Vec<String>) -> Option<Flag> {
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match args.as_slice() {
        ["-f", filename] | ["--file", filename] => Some(Flag::File((*filename).to_string())),
        ["-h"] | ["--help"] => Some(Flag::Help),
        ["-v"] | ["--version"] => Some(Flag::Version),
        [] => None,
        arg => {
            eprintln!("error: invalid flag '{:?}'", arg);
            std::process::exit(1);
        }
    }
}
