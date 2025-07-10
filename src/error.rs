// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-10

pub struct Error {
    message: String,
}

impl Error {
    pub fn init(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}

