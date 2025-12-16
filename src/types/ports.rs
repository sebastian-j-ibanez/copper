// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-11-30

//! I/O port types.

use crate::error::Error;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::rc::Rc;

type RcRef<T> = Rc<RefCell<T>>;

#[derive(Debug, Clone)]
pub enum Port {
    TextInput(RcRef<dyn TextInputPort>),
    TextOutput(RcRef<dyn TextOutputPort>),
    BinaryInput(RcRef<dyn BinaryInputPort>),
    BinaryOutput(RcRef<dyn BinaryOutputPort>),
}

impl Port {
    /// Create a new `Port` from a `TextInputPort`.
    pub fn from_text_input<T: TextInputPort + 'static>(port: T) -> Port {
        Port::TextInput(Rc::new(RefCell::new(port)))
    }

    /// Create a new `Port` from a `TextOutputPort`.
    pub fn from_text_output<T: TextOutputPort + 'static>(port: T) -> Port {
        Port::TextOutput(Rc::new(RefCell::new(port)))
    }

    /// Create a new `Port` from a `BinaryInputPort`.
    pub fn from_binary_input<B: BinaryInputPort + 'static>(port: B) -> Port {
        Port::BinaryInput(Rc::new(RefCell::new(port)))
    }

    /// Create a new `Port` from a `BinaryOutputPort`.
    pub fn from_binary_output<B: BinaryOutputPort + 'static>(port: B) -> Port {
        Port::BinaryOutput(Rc::new(RefCell::new(port)))
    }

    /// Close `Port` and drop stream.
    pub fn close(&self) {
        match self {
            Self::TextInput(p) => p.borrow_mut().close(),
            Self::TextOutput(p) => p.borrow_mut().close(),
            Self::BinaryInput(p) => p.borrow_mut().close(),
            Self::BinaryOutput(p) => p.borrow_mut().close(),
        }
    }

    /// Return if `Port` is open.
    pub fn is_open(&self) -> bool {
        match self {
            Self::TextInput(p) => p.borrow_mut().is_open(),
            Self::TextOutput(p) => p.borrow_mut().is_open(),
            Self::BinaryInput(p) => p.borrow_mut().is_open(),
            Self::BinaryOutput(p) => p.borrow_mut().is_open(),
        }
    }

    /// Return if `Port` is output.
    pub fn is_output(&self) -> bool {
        match self {
            Self::TextInput(_) | Self::BinaryInput(_) => false,
            Self::TextOutput(_) | Self::BinaryOutput(_) => true,
        }
    }

    /// Return if `Port` is input.
    pub fn is_input(&self) -> bool {
        match self {
            Self::TextInput(_) | Self::BinaryInput(_) => true,
            Self::TextOutput(_) | Self::BinaryOutput(_) => false,
        }
    }

    /// Return if `Port` is textual.
    pub fn is_textual(&self) -> bool {
        match self {
            Self::TextInput(_) | Self::TextOutput(_) => true,
            Self::BinaryInput(_) | Self::BinaryOutput(_) => false,
        }
    }

    /// Return if `Port` is textual.
    pub fn is_binary(&self) -> bool {
        match self {
            Self::TextInput(_) | Self::TextOutput(_) => false,
            Self::BinaryInput(_) | Self::BinaryOutput(_) => true,
        }
    }
}

pub trait PortHandler: fmt::Debug {
    fn close(&mut self);
    fn is_open(&self) -> bool;
}

pub trait TextInputPort: PortHandler {
    /// Read `char` from `&self.reader`.
    fn read_char(&mut self) -> std::result::Result<char, Error>;

    /// Read `char` to `&self.writer` without incrementing position in `Port` buffer.
    fn peek_char(&mut self) -> std::result::Result<Option<char>, Error>;

    /// Read `char` from `&self.reader`.
    fn read_line(&mut self) -> std::result::Result<String, Error>;

    /// Read lines from `&self.reader`.
    fn read_lines(&mut self) -> std::result::Result<Vec<String>, Error>;
}

pub trait TextOutputPort: PortHandler {
    /// Write `char` to `&self.writer`.
    fn write_char(&mut self, ch: char) -> std::result::Result<(), Error>;

    /// Flush `&self.writer`. Defaults to no-op if port is not buffered.
    fn flush(&mut self) -> std::result::Result<(), Error> {
        Ok(())
    }
}

