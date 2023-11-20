use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;

use crate::parser::line::*;
use crate::simulation::registry;
use crate::util::error::ParserError;
use crate::util::word_size::WordSize;

pub enum EZNumberFormat {
    Decimal(String),
    Hexadecimal(String),
    Binary(String),
    DecimalFloat(String),
    HexadecimalFloat(String),
    BinaryFloat(String),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Token {
    NumericalImmediate(EZNumber),
    CharacterImmediate(char),
    StringImmediate(String),
    Register(usize),
    Dereference(i64, usize),
    LabelReference(String),
}

pub fn is_alphanumeric_underscore(c: &char) -> bool {
    c.is_alphanumeric() || c == &'_'
}

pub fn all_alphanumeric_underscore(text: &str) -> bool {
    if text.len() > 0 && is_numeric(&format!("{}", text.chars().nth(0).unwrap())) {
        return false;
    }

    for c in text.chars() {
        if !is_alphanumeric_underscore(&c) {
            return false;
        }
    }
    true
}

pub fn get_number_type(text: String) -> EZNumberFormat {
    if text.contains('.') {
        match text {
            s if s.starts_with("0x") || s.starts_with("-0x") => EZNumberFormat::HexadecimalFloat(s),
            s if s.starts_with("0b") || s.starts_with("-0b") => EZNumberFormat::BinaryFloat(s),
            s => EZNumberFormat::DecimalFloat(s),
        }
    } else {
        match text {
            s if s.starts_with("0x") || s.starts_with("-0x") => EZNumberFormat::Hexadecimal(s),
            s if s.starts_with("0b") || s.starts_with("-0b") => EZNumberFormat::Binary(s),
            s => EZNumberFormat::Decimal(s),
        }
    }
}

pub fn text_to_number(token: String) -> Result<EZNumber, ParserError> {
    match get_number_type(token) {
        EZNumberFormat::Hexadecimal(s) => i64::from_str_radix(s.replace("0x", "").as_str(), 16)
            .map_err(ParserError::from)
            .map(EZNumber::from),
        EZNumberFormat::Binary(s) => i64::from_str_radix(s.replace("0b", "").as_str(), 2)
            .map_err(ParserError::from)
            .map(EZNumber::from),
        EZNumberFormat::Decimal(s) => i64::from_str_radix(s.as_str(), 10)
            .map_err(ParserError::from)
            .map(EZNumber::from),
        EZNumberFormat::HexadecimalFloat(s) => {
            parse_float_string(&s.replace("0x", ""), 16u8).map(EZNumber::from)
        }
        EZNumberFormat::BinaryFloat(s) => {
            parse_float_string(&s.replace("0b", ""), 2u8).map(EZNumber::from)
        }
        EZNumberFormat::DecimalFloat(s) => {
            let k = s.parse::<f64>();
            match k {
                Ok(x) => Ok(EZNumber::Float(x)),
                Err(e) => Err(ParserError::from(e)),
            }
        }
    }
}

pub fn parse_float_string(string: &String, base: u8) -> Result<f64, ParserError> {
    if string.find(".") != string.rfind(".") {
        return Err(ParserError::NumericalImmediateError(string.to_string()).into());
    }
    let mut number = string.as_str();

    let mut negative = false;
    if string.starts_with("-") {
        number = &number[1..];
        negative = true;
    }

    let whole: i64 = match number.split(".").nth(0) {
        None => 0,
        Some(first) => match i64::from_str_radix(first, base as u32) {
            Ok(i) => i,
            Err(e) => return Err(ParserError::from(e)),
        },
    };

    let mut tail = String::new();
    let mantissa: i64 = match number.split(".").nth(1) {
        None => 0,
        Some(last) => {
            tail = String::from(last.trim_end_matches('0'));
            if tail.len() == 0 {
                0
            } else {
                match i64::from_str_radix(&tail, base as u32) {
                    Ok(i) => i,
                    Err(e) => return Err(ParserError::from(e)),
                }
            }
        }
    };

    let mut result = (whole as f64) + (mantissa as f64) / f64::powi(base as f64, tail.len() as i32);

    if negative {
        result *= -1f64;
    };

    Ok(result)
}

pub fn looks_like_label(token: &String) -> bool {
    match token.rfind(':') {
        None => false,
        Some(index) => index == token.len() - 1,
    }
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

pub fn get_register(token: &String) -> Result<Token, ParserError> {
    match registry::get_register_number(token) {
        Ok(n) => Ok(Token::Register(n)),
        Err(_) => Err(ParserError::UnknownRegisterError(token.to_string())),
    }
}

pub fn looks_like_dereference(token: &String) -> bool {
    // OnceLock is used to only parse regex pattern once
    // performance can still be gained by using something other than regex
    static PATTERN_CELL: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN_CELL.get_or_init(|| Regex::new("^(-?\\d+)?\\(\\$.+\\)$").unwrap());
    pattern.is_match(token)
}

pub fn get_dereference(token: &String) -> Result<Token, ParserError> {
    let lparen = match token.find('(') {
        None => return Err(ParserError::DereferenceError(token.to_string())),
        Some(x) => x,
    };
    let rparen = match token.rfind(')') {
        None => return Err(ParserError::DereferenceError(token.to_string())),
        Some(x) => x,
    };

    let register_string: String = token
        .chars()
        .skip(lparen + 1)
        .take(rparen - lparen - 1)
        .collect();

    let register = match registry::get_register_number(&register_string) {
        Ok(n) => n,
        Err(_) => return Err(ParserError::UnknownRegisterError(register_string)),
    };

    let offset_string: String = if lparen > 0 {
        token.chars().take(lparen).collect()
    } else {
        "".to_string()
    };

    let offset: i64 = if offset_string.is_empty() {
        0
    } else {
        match i64::from_str(&offset_string) {
            Ok(x) => x,
            Err(_) => return Err(ParserError::DereferenceError(token.to_string())),
        }
    };

    Ok(Token::Dereference(offset, register))
}

pub fn looks_like_numerical_immediate(token: &String) -> bool {
    !token.is_empty() && is_numeric(token)
}

pub fn get_numerical_immediate(token: &String) -> Result<Token, ParserError> {
    Ok(Token::NumericalImmediate(text_to_number(
        token.to_string(),
    )?))
}

pub fn looks_like_string_immediate(token: &String) -> bool {
    token.len() > 1 && token.starts_with('"') && token.ends_with('"')
}

pub fn looks_like_character_immediate(token: &String) -> bool {
    token.len() > 1 && token.starts_with('\'') && token.ends_with('\'')
}

pub fn get_character_immediate(token: &String) -> Result<Token, ParserError> {
    if token.len() == 3 {
        return Ok(Token::CharacterImmediate(token.chars().nth(1).unwrap()));
    } else if token.len() == 4 {
        //TODO consider reworking this part of the program as it is not at feature parity with main
        let mut temp = token.clone();
        temp.pop();
        if token.chars().nth(1).unwrap() != '\\' {
            return Err(ParserError::CharacterImmediateError(token.to_string()).into());
        }
        match temp.pop() {
            None => Err(ParserError::CharacterImmediateError(token.to_string()).into()),
            Some(c) => match c {
                't' => Ok(Token::CharacterImmediate('\t')),
                'n' => Ok(Token::CharacterImmediate('\n')),
                'r' => Ok(Token::CharacterImmediate('\r')),
                '\'' => Ok(Token::CharacterImmediate('\'')),
                '"' => Ok(Token::CharacterImmediate('\"')),
                '\\' => Ok(Token::CharacterImmediate('\\')),
                _ => Err(ParserError::CharacterImmediateError(token.to_string()).into()),
            },
        }
    } else {
        Err(ParserError::CharacterImmediateError(token.to_string()).into())
    }
}

pub fn parse_line(line: &String, word_size: &WordSize) -> Option<Result<Line, ParserError>> {
    let tokens = tokenize_line(line);

    if tokens.len() == 0 {
        None
    } else {
        Some(Line::new(&tokens[0], (&tokens[1..]).to_vec(), word_size))
    }
}

// TODO ake the simulator use this
pub fn parse_lines(lines: &String, word_size: &WordSize) -> Result<Vec<Line>, ParserError> {
    let mut output: Vec<Line> = Vec::new();
    for s in lines.lines() {
        output.push(match parse_line(&s.into(), word_size) {
            Some(v) => v,
            None => continue,
        }?);
    }
    Ok(output)
}

pub fn get_string_immediate(token: &String) -> Result<String, ParserError> {
    if token.len() < 2 {
        return Err(ParserError::StringImmediateError(token.to_string()).into());
    }

    let chars_full = token.chars();
    let mut tmp = token.clone();
    tmp.remove(0);
    tmp.push('\0');
    let chars_zip = tmp.chars();

    let mut result = "".to_string();
    let mut last_character: Option<char> = None;
    for (current, next) in chars_full.zip(chars_zip) {
        match last_character {
            Some('\\') => {
                last_character = Some(current);
                continue;
            }
            _ => {}
        }
        if current == '\\' {
            match next {
                't' => result.push('\t'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                '\'' => result.push('\''),
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                _ => return Err(ParserError::StringImmediateError(token.to_string()).into()),
            }
            last_character = Some('\\');
        } else {
            result.push(current);
            last_character = Some(current);
        }
    }
    Ok(result.to_string())
}

pub fn is_numeric(token: &String) -> bool {
    // OnceLock is used to only parse regex pattern once
    // performance can still be gained by using something other than regex
    static BINARY_PATTERN_CELL: OnceLock<Regex> = OnceLock::new();
    static HEX_PATTERN_CELL: OnceLock<Regex> = OnceLock::new();
    static DECIMAL_PATTERN_CELL: OnceLock<Regex> = OnceLock::new();
    let binary_pattern =
        BINARY_PATTERN_CELL.get_or_init(|| Regex::new("^-?0b[10]+\\.?[01]*$").unwrap());
    let hex_pattern =
        HEX_PATTERN_CELL.get_or_init(|| Regex::new("^-?0x[\\d|a-f]+\\.?[\\d|a-f]*$").unwrap());
    let decimal_pattern =
        DECIMAL_PATTERN_CELL.get_or_init(|| Regex::new("^-?[\\d]+\\.?[\\d]*$").unwrap());
    let lower = token.to_lowercase();
    binary_pattern.is_match(lower.as_str())
        || hex_pattern.is_match(lower.as_str())
        || decimal_pattern.is_match(lower.as_str())
}

pub fn tokenize_line(text: &String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let trimmed = match text.split("#").nth(0) {
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
