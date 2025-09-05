// Copyright (c) 2025 Sebastian Ibanez
// Author: Sebastian Ibanez
// Created: 2025-07-16

//! A flexible Number type for integers, real, rational, and complex numbers.

use crate::error::Error;
use num_bigint::BigInt;
use num_complex::Complex64;
use num_integer::Integer;
use num_rational::Rational64;
use num_traits::{FromPrimitive, Num, Pow, ToPrimitive, Zero};
use std::num::ParseFloatError;
use std::ops::Rem;
use std::{
    fmt::{self},
    ops::Add,
    ops::Div,
    ops::Mul,
    ops::Sub,
};
use std::cmp::Ordering;
use crate::types::Number::{Complex, Float, Int, Rational};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(IntVariant),
    Float(f64),
    Complex(Complex64),
    Rational(Rational64),
}

impl Number {
    /// Convert a string to a number.
    pub fn from_token(s: &str) -> Result<Self, Error> {
        // Complex number
        if let Some(i_pos) = s.find('i') {
            if i_pos == s.len() - 1 {
                let without_i = &s[0..s.len() - 1];

                if without_i.is_empty() || without_i == "+" {
                    return Ok(Complex(Complex64::new(0.0, 1.0)));
                } else if without_i == "-" {
                    return Ok(Complex(Complex64::new(0.0, -1.0)));
                } else if let Ok(im_val) = without_i.parse::<f64>() {
                    return Ok(Complex(Complex64::new(0.0, im_val)));
                }

                let parts: Vec<&str> = without_i
                    .split(&['+', '-'][..])
                    .filter(|&p| !p.is_empty())
                    .collect();
                if parts.len() == 2 {
                    let re_part_str;
                    let im_part_str;
                    let mut sep_index = 0;
                    for (i, char) in without_i.chars().enumerate() {
                        if (char == '+' || char == '-') && i > 0 {
                            sep_index = i;
                            break;
                        }
                    }

                    if sep_index > 0 {
                        re_part_str = &without_i[0..sep_index];
                        im_part_str = &without_i[sep_index..];
                        let re = re_part_str.parse::<f64>().map_err(ParseFloatError::from);
                        let im = im_part_str.parse::<f64>().map_err(ParseFloatError::from);
                        return match (re, im) {
                            (Ok(re), Ok(im)) => Ok(Complex(Complex64::new(re, im))),
                            _ => Err(Error::Message("unable to parse complex number".to_string())),
                        };
                    }
                } else if parts.len() == 1
                    && (without_i.starts_with('+') || without_i.starts_with('-'))
                    && without_i.contains(|c: char| c.is_digit(10) || c == '.')
                {
                    if let Ok(im_val) = without_i.parse::<f64>() {
                        return Ok(Complex(Complex64::new(0.0, im_val)));
                    }
                }
            }
        }

        // Rational number
        if let Some(slash_pos) = s.find('/') {
            if slash_pos > 0 && slash_pos < s.len() - 1 {
                let num_parse_result = s[0..slash_pos].parse::<i64>();
                let den_parse_result = s[slash_pos + 1..].parse::<i64>();

                match (num_parse_result, den_parse_result) {
                    (Ok(num), Ok(den)) => {
                        if den == 0 {
                            return Err(Error::Message(
                                "division by zero in rational number".to_string(),
                            ));
                        }
                        return Ok(Number::from_rational(num, den));
                    }
                    (Err(e), _) => Err(Error::Message(format!(
                        "invalid rational numerator format: {}",
                        e
                    ))),
                    (_, Err(e)) => Err(Error::Message(format!(
                        "invalid rational denominator format: {}",
                        e
                    ))),
                }?;
            }
        }

        // Real number
        if s.contains('.') {
            if let Ok(f) = s.parse::<f64>() {
                return Ok(Float(f));
            }
        }

        // Fixnum
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Number::from_i64(i));
        }

        // Bignum
        if let Ok(b) = BigInt::from_str_radix(s, 10) {
            return Ok(Number::from_bigint(b));
        }

        let m = format!("unable to parse into Number: '{}'", s);
        Err(Error::Message(m))
    }

    pub fn from_i64(value: i64) -> Self {
        Int(IntVariant::Small(value))
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Int(int_var) => match int_var {
                IntVariant::Small(i) => Some(*i),
                IntVariant::Big(b) => b.to_i64(),
            },
            Float(f) => {
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
            Rational(r) => {
                if r.is_integer() {
                    r.to_i64()
                } else {
                    None
                }
            }
            Complex(_) => None,
        }
    }

    pub fn from_f64(value: f64) -> Self {
        Float(value)
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Int(int_var) => match int_var {
                IntVariant::Small(i) => Some(*i as f64),
                IntVariant::Big(b) => b.to_f64(),
            },
            Float(f) => Some(*f),
            Rational(r) => r.to_f64(),
            Complex(_) => None,
        }
    }

    pub fn from_rational(num_val: i64, den_val: i64) -> Self {
        if den_val == 0 {
            Float(f64::NAN)
        } else {
            let rational = Rational64::new(num_val, den_val);
            if rational.denom() == &1 {
                Int(IntVariant::Small(*rational.numer()))
            } else {
                Rational(rational)
            }
        }
    }

    pub fn from_bigint(value: BigInt) -> Self {
        if let Some(i64_val) = value.to_i64() {
            Int(IntVariant::Small(i64_val))
        } else {
            Int(IntVariant::Big(value))
        }
    }

    pub fn from_usize(size: usize) -> Self {
        Int(IntVariant::Small(size as i64))
    }

    /// Check if float can be simplified as an integer.
    fn rationalize_float(value: f64) -> Number {
        if value.fract() == 0.0 && value.is_finite() {
            if value >= i64::MIN as f64 && value <= i64::MAX as f64 {
                Number::from_i64(value as i64)
            } else {
                Int(IntVariant::Big(
                    BigInt::from_f64(value).unwrap_or_else(|| BigInt::from(0)),
                ))
            }
        } else {
            Float(value)
        }
    }

    /// Raise a number to the exponent of another number. Complex numbers are unsupported.
    pub fn pow(&self, exponent: &Number) -> Result<Number, Error> {
        match (self, exponent) {
            // Integer base
            (Int(base), Int(exponent)) => {
                let result = base.clone().pow(exponent.clone())?;
                Ok(Int(result))
            }
            (Int(base), Rational(exponent)) => {
                let base_float = match base {
                    IntVariant::Small(i) => *i as f64,
                    IntVariant::Big(b) => b
                        .to_f64()
                        .ok_or(Error::Message("unable to convert base to f64".to_string()))?,
                };
                let exp_float = exponent.to_f64().ok_or(Error::Message(
                    "unable to convert exponent to f64".to_string(),
                ))?;
                let result = base_float.powf(exp_float);

                Ok(Number::rationalize_float(result))
            }
            (Int(base), Float(exponent)) => {
                let base_float = match base {
                    IntVariant::Small(i) => *i as f64,
                    IntVariant::Big(b) => b
                        .to_f64()
                        .ok_or(Error::Message("unable to convert base to f64".to_string()))?,
                };
                let result = base_float.powf(*exponent);
                Ok(Number::rationalize_float(result))
            }
            // Rational base
            (Rational(base), Int(exponent)) => {
                let exp_i64 = match exponent {
                    IntVariant::Small(i) => *i,
                    IntVariant::Big(_) => {
                        return self.pow_via_float(&Int(exponent.clone()));
                    }
                };

                if exp_i64 == 0 {
                    return Ok(Number::from_i64(1));
                }

                let result = if exp_i64 < 0 {
                    let inverted = Rational64::new(*base.denom(), *base.numer());
                    inverted.pow((-exp_i64) as u32)
                } else {
                    base.pow(exp_i64 as i32)
                };

                if result.is_integer() {
                    Ok(Number::from_i64(*result.numer()))
                } else {
                    Ok(Rational(result))
                }
            }
            (Rational(base), Rational(exponent)) => {
                let base_float = base
                    .to_f64()
                    .ok_or(Error::Message("unable to convert base to f64".to_string()))?;
                let exp_float = exponent.to_f64().ok_or(Error::Message(
                    "unable to convert exponent to f64".to_string(),
                ))?;
                let result = base_float.powf(exp_float);
                Ok(Number::rationalize_float(result))
            }
            (Rational(base), Float(exponent)) => {
                let base_float = base
                    .to_f64()
                    .ok_or(Error::Message("unable to convert base to f64".to_string()))?;
                let result = base_float.powf(*exponent);
                Ok(Number::rationalize_float(result))
            }
            // Float base
            (Float(base), Int(exponent)) => {
                let exp_float = match exponent {
                    IntVariant::Small(i) => *i as f64,
                    IntVariant::Big(b) => b.to_f64().ok_or(Error::Message(
                        "unable to convert exponent to f64".to_string(),
                    ))?,
                };
                let result = base.powf(exp_float);
                Ok(Number::rationalize_float(result))
            }
            (Float(base), Rational(exponent)) => {
                let exp_float = exponent.to_f64().ok_or(Error::Message(
                    "unable to convert exponent to f64".to_string(),
                ))?;
                let result = base.powf(exp_float);
                Ok(Number::rationalize_float(result))
            }
            (Float(base), Float(exponent)) => {
                let result = base.powf(*exponent);
                Ok(Number::rationalize_float(result))
            }
            _ => Err(Error::Message(
                "pow is not implemented for this number type".to_string(),
            )),
        }
    }

    fn pow_via_float(&self, exponent: &Number) -> Result<Number, Error> {
        let base_float = self
            .to_f64()
            .ok_or(Error::Message("unable to convert base to f64".to_string()))?;
        let exp_float = exponent.to_f64().ok_or(Error::Message(
            "unable to convert exponent to f64".to_string(),
        ))?;
        let result = base_float.powf(exp_float);
        Ok(Number::rationalize_float(result))
    }
}

