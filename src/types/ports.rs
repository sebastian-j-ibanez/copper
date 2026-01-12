// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-11-30

//! I/O port types.

use crate::error::Error;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::rc::Rc;

type RcRef<T> = Rc<RefCell<T>>;
pub type FileInputBuf = BufReader<File>;
pub type FileOutputBuf = BufWriter<File>;

#[derive(Debug, Clone)]
pub enum Port {
    TextInput(RcRef<TextInputPort>),
    TextOutput(RcRef<TextOutputPort>),
    BinaryInput(RcRef<BinaryInputPort>),
    BinaryOutput(RcRef<BinaryOutputPort>),
}

impl Port {
    pub fn text_input_file(path: &str) -> Result<Self, Error> {
        Ok(Port::TextInput(Rc::new(RefCell::new(
            TextInputPort::from_file(path)?,
        ))))
    }

    pub fn text_input_string(s: String) -> Self {
        Port::TextInput(Rc::new(RefCell::new(TextInputPort::from_string(s))))
    }

    pub fn text_input_stdin() -> Self {
        Port::TextInput(Rc::new(RefCell::new(TextInputPort::Stdin)))
    }

    pub fn text_output_file(path: &str) -> Result<Self, Error> {
        Ok(Port::TextOutput(Rc::new(RefCell::new(
            TextOutputPort::from_file(path)?,
        ))))
    }

    pub fn text_output_string() -> Self {
        Port::TextOutput(Rc::new(RefCell::new(TextOutputPort::new_string())))
    }

    pub fn text_output_stdout() -> Self {
        Port::TextOutput(Rc::new(RefCell::new(TextOutputPort::Stdout)))
    }

    pub fn binary_input_file(path: &str) -> Result<Self, Error> {
        Ok(Port::BinaryInput(Rc::new(RefCell::new(
            BinaryInputPort::from_file(path)?,
        ))))
    }

    pub fn binary_output_file(path: &str) -> Result<Self, Error> {
        Ok(Port::BinaryOutput(Rc::new(RefCell::new(
            BinaryOutputPort::from_file(path)?,
        ))))
    }

    pub fn close(&self) {
        match self {
            Self::TextInput(p) => p.borrow_mut().close(),
            Self::TextOutput(p) => p.borrow_mut().close(),
            Self::BinaryInput(p) => p.borrow_mut().close(),
            Self::BinaryOutput(p) => p.borrow_mut().close(),
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::TextInput(p) => p.borrow().is_open(),
            Self::TextOutput(p) => p.borrow().is_open(),
            Self::BinaryInput(p) => p.borrow().is_open(),
            Self::BinaryOutput(p) => p.borrow().is_open(),
        }
    }

    pub fn is_input(&self) -> bool {
        matches!(self, Self::TextInput(_) | Self::BinaryInput(_))
    }

    pub fn is_output(&self) -> bool {
        matches!(self, Self::TextOutput(_) | Self::BinaryOutput(_))
    }

    pub fn is_textual(&self) -> bool {
        matches!(self, Self::TextInput(_) | Self::TextOutput(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::BinaryInput(_) | Self::BinaryOutput(_))
    }
}

#[derive(Debug)]
pub enum TextInputPort {
    File(Option<FileInputBuf>),
    String(Option<VecDeque<char>>),
    Stdin,
}

impl TextInputPort {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let file =
            File::open(path).map_err(|e| Error::Message(format!("unable to open file: {}", e)))?;
        Ok(Self::File(Some(BufReader::new(file))))
    }

    pub fn from_string(s: String) -> Self {
        Self::String(Some(s.chars().collect()))
    }

    pub fn close(&mut self) {
        match self {
            Self::File(stream) => {
                stream.take();
            }
            Self::String(stream) => {
                stream.take();
            }
            Self::Stdin => {}
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::File(stream) => stream.is_some(),
            Self::String(stream) => stream.is_some(),
            Self::Stdin => true,
        }
    }

    pub fn read_char(&mut self) -> Result<char, Error> {
        match self {
            Self::File(Some(reader)) => {
                let mut buf = [0u8; 1];
                match reader.read(&mut buf) {
                    Ok(1) => Ok(buf[0] as char),
                    Ok(0) => Err(Error::new("end of file")),
                    _ => Err(Error::new("unable to read from file")),
                }
            }
            Self::File(None) => Err(Error::new("port is closed")),

            Self::String(Some(stream)) => stream
                .pop_front()
                .ok_or_else(|| Error::new("port is empty")),
            Self::String(None) => Err(Error::new("port is closed")),

            Self::Stdin => {
                let mut buf = [0u8; 1];
                io::stdin()
                    .read_exact(&mut buf)
                    .map_err(|_| Error::new("unable to read from stdin"))?;
                Ok(buf[0] as char)
            }
        }
    }

    pub fn peek_char(&mut self) -> Result<Option<char>, Error> {
        match self {
            Self::File(Some(reader)) => match reader.fill_buf() {
                Ok(bytes) if bytes.is_empty() => Ok(None),
                Ok(bytes) => Ok(Some(bytes[0] as char)),
                Err(e) => Err(Error::Message(format!("unable to peek: {}", e))),
            },
            Self::File(None) => Err(Error::new("port is closed")),

            Self::String(Some(stream)) => Ok(stream.front().copied()),
            Self::String(None) => Err(Error::new("port is closed")),

            Self::Stdin => Ok(None), // stdin doesn't support peek
        }
    }

