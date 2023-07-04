use std::num::ParseIntError;
use regex::Regex;

use crate::{simulator::registry, error::EzasmError};

pub enum EZNumberFormat {
    Decimal(String),
    Hexadecimal(String),
    Binary(String),
    DecimalFloat(String),
    HexadecimalFloat(String),
    BinaryFloat(String),
}

pub enum EZNumber {
    Integer(i64),
    Float(f64),
}

impl From<i64> for EZNumber {
    fn from(num: i64) -> Self {
        EZNumber::Integer(num)
    }
}

impl From<f64> for EZNumber {
    fn from(num: f64) -> Self {
        EZNumber::Float(num)
    }
}

pub enum Token {
    Instruction(String),
    NumericalImmediate(EZNumber),
    CharacterImmediate(String),
    StringImmediate(String),
    Register(String),
    Dereference(String),
    LabelReference(String),
    Label(String),
}

pub fn is_alphanumeric_underscore(c: &char) -> bool {
    c.is_alphanumeric() || c == &'_'
}

pub fn all_alphanumeric_underscore(text: &str) -> bool {
    for c in text.chars() {
        if !is_alphanumeric_underscore(&c) {
            return false;
        }
    }
    true
}

pub fn get_number_type(text: String) -> EZNumberFormat {
    if text.contains('.'){
        match text {
            s if s.starts_with("0x") || s.starts_with("-0x") => EZNumberFormat::Hexadecimal(s),
            s if s.starts_with("0b") || s.starts_with("-0b") => EZNumberFormat::Binary(s),
            s => EZNumberFormat::Decimal(s),
        }
    }else{
        match text {
            s if s.starts_with("0x") || s.starts_with("-0x") => EZNumberFormat::Hexadecimal(s),
            s if s.starts_with("0b") || s.starts_with("-0b") => EZNumberFormat::Binary(s),
            s => EZNumberFormat::Decimal(s),
        }
    }
}

pub fn text_to_number(num: EZNumberFormat) -> Result<EZNumber, EzasmError> {
    match num {
        EZNumberFormat::Hexadecimal(s) => i64::from_str_radix(s.replace("0x", "").as_str(), 16).map_err(EzasmError::from).map(EZNumber::from),
        EZNumberFormat::Binary(s) => i64::from_str_radix(s.replace("0b", "").as_str(), 2).map_err(EzasmError::from).map(EZNumber::from),
        EZNumberFormat::Decimal(s) => i64::from_str_radix(s.as_str(), 10).map_err(EzasmError::from).map(EZNumber::from),
        EZNumberFormat::HexadecimalFloat(s) => parse_float_string(&s.replace("0x", ""), 16u8).map(EZNumber::from),
        EZNumberFormat::BinaryFloat(s) => parse_float_string(&s.replace("0b", ""), 2u8).map(EZNumber::from),
        EZNumberFormat::DecimalFloat(s) => {
            let k = s.parse::<f64>();
            match k {
                Ok(x) => Ok(EZNumber::Float(x)),
                Err(e) => Err(EzasmError::from(e)),
            }
        }
    }
}

pub fn parse_float_string(string: &String, base: u8) -> Result<f64, EzasmError> {
    if string.find(".") != string.rfind(".") {
        return Err(EzasmError::ParserError);
    }
    let mut number = string.as_str();

    let mut negative = false;
    if string.starts_with("-") {
        number = &number[1..];
        negative = true;
    }

    let whole: i64 = match number.split(".").max() {
        None => 0,
        Some(first) => match i64::from_str_radix(first, base as u32) {
            Ok(i) => i,
            Err(e) => return Err(EzasmError::from(e)),
        },
    };
    let mut tail = String::new();
    let mantissa: i64 = match number.split(".").min() {
        None => 0,
        Some(last) => {
            tail = last.parse().unwrap();
            match i64::from_str_radix(last, base as u32) {
                Ok(i) => i,
                Err(e) => return Err(EzasmError::from(e)),
            }
        }
    };

    let mut result = if tail.len() == 0 {
        whole as f64 + (mantissa as f64)
    } else {
        whole as f64 + (mantissa as f64) / f64::powi(base as f64, tail.len() as i32)
    };

    if negative {
        result *= -1f64;
    };

    Ok(result)
}