impl Add for Number {
    type Output = Result<Number, Error>;
    fn add(self, other: Number) -> Self::Output {
        match (self, other) {
            // Case 1: Complex + Any
            (Complex(c1), Complex(c2)) => Ok(Complex(c1 + c2)),
            (Complex(c1), Float(r2)) => {
                Ok(Complex(c1 + Complex64::new(r2, 0.0)))
            }
            (Complex(c1), Rational(r2)) => Ok(Complex(
                c1 + Complex64::new(r2.to_f64().unwrap(), 0.0),
            )),
            (Complex(c1), Int(i2)) => Ok(Complex(
                c1 + Complex64::new(i2.to_f64().unwrap(), 0.0),
            )),

            // Case 2: Real + Any (that hasn't been handled by Complex + Any)
            (Float(r1), Complex(c2)) => {
                Ok(Complex(Complex64::new(r1, 0.0) + c2))
            }
            (Float(r1), Float(r2)) => Ok(Float(r1 + r2)),
            (Float(r1), Rational(r2)) => {
                Ok(Float(r1 + r2.to_f64().unwrap()))
            }
            (Float(r1), Int(i2)) => {
                Ok(Float(r1 + i2.to_f64().unwrap()))
            }

            // Case 3: Rational + Any (that hasn't been handled by Complex/Real + Any)
            (Rational(r1), Complex(c2)) => Ok(Complex(
                Complex64::new(r1.to_f64().unwrap(), 0.0) + c2,
            )),
            (Rational(r1), Float(r2)) => {
                Ok(Float(r1.to_f64().unwrap() + r2))
            }
            (Rational(r1), Rational(r2)) => Ok(Rational(r1 + r2)),
            (Rational(r1), Int(i2)) => {
                let i2_rational = match i2 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        if let Some(f) = b.to_i64() {
                            Rational64::new(f, 1)
                        } else {
                            return Ok(Float(r1.to_f64().unwrap() + b.to_f64().unwrap()));
                        }
                    }
                };
                Ok(Rational(r1 + i2_rational))
            }