    pub fn read_line(&mut self) -> Result<String, Error> {
        match self {
            Self::File(Some(reader)) => {
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .map_err(|e| Error::Message(format!("unable to read line: {}", e)))?;
                Ok(line)
            }
            Self::File(None) => Err(Error::new("port is closed")),

            Self::String(Some(stream)) => {
                let mut line = String::new();
                while let Some(c) = stream.pop_front() {
                    if c == '\n' {
                        break;
                    }
                    line.push(c);
                }
                Ok(line)
            }
            Self::String(None) => Err(Error::new("port is closed")),

            Self::Stdin => {
                let mut line = String::new();
                io::stdin()
                    .read_line(&mut line)
                    .map_err(|_| Error::new("unable to read from stdin"))?;
                Ok(line)
            }
        }
    }

    pub fn read_lines(&mut self) -> Result<Vec<String>, Error> {
        match self {
            Self::File(Some(reader)) => {
                let mut lines = Vec::new();
                for line in reader.lines() {
                    lines.push(line.map_err(|e| Error::Message(format!("read error: {}", e)))?);
                }
                Ok(lines)
            }
            Self::File(None) => Err(Error::new("port is closed")),

            Self::String(Some(stream)) => {
                let mut lines = Vec::new();
                let mut line = String::new();
                while let Some(c) = stream.pop_front() {
                    if c == '\n' {
                        lines.push(line.clone());
                        line.clear();
                    } else {
                        line.push(c);
                    }
                }
                if !line.is_empty() {
                    lines.push(line);
                }
                Ok(lines)
            }
            Self::String(None) => Err(Error::new("port is closed")),

            Self::Stdin => io::stdin()
                .lines()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Error::Message(format!("unable to read lines: {}", e))),
        }
    }
}

#[derive(Debug)]
pub enum TextOutputPort {
    File(Option<FileOutputBuf>),
    String(Option<String>),
    Stdout,
}

impl TextOutputPort {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let file = File::create(path)
            .map_err(|e| Error::Message(format!("unable to create file: {}", e)))?;
        Ok(Self::File(Some(BufWriter::new(file))))
    }

    pub fn from_string(s: String) -> Self {
        Self::String(Some(s))
    }

    pub fn new_string() -> Self {
        Self::String(Some(String::new()))
    }

    pub fn close(&mut self) {
        match self {
            Self::File(stream) => {
                stream.take();
            }
            Self::String(stream) => {
                stream.take();
            }
            Self::Stdout => {}
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::File(stream) => stream.is_some(),
            Self::String(stream) => stream.is_some(),
            Self::Stdout => true,
        }
    }

    pub fn write_char(&mut self, ch: char) -> Result<(), Error> {
        match self {
            Self::File(Some(writer)) => writer
                .write_all(&[ch as u8])
                .map_err(|e| Error::Message(format!("write failed: {}", e))),
            Self::File(None) => Err(Error::new("port is closed")),

            Self::String(Some(s)) => {
                s.push(ch);
                Ok(())
            }
            Self::String(None) => Err(Error::new("port is closed")),

            Self::Stdout => {
                print!("{}", ch);
                Ok(())
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        match self {
            Self::File(Some(writer)) => writer.flush().map_err(|_| Error::new("unable to flush")),
            Self::Stdout => io::stdout()
                .flush()
                .map_err(|_| Error::new("unable to flush")),
            _ => Ok(()),
        }
    }

    pub fn get_output_string(&self) -> Option<&str> {
        match self {
            Self::String(Some(s)) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum BinaryInputPort {
    File(Option<FileInputBuf>),
}

impl BinaryInputPort {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let file =
            File::open(path).map_err(|e| Error::Message(format!("unable to open file: {}", e)))?;
        Ok(Self::File(Some(BufReader::new(file))))
    }

    pub fn close(&mut self) {
        match self {
            Self::File(stream) => {
                stream.take();
            }
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::File(stream) => stream.is_some(),
        }
    }

    pub fn read_byte(&mut self) -> Result<u8, Error> {
        match self {
            Self::File(Some(reader)) => {
                let mut buf = [0u8; 1];
                match reader.read(&mut buf) {
                    Ok(1) => Ok(buf[0]),
                    Ok(0) => Err(Error::new("end of file")),
                    _ => Err(Error::new("unable to read from file")),
                }
            }
            Self::File(None) => Err(Error::new("port is closed")),
        }
    }

    pub fn peek_byte(&mut self) -> Result<Option<u8>, Error> {
        match self {
            Self::File(Some(reader)) => match reader.fill_buf() {
                Ok(bytes) if bytes.is_empty() => Ok(None),
                Ok(bytes) => Ok(Some(bytes[0])),
                Err(e) => Err(Error::Message(format!("unable to peek: {}", e))),
            },
            Self::File(None) => Err(Error::new("port is closed")),
        }
    }
}

#[derive(Debug)]
pub enum BinaryOutputPort {
    File(Option<FileOutputBuf>),
}

impl BinaryOutputPort {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let file = File::create(path)
            .map_err(|e| Error::Message(format!("unable to create file: {}", e)))?;
        Ok(Self::File(Some(BufWriter::new(file))))
    }

    pub fn close(&mut self) {
        match self {
            Self::File(stream) => {
                stream.take();
            }
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::File(stream) => stream.is_some(),
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        match self {
            Self::File(Some(writer)) => writer
                .write_all(&[byte])
                .map_err(|e| Error::Message(format!("write failed: {}", e))),
            Self::File(None) => Err(Error::new("port is closed")),
        }
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        match self {
            Self::File(Some(writer)) => writer.flush().map_err(|_| Error::new("unable to flush")),
            _ => Ok(()),
        }
    }
}