pub trait BinaryInputPort: PortHandler {
    /// Read `u8` to `&self.writer`.
    fn read_byte(&mut self) -> std::result::Result<u8, Error>;

    /// Read `u8` to `&self.writer` without incrementing position in `Port` buffer.
    fn peek_byte(&mut self) -> std::result::Result<Option<u8>, Error>;
}

pub trait BinaryOutputPort: PortHandler {
    /// Write `u8` to `&self.reader`.
    fn write_byte(&mut self, byte: u8) -> std::result::Result<(), Error>;

    /// Flush `&self.writer`. Defaults to no-op if port is not buffered.
    fn flush(&mut self) -> std::result::Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct TextFileInput {
    stream: Option<BufReader<File>>,
}

impl TextFileInput {
    /// Open a new `File` from `path`.
    pub fn open(path: &String) -> std::result::Result<TextFileInput, Error> {
        let file =
            File::open(path).map_err(|e| Error::Message(format!("unable to open file: {}", e)))?;

        let file_input = TextFileInput {
            stream: Some(BufReader::new(file)),
        };

        Ok(file_input)
    }
}

impl PortHandler for TextFileInput {
    fn close(&mut self) {
        if let Some(s) = self.stream.take() {
            drop(s)
        }
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }
}

impl TextInputPort for TextFileInput {
    /// Read a char from `&self.writer`. Return `Err` if no char was read.
    fn read_char(&mut self) -> std::result::Result<char, Error> {
        let mut buf: [u8; 1] = [0; 1];
        let reader = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::Message(format!("port is closed")))?;

        match reader.read(&mut buf) {
            Ok(1) => Ok(buf[0] as char),
            Ok(0) => Err(Error::new("read 0 characters from file")),
            _ => Err(Error::new("unable to read from file")),
        }
    }

    fn peek_char(&mut self) -> std::result::Result<Option<char>, Error> {
        let reader = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::Message(format!("port is closed")))?;

        match reader.fill_buf() {
            Ok(bytes) if bytes.is_empty() => Ok(None),
            Ok(bytes) => Ok(Some(bytes[0] as char)),
            Err(e) => Err(Error::Message(format!(
                "unable to read byte: {}",
                e.to_string()
            ))),
        }
    }

    fn read_line(&mut self) -> std::result::Result<String, Error> {
        todo!()
    }

    fn read_lines(&mut self) -> std::result::Result<Vec<String>, Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TextFileOutput {
    stream: Option<BufWriter<File>>,
}

impl TextFileOutput {
    /// Open a new `File` from `path`.
    pub fn open(path: &String) -> std::result::Result<TextFileOutput, Error> {
        let file = File::create(path)
            .map_err(|e| Error::Message(format!("unable to open file: {}", e)))?;

        let file_output = TextFileOutput {
            stream: Some(BufWriter::new(file)),
        };

        Ok(file_output)
    }
}

impl PortHandler for TextFileOutput {
    fn close(&mut self) {
        if let Some(s) = self.stream.take() {
            drop(s)
        }
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }
}

impl TextOutputPort for TextFileOutput {
    fn write_char(&mut self, ch: char) -> std::result::Result<(), Error> {
        let buffer = &[ch as u8];
        let writer = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        match writer.write(buffer) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Message(format!("write failed: {}", e.to_string()))),
        }
    }

    fn flush(&mut self) -> std::result::Result<(), Error> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        stream
            .flush()
            .map_err(|_| Error::new("unable to flush port"))
    }
}

#[derive(Debug)]
pub struct BinaryFileInput {
    stream: Option<BufReader<File>>,
}

impl BinaryFileInput {
    /// Open a new `File` from `path`.
    pub fn open(path: &String) -> std::result::Result<BinaryFileInput, Error> {
        let file =
            File::open(path).map_err(|e| Error::Message(format!("unable to open file: {}", e)))?;

        let file_output = BinaryFileInput {
            stream: Some(BufReader::new(file)),
        };

        Ok(file_output)
    }
}

impl PortHandler for BinaryFileInput {
    fn close(&mut self) {
        if let Some(s) = self.stream.take() {
            drop(s)
        }
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }
}