            // Case 4: Integer + Any (that hasn't been handled by higher types)
            (Int(i1), Complex(c2)) => Ok(Complex(
                Complex64::new(i1.to_f64().unwrap(), 0.0) + c2,
            )),
            (Int(i1), Float(r2)) => {
                Ok(Float(i1.to_f64().unwrap() + r2))
            }
            (Int(i1), Rational(r2)) => {
                let i1_rational = match i1 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        if let Some(f) = b.to_i64() {
                            Rational64::new(f, 1)
                        } else {
                            return Ok(Float(b.to_f64().unwrap() + r2.to_f64().unwrap()));
                        }
                    }
                };
                Ok(Rational(i1_rational + r2))
            }
            (Int(i1), Int(i2)) => match (i1, i2) {
                (IntVariant::Small(f1), IntVariant::Small(f2)) => {
                    let sum = f1.checked_add(f2);
                    match sum {
                        Some(s) => Ok(Int(IntVariant::Small(s))),
                        None => {
                            let b1 = BigInt::from(f1);
                            let b2 = BigInt::from(f2);
                            Ok(Number::from_bigint(b1 + b2))
                        }
                    }
                }
                (IntVariant::Big(b1), IntVariant::Big(b2)) => {
                    Ok(Number::from_bigint(b1 + b2))
                }
                (IntVariant::Small(f1), IntVariant::Big(b2)) => {
                    let b1 = BigInt::from(f1);
                    Ok(Number::from_bigint(b1 + b2))
                }
                (IntVariant::Big(b1), IntVariant::Small(f2)) => {
                    let b2 = BigInt::from(f2);
                    Ok(Number::from_bigint(b1 + b2))
                }
            },
        }
    }
}