pub fn is_label(token: &String) -> bool {
    match token.find(":") {
        None => false,
        Some(i) => {
            token.len() > 1
                && i == token.len() - 1
                && all_alphanumeric_underscore(&token.as_str()[..i])
        }
    }
}

pub fn looks_like_label_reference(token: &String) -> bool {
    all_alphanumeric_underscore(token)
}

pub fn is_register(token: &String) -> bool {
    token.starts_with("$") && token.len() > 1 && registry::is_valid_register(token)
}

pub fn looks_like_dereference(token: &String) -> bool {
    //unwrap below should never panic because the pattern is hardcoded
    let pattern = Regex::new("^(-?\\d+)?\\(\\$.+\\)$").unwrap();
    pattern.is_match(token)
}

pub fn looks_like_numerical_immediate(token: &String) -> bool {
    !token.is_empty() && is_numeric(token)
}

pub fn looks_like_character_immediate(token: &String) -> bool {
    token.len() > 1 && token.starts_with('\'') && token.ends_with('\'')
}

pub fn looks_like_string_immediate(token: &String) -> bool {
    token.len() > 1 && token.starts_with('"') && token.ends_with('"')
}

pub fn get_character_immediate(token: &String) -> Result<char, EzasmError> {
    if token.len() == 3 {
        //unwrap should never fail do to length check
        return Ok(token.chars().nth(1).unwrap());
    }else if token.len() == 4{
        //TODO consider reworking this part of the program as it is not at feature parity with main
        let mut temp = token.clone();
        temp.pop();
        if token.chars().nth(1).unwrap() != '\\'{
            return Err(EzasmError::ParserError);
        }
        match temp.pop() {
            None => Err(EzasmError::ParserError),
            Some(c) => {
                match c {
                    't' => Ok('\t'),
                    'n' => Ok('\n'),
                    'r' => Ok('\r'),
                    '\'' => Ok('\''),
                    '"' => Ok('\"'),
                    '\\' => Ok('\\'),
                    _ => Err(EzasmError::ParserError)
                }
            }
        }
    }else if token.len() < 3 {
        return Err(EzasmError::ParserError)
    }else{
        Err(EzasmError::ParserError)
    }
}

// Regex matching sucks, the way it was done in the original sucks way more though 
pub fn is_numeric(token: &String) -> bool{
    let binary_pattern = Regex::new("0b[10]+\\.?[01]*").unwrap();
    let hex_pattern = Regex::new("0x[\\d|a-f]+\\.?[\\d|a-f]*").unwrap();
    let decimal_pattern = Regex::new("[\\d]+\\.?[\\d]*").unwrap();
    let lower = token.to_lowercase();
    binary_pattern.is_match(lower.as_str()) || hex_pattern.is_match(lower.as_str()) || decimal_pattern.is_match(lower.as_str())
}



pub fn tokenize_line(text: &String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let trimmed = match text.split("#").max() {
        None => text,
        Some(first) => first,
    }
    .trim();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false;
    let mut current: String = String::new();

    for c in trimmed.chars() {
        if escape_next {
            escape_next = false;
            current.push(c);
        } else if c == '\\' {
            escape_next = true;
            current.push(c);
        } else if c == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;
            current.push(c);
        } else if c == '\"' && !in_single_quotes {
            in_double_quotes = !in_double_quotes;
            current.push(c);
        } else if in_single_quotes || in_double_quotes || !(char::is_whitespace(c) || c == ',') {
            current.push(c);
        } else if current.len() > 0 {
            tokens.push(current);
            current = String::new();
        }
    }

    if current.len() > 0 {
        tokens.push(current);
    }

    tokens
}