impl BinaryInputPort for BinaryFileInput {
    fn read_byte(&mut self) -> std::result::Result<u8, Error> {
        let mut buffer: [u8; 1] = [0; 1];
        let reader = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        match reader.read(&mut buffer) {
            Ok(_) => Ok(buffer[0]),
            Err(e) => Err(Error::Message(format!("read failed: {}", e.to_string()))),
        }
    }

    fn peek_byte(&mut self) -> std::result::Result<Option<u8>, Error> {
        let reader = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::Message(format!("port is closed")))?;

        match reader.fill_buf() {
            Ok(bytes) if bytes.is_empty() => Ok(None),
            Ok(bytes) => Ok(Some(bytes[0])),
            Err(e) => Err(Error::Message(format!(
                "unable to read byte: {}",
                e.to_string()
            ))),
        }
    }
}

#[derive(Debug)]
pub struct BinaryFileOutput {
    stream: Option<BufWriter<File>>,
}

impl BinaryFileOutput {
    /// Open a new `File` from `path`.
    pub fn open(path: &String) -> std::result::Result<BinaryFileOutput, Error> {
        let file = File::create(path)
            .map_err(|e| Error::Message(format!("unable to open file: {}", e)))?;

        let file_output = BinaryFileOutput {
            stream: Some(BufWriter::new(file)),
        };

        Ok(file_output)
    }
}

impl PortHandler for BinaryFileOutput {
    fn close(&mut self) {
        if let Some(s) = self.stream.take() {
            drop(s)
        }
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }
}

impl BinaryOutputPort for BinaryFileOutput {
    fn write_byte(&mut self, byte: u8) -> std::result::Result<(), Error> {
        let buffer = &[byte];
        let writer = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        match writer.write(buffer) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Message(format!(
                "unable to write to port: {}",
                e.to_string()
            ))),
        }
    }

    fn flush(&mut self) -> std::result::Result<(), Error> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        stream
            .flush()
            .map_err(|_| Error::new("unable to flush port"))
    }
}

#[derive(Debug)]
pub struct StringInputPort {
    stream: Option<VecDeque<char>>,
}

impl StringInputPort {
    pub fn open(s: String) -> Self {
        let mut dq = VecDeque::new();
        s.chars().for_each(|c| dq.push_back(c));
        Self { stream: Some(dq) }
    }
}

impl PortHandler for StringInputPort {
    fn close(&mut self) {
        if let Some(s) = self.stream.take() {
            drop(s)
        }
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }
}

impl TextInputPort for StringInputPort {
    fn read_char(&mut self) -> std::result::Result<char, Error> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        match stream.pop_front() {
            Some(c) => Ok(c),
            None => Err(Error::new("port is empty")),
        }
    }

    fn peek_char(&mut self) -> std::result::Result<Option<char>, Error> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        match stream.front() {
            Some(c) => Ok(Some(*c)),
            None => Err(Error::new("port is empty")),
        }
    }

    fn read_line(&mut self) -> std::result::Result<String, Error> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        let mut line = String::new();
        while let Some(c) = stream.pop_front_if(|c| *c != '\n') {
            line.push(c);
        }
        Ok(line)
    }

    fn read_lines(&mut self) -> std::result::Result<Vec<String>, Error> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?;

        let mut lines = Vec::new();
        let mut line = String::new();
        while let Some(c) = stream.pop_front() {
            line.push(c);
            if c == '\n' {
                lines.push(line.clone());
                line.clear();
            }
        }
        Ok(lines)
    }
}

#[derive(Debug)]
pub struct StringOutputPort {
    stream: Option<String>,
}

impl StringOutputPort {
    pub fn new() -> Self {
        Self {
            stream: Some(String::new()),
        }
    }

    pub fn open(s: String) -> Self {
        Self { stream: Some(s) }
    }
}

impl PortHandler for StringOutputPort {
    fn close(&mut self) {
        if let Some(s) = self.stream.take() {
            drop(s)
        }
    }

    fn is_open(&self) -> bool {
        self.stream.is_some()
    }
}

impl TextOutputPort for StringOutputPort {
    fn write_char(&mut self, ch: char) -> std::result::Result<(), Error> {
        self.stream
            .as_mut()
            .ok_or_else(|| Error::new("port is closed"))?
            .push(ch);

        Ok(())
    }
}