impl Sub for Number {
    type Output = Result<Number, Error>;
    fn sub(self, other: Number) -> Self::Output {
        match (self, other) {
            // Complex - Any
            (Complex(c1), Complex(c2)) => Ok(Complex(c1 - c2)),
            (Complex(c1), Float(r2)) => {
                Ok(Complex(c1 - Complex64::new(r2, 0.0)))
            }
            (Complex(c1), Rational(r2)) => Ok(Complex(
                c1 - Complex64::new(r2.to_f64().unwrap(), 0.0),
            )),
            (Complex(c1), Int(i2)) => Ok(Complex(
                c1 - Complex64::new(i2.to_f64().unwrap(), 0.0),
            )),

            // Real - Any
            (Float(r1), Complex(c2)) => {
                Ok(Complex(Complex64::new(r1, 0.0) - c2))
            }
            (Float(r1), Float(r2)) => Ok(Float(r1 - r2)),
            (Float(r1), Rational(r2)) => {
                Ok(Float(r1 - r2.to_f64().unwrap()))
            }
            (Float(r1), Int(i2)) => {
                Ok(Float(r1 - i2.to_f64().unwrap()))
            }

            //Rational - Any
            (Rational(r1), Complex(c2)) => Ok(Complex(
                Complex64::new(r1.to_f64().unwrap(), 0.0) - c2,
            )),
            (Rational(r1), Float(r2)) => {
                Ok(Float(r1.to_f64().unwrap() - r2))
            }
            (Rational(r1), Rational(r2)) => Ok(Rational(r1 - r2)),
            (Rational(r1), Int(i2)) => {
                let i2_rational = match i2 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        if let Some(f) = b.to_i64() {
                            Rational64::new(f, 1)
                        } else {
                            return Ok(Float(r1.to_f64().unwrap() - b.to_f64().unwrap()));
                        }
                    }
                };
                Ok(Rational(r1 - i2_rational))
            }

