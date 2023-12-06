use crate::simulation::reader::ReaderBox;
use crate::util::error::IoError;
use std::io::Read;

pub fn skip_whitespace(reader: &mut ReaderBox) -> Result<char, IoError> {
    while let Some(byte) = reader.bytes().next() {
        match byte {
            Ok(c) => {
                if !c.is_ascii_whitespace() {
                    return Ok(c as char);
                }
            }
            Err(error) => return Err(IoError::StdIoError(error)),
        }
    }
    Err(IoError::ReadError)
}

pub fn walk_word(reader: &mut ReaderBox) -> Result<String, IoError> {
    let first = skip_whitespace(reader)?;
    let mut output: String = String::from(first);

    while let Some(byte) = reader.bytes().next() {
        match byte {
            Ok(c) => {
                if c.is_ascii_whitespace() {
                    return Ok(output);
                } else {
                    output.push(c as char)
                }
            }
            Err(error) => return Err(IoError::StdIoError(error)),
        }
    }

    Ok(output)
}

pub fn walk_line(reader: &mut ReaderBox) -> Result<String, IoError> {
    let first = skip_whitespace(reader)?;
    let mut output: String = String::from(first);

    while let Some(byte) = reader.bytes().next() {
        match byte {
            Ok(c) => {
                if c as char == '\n' {
                    return Ok(output);
                } else {
                    output.push(c as char)
                }
            }
            Err(error) => return Err(IoError::StdIoError(error)),
        }
    }

    Ok(output)
}
