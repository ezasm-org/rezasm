use std::num::ParseIntError;
use regex::Regex;

use crate::simulator::registry;

pub enum EZNumber {
    Decimal(String),
    Hexadecimal(String),
    Binary(String),
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

pub fn get_number_type(text: String) -> EZNumber {
    match text {
        s if s.starts_with("0x") || s.starts_with("-0x") => EZNumber::Hexadecimal(s),
        s if s.starts_with("0b") || s.starts_with("-0b") => EZNumber::Binary(s),
        s => EZNumber::Decimal(s),
    }
}

pub fn text_to_integer(num: EZNumber) -> Result<i64, ParseIntError> {
    match num {
        EZNumber::Hexadecimal(s) => i64::from_str_radix(s.replace("0x", "").as_str(), 16),
        EZNumber::Binary(s) => i64::from_str_radix(s.replace("0b", "").as_str(), 2),
        EZNumber::Decimal(s) => i64::from_str_radix(s.as_str(), 10),
    }
}

pub fn text_to_float(num: EZNumber) -> f64 {
    match num {
        EZNumber::Hexadecimal(s) => parse_float_string(&s.replace("0x", ""), 16u8),
        EZNumber::Binary(s) => parse_float_string(&s.replace("0b", ""), 2u8),
        EZNumber::Decimal(s) => Some(s.parse::<f64>().unwrap()),
    }
    .unwrap()
}

pub fn parse_float_string(string: &String, base: u8) -> Option<f64> {
    if string.find(".") != string.rfind(".") {
        return None;
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
            Err(_) => return None,
        },
    };
    let mut tail = String::new();
    let mantissa: i64 = match number.split(".").min() {
        None => 0,
        Some(last) => {
            tail = last.parse().unwrap();
            match i64::from_str_radix(last, base as u32) {
                Ok(i) => i,
                Err(_) => return None,
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

    Some(result)
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