            // Integer - Any
            (Int(i1), Complex(c2)) => Ok(Complex(
                Complex64::new(i1.to_f64().unwrap(), 0.0) - c2,
            )),
            (Int(i1), Float(r2)) => {
                Ok(Float(i1.to_f64().unwrap() - r2))
            }
            (Int(i1), Rational(r2)) => {
                // Promote integer to rational
                let i1_rational = match i1 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        if let Some(f) = b.to_i64() {
                            Rational64::new(f, 1)
                        } else {
                            return Ok(Float(b.to_f64().unwrap() - r2.to_f64().unwrap()));
                        }
                    }
                };
                Ok(Rational(i1_rational - r2))
            }
            (Int(i1), Int(i2)) => {
                match (i1, i2) {
                    (IntVariant::Small(f1), IntVariant::Small(f2)) => {
                        let diff = f1.checked_sub(f2);
                        match diff {
                            Some(s) => Ok(Int(IntVariant::Small(s))),
                            None => {
                                // Overflow: promote to Bignum
                                let b1 = BigInt::from(f1);
                                let b2 = BigInt::from(f2);
                                Ok(Number::from_bigint(b1 - b2))
                            }
                        }
                    }
                    (IntVariant::Big(b1), IntVariant::Big(b2)) => {
                        Ok(Number::from_bigint(b1 - b2))
                    }
                    (IntVariant::Small(f1), IntVariant::Big(b2)) => {
                        let b1 = BigInt::from(f1);
                        Ok(Number::from_bigint(b1 - b2))
                    }
                    (IntVariant::Big(b1), IntVariant::Small(f2)) => {
                        let b2 = BigInt::from(f2);
                        Ok(Number::from_bigint(b1 - b2))
                    }
                }
            }
        }
    }
}
impl Mul for Number {
    type Output = Result<Number, Error>;
    fn mul(self, other: Number) -> Self::Output {
        match (self, other) {
            // Complex * Any
            (Complex(c1), Complex(c2)) => Ok(Complex(c1 * c2)),
            (Complex(c1), Float(r2)) => {
                Ok(Complex(c1 * Complex64::new(r2, 0.0)))
            }
            (Complex(c1), Rational(r2)) => Ok(Complex(
                c1 * Complex64::new(r2.to_f64().unwrap(), 0.0),
            )),
            (Complex(c1), Int(i2)) => Ok(Complex(
                c1 * Complex64::new(i2.to_f64().unwrap(), 0.0),
            )),

            // Real * Any
            (Float(r1), Complex(c2)) => {
                Ok(Complex(Complex64::new(r1, 0.0) * c2))
            }
            (Float(r1), Float(r2)) => Ok(Float(r1 * r2)),
            (Float(r1), Rational(r2)) => {
                Ok(Float(r1 * r2.to_f64().unwrap()))
            }
            (Float(r1), Int(i2)) => {
                Ok(Float(r1 * i2.to_f64().unwrap()))
            }

            // Rational * Any
            (Rational(r1), Complex(c2)) => Ok(Complex(
                Complex64::new(r1.to_f64().unwrap(), 0.0) * c2,
            )),
            (Rational(r1), Float(r2)) => {
                Ok(Float(r1.to_f64().unwrap() * r2))
            }
            (Rational(r1), Rational(r2)) => Ok(Rational(r1 * r2)),
            (Rational(r1), Int(i2)) => {
                let i2_rational = match i2 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        b.to_i64()
                            .map(|f| Rational64::new(f, 1))
                            .ok_or(Error::Message(
                                "unable to create rational number from i64".to_string(),
                            ))?
                    }
                };
                Ok(Rational(r1 * i2_rational))
            }

            // Integer * Any
            (Int(i1), Complex(c2)) => Ok(Complex(
                Complex64::new(i1.to_f64().unwrap(), 0.0) * c2,
            )),
            (Int(i1), Float(r2)) => {
                Ok(Float(i1.to_f64().unwrap() * r2))
            }
            (Int(i1), Rational(r2)) => {
                let i1_rational = match i1 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        b.to_i64()
                            .map(|f| Rational64::new(f, 1))
                            .ok_or(Error::Message(
                                "unable to create rational number from i64".to_string(),
                            ))?
                    }
                };
                Ok(Rational(i1_rational * r2))
            }
            (Int(i1), Int(i2)) => match (i1, i2) {
                (IntVariant::Small(f1), IntVariant::Small(f2)) => {
                    let prod = f1.checked_mul(f2);
                    match prod {
                        Some(s) => Ok(Int(IntVariant::Small(s))),
                        None => Ok(Number::from_bigint(BigInt::from(f1) * BigInt::from(f2))),
                    }
                }
                (IntVariant::Big(b1), IntVariant::Big(b2)) => {
                    Ok(Number::from_bigint(b1 * b2))
                }
                (IntVariant::Small(f1), IntVariant::Big(b2)) => {
                    Ok(Number::from_bigint(BigInt::from(f1) * b2))
                }
                (IntVariant::Big(b1), IntVariant::Small(f2)) => {
                    Ok(Number::from_bigint(b1 * BigInt::from(f2)))
                }
            },
        }
    }
}

impl Div for Number {
    type Output = Result<Number, Error>;
    fn div(self, other: Number) -> Self::Output {
        // Pre-check for division by exact zero
        match &other {
            Int(IntVariant::Small(0)) => {
                return Err(Error::Message("unable to divide by 0".to_string()));
            }
            Int(IntVariant::Big(b)) if b == &BigInt::from(0) => {
                return Err(Error::Message("unable to divide by 0".to_string()));
            }
            Rational(r) if r.is_zero() => {
                return Err(Error::Message("unable to divide by 0".to_string()));
            }
            _ => {}
        }

        match (self, other) {
            // Complex / Any
            (Complex(c1), Complex(c2)) => Ok(Complex(c1 / c2)),
            (Complex(c1), Float(r2)) => {
                Ok(Complex(c1 / Complex64::new(r2, 0.0)))
            }
            (Complex(c1), Rational(r2)) => Ok(Complex(
                c1 / Complex64::new(r2.to_f64().unwrap(), 0.0),
            )),
            (Complex(c1), Int(i2)) => Ok(Complex(
                c1 / Complex64::new(i2.to_f64().unwrap(), 0.0),
            )),

            // Real / Any
            (Float(r1), Complex(c2)) => {
                Ok(Complex(Complex64::new(r1, 0.0) / c2))
            }
            (Float(r1), Float(r2)) => Ok(Float(r1 / r2)),
            (Float(r1), Rational(r2)) => {
                Ok(Float(r1 / r2.to_f64().unwrap()))
            }
            (Float(r1), Int(i2)) => {
                Ok(Float(r1 / i2.to_f64().unwrap()))
            }

            // Rational / Any
            (Rational(r1), Complex(c2)) => Ok(Complex(
                Complex64::new(r1.to_f64().unwrap(), 0.0) / c2,
            )),
            (Rational(r1), Float(r2)) => {
                Ok(Float(r1.to_f64().unwrap() / r2))
            }
            (Rational(r1), Rational(r2)) => Ok(Rational(r1 / r2)),
            (Rational(r1), Int(i2)) => {
                let i2_rational = match i2 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        b.to_i64()
                            .map(|f| Rational64::new(f, 1))
                            .ok_or(Error::Message(
                                "unable to create rational number from i64".to_string(),
                            ))?
                    }
                };
                Ok(Rational(r1 / i2_rational))
            }

            // Integer / Any
            (Int(i1), Complex(c2)) => Ok(Complex(
                Complex64::new(i1.to_f64().unwrap(), 0.0) / c2,
            )),
            (Int(i1), Float(r2)) => {
                Ok(Float(i1.to_f64().unwrap() / r2))
            }
            (Int(i1), Rational(r2)) => {
                let i1_rational = match i1 {
                    IntVariant::Small(f) => Rational64::new(f, 1),
                    IntVariant::Big(b) => {
                        b.to_i64()
                            .map(|f| Rational64::new(f, 1))
                            .ok_or(Error::Message(
                                "unable to create rational number from i64".to_string(),
                            ))?
                    }
                };
                Ok(Rational(i1_rational / r2))
            }
            (Int(i1), Int(i2)) => match (i1, i2) {
                (IntVariant::Small(f1), IntVariant::Small(f2)) => {
                    if f1 % f2 == 0 {
                        Ok(Number::from_i64(f1 / f2))
                    } else {
                        Ok(Rational(Rational64::new(f1, f2)))
                    }
                }
                (IntVariant::Big(b1), IntVariant::Big(b2)) => {
                    if b1.is_multiple_of(&b2) {
                        Ok(Number::from_bigint(b1 / b2))
                    } else {
                        let r_num = b1.to_i64().ok_or(Error::Message(
                            "number too large for rational conversion".to_string(),
                        ))?;
                        let r_den = b2.to_i64().ok_or(Error::Message(
                            "number too large for rational conversion".to_string(),
                        ))?;
                        Ok(Rational(Rational64::new(r_num, r_den)))
                    }
                }
                (IntVariant::Small(f1), IntVariant::Big(b2)) => {
                    let b1 = BigInt::from(f1);
                    if b1.is_multiple_of(&b2) {
                        Ok(Number::from_bigint(b1 / b2))
                    } else {
                        let r_num = b1.to_i64().ok_or(Error::Message(
                            "number too large for rational conversion".to_string(),
                        ))?;
                        let r_den = b2.to_i64().ok_or(Error::Message(
                            "number too large for rational conversion".to_string(),
                        ))?;
                        Ok(Rational(Rational64::new(r_num, r_den)))
                    }
                }
                (IntVariant::Big(b1), IntVariant::Small(f2)) => {
                    let b2 = BigInt::from(f2);
                    if b1.is_multiple_of(&b2) {
                        Ok(Number::from_bigint(b1 / b2))
                    } else {
                        let r_num = b1.to_i64().ok_or(Error::Message(
                            "number too large for rational conversion".to_string(),
                        ))?;
                        let r_den = b2.to_i64().ok_or(Error::Message(
                            "number too large for rational conversion".to_string(),
                        ))?;
                        Ok(Rational(Rational64::new(r_num, r_den)))
                    }
                }
            },
        }
    }
}

impl Rem for Number {
    type Output = Result<Number, Error>;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Int(i1), Int(i2)) => match (i1, i2) {
                (IntVariant::Small(i1), IntVariant::Small(i2)) => {
                    Ok(Int(IntVariant::Small(i1 % i2)))
                }
                (IntVariant::Big(i1), IntVariant::Big(i2)) => {
                    Ok(Int(IntVariant::Big(i1 % i2)))
                }
                (IntVariant::Small(i1), IntVariant::Big(i2)) => {
                    Ok(Int(IntVariant::Big(i1 % i2)))
                }
                (IntVariant::Big(i1), IntVariant::Small(i2)) => {
                    Ok(Int(IntVariant::Big(i1 % i2)))
                }
            },
            (_, _) => Err(Error::Message("expected integer".to_string())),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Int(IntVariant::Small(i)) => write!(f, "{}", i),
            Int(IntVariant::Big(b)) => write!(f, "{}", b),
            Rational(r) => write!(f, "{}", r),
            Float(r) => write!(f, "{}", r),
            Complex(c) => write!(f, "{}", c),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Int(i1), Int(i2)) => i1.partial_cmp(i2),
            (Int(i1), Float(f2)) => {
                let i1_float = i1.to_f64()?;

                if i1_float < *f2 {
                    return Some(Ordering::Less)
                } else if i1_float > *f2 {
                    return Some(Ordering::Greater)
                }

                Some(Ordering::Equal)
            }
            (Int(i1), Rational(r2)) => {
                let i1_rational = match i1 {
                    IntVariant::Small(s) => Rational64::from_i64(*s)?,
                    IntVariant::Big(h) => Rational64::from_i64(h.to_i64()?)?,
                };

                i1_rational.partial_cmp(&r2)
            }
            (Float(f1), Float(f2)) => f1.partial_cmp(f2),
            (Float(f1), Int(i2)) => {
                let i2_float = i2.to_f64()?;
                f1.partial_cmp(&i2_float)
            }
            (Float(f1), Rational(r2)) => {
                let f1_rational = Rational64::from_f64(*f1)?;
                f1_rational.partial_cmp(&r2)
            }
            (Rational(r1), Rational(r2)) => r1.partial_cmp(r2),
            (Rational(r1), Int(i2)) => {
                let i2_rational = match i2 {
                    IntVariant::Small(s) => Rational64::from_i64(*s)?,
                    IntVariant::Big(h) => Rational64::from_i64(h.to_i64()?)?,
                };

                r1.partial_cmp(&i2_rational)
            }
            (Rational(r1), Float(f2)) => {
                let f2_rational = Rational64::from_f64(*f2)?;
                r1.partial_cmp(&f2_rational)
            }
            // Complex numbers cannot be ordered, only compared for equality.
            (Int(i1), Complex(c2)) => {
                let i1_complex = match i1 {
                    IntVariant::Small(s) => Complex64::from_i64(*s),
                    IntVariant::Big(h) => Complex64::from_i64(h.to_i64()?),
                };

                if i1_complex == Some(*c2) {
                    return Some(Ordering::Equal)
                }

                None
            }
            (Float(f1), Complex(c2)) => {
                let f1_complex = Complex64::from_f64(*f1)?;

                if f1_complex == *c2 {
                    return Some(Ordering::Equal)
                }

                None
            }
            (Rational(r1), Complex(c2)) => {
                let r1_complex = Complex64::from_i64(r1.to_i64()?)?;

                if r1_complex == *c2 {
                    return Some(Ordering::Equal)
                }

                None
            }
            (Complex(c1), Complex(c2)) => {
                if c1 == c2 {
                    return Some(Ordering::Equal)
                }

                None
            }
            _ => None
        }
    }
}

/// Integer that is either fixed length or unbounded.
#[derive(Debug, Clone, PartialEq)]
pub enum IntVariant {
    Small(i64),
    Big(BigInt),
}

impl PartialOrd for IntVariant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (IntVariant::Small(i1), IntVariant::Small(i2)) => i1.partial_cmp(i2),
            (IntVariant::Big(i1), IntVariant::Big(i2)) => i1.partial_cmp(i2),
            (IntVariant::Big(i1), IntVariant::Small(i2)) => {
                let heap_i2 = &BigInt::from(*i2);
                i1.partial_cmp(heap_i2)
            },
            (IntVariant::Small(i1), IntVariant::Big(i2)) => {
                let heap_i1 = &BigInt::from(*i1);
                heap_i1.partial_cmp(i2)
            },
        }
    }
}

impl ToPrimitive for IntVariant {
    fn to_i64(&self) -> Option<i64> {
        match self {
            IntVariant::Small(f) => Some(*f),
            IntVariant::Big(b) => b.to_i64(),
        }
    }
    fn to_u64(&self) -> Option<u64> {
        match self {
            IntVariant::Small(f) => Some(*f as u64),
            IntVariant::Big(b) => b.to_u64(),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            IntVariant::Small(f) => Some(*f as f64),
            IntVariant::Big(b) => b.to_f64(),
        }
    }
}

impl Pow<IntVariant> for IntVariant {
    type Output = Result<IntVariant, Error>;
    fn pow(self, rhs: IntVariant) -> Self::Output {
        match (self, rhs) {
            (IntVariant::Small(f), IntVariant::Small(r)) => {
                let mut result = 1;
                for _ in 0..r {
                    result *= f;
                }
                Ok(IntVariant::Small(result))
            }
            (IntVariant::Big(b), IntVariant::Small(r)) => {
                let mut result = BigInt::from(1);
                for _ in 0..r {
                    result *= b.clone();
                }
                Ok(IntVariant::Big(result))
            }
            (IntVariant::Small(f), IntVariant::Big(r)) => {
                let r = r.to_u64().ok_or(Error::Message(
                    "number too large for rational conversion".to_string(),
                ))?;
                let mut result = BigInt::from(1);
                for _ in 0..r {
                    result *= BigInt::from(f);
                }
                Ok(IntVariant::Big(result))
            }
            (IntVariant::Big(b), IntVariant::Big(r)) => {
                let r = r.to_u64().ok_or(Error::Message(
                    "number too large for rational conversion".to_string(),
                ))?;
                let mut result = BigInt::from(1);
                for _ in 0..r {
                    result *= b.clone();
                }
                Ok(IntVariant::Big(result))
            }
        }
    }
}
