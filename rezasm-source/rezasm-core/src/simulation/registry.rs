use std::str::FromStr;

use bimap::BiMap;
use lazy_static::lazy_static;

use crate::simulation::register::Register;
use crate::util::error::ParserError;
use crate::util::raw_data::RawData;
use crate::util::word_size::WordSize;

const REGISTERS_COUNT: usize = 54;

pub const ZERO_NUMBER: usize = 0;
pub const FID_NUMBER: usize = 2;
pub const PC_NUMBER: usize = 3;
pub const SP_NUMBER: usize = 4;
pub const RA_NUMBER: usize = 5;
pub const R0_NUMBER: usize = 9;

lazy_static! {
    pub static ref REGISTERS_MAP: BiMap<String, usize> = {
        let mut temp_map: BiMap<String, usize> = BiMap::new();
        for (i, reg) in ALL_REGISTERS.iter().enumerate() {
            temp_map.insert(reg.to_string(), i);
        }
        temp_map
    };
}

// Base registers
pub const ZERO: &str = "ZERO"; // The number zero
pub const PID: &str = "PID"; // Program identifier
pub const FID: &str = "FID"; // File Identifier
pub const PC: &str = "PC"; // Program counter
pub const SP: &str = "SP"; // Stack pointer
pub const RA: &str = "RA"; // Return address
pub const A0: &str = "A0"; // Argument 0
pub const A1: &str = "A1"; // Argument 1
pub const A2: &str = "A2"; // Argument 2
pub const R0: &str = "R0"; // Return 0
pub const R1: &str = "R1"; // Return 1
pub const R2: &str = "R2"; // Return 2
pub const BASE_REGISTERS: [&str; 12] = [ZERO, PID, FID, PC, SP, RA, A0, A1, A2, R0, R1, R2];

// Saved registers
pub const S0: &str = "S0";
pub const S1: &str = "S1";
pub const S2: &str = "S2";
pub const S3: &str = "S3";
pub const S4: &str = "S4";
pub const S5: &str = "S5";
pub const S6: &str = "S6";
pub const S7: &str = "S7";
pub const S8: &str = "S8";
pub const S9: &str = "S9";
pub const SAVED_REGISTERS: [&str; 10] = [S0, S1, S2, S3, S4, S5, S6, S7, S8, S9];

// Temporary registers
pub const T0: &str = "T0";
pub const T1: &str = "T1";
pub const T2: &str = "T2";
pub const T3: &str = "T3";
pub const T4: &str = "T4";
pub const T5: &str = "T5";
pub const T6: &str = "T6";
pub const T7: &str = "T7";
pub const T8: &str = "T8";
pub const T9: &str = "T9";
pub const TEMPORARY_REGISTERS: [&str; 10] = [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9];

// Saved float registers
pub const FS0: &str = "FS0";
pub const FS1: &str = "FS1";
pub const FS2: &str = "FS2";
pub const FS3: &str = "FS3";
pub const FS4: &str = "FS4";
pub const FS5: &str = "FS5";
pub const FS6: &str = "FS6";
pub const FS7: &str = "FS7";
pub const FS8: &str = "FS8";
pub const FS9: &str = "FS9";
pub const FLOAT_SAVED_REGISTERS: [&str; 10] = [FS0, FS1, FS2, FS3, FS4, FS5, FS6, FS7, FS8, FS9];

// Temporary float registers
pub const FT0: &str = "FT0";
pub const FT1: &str = "FT1";
pub const FT2: &str = "FT2";
pub const FT3: &str = "FT3";
pub const FT4: &str = "FT4";
pub const FT5: &str = "FT5";
pub const FT6: &str = "FT6";
pub const FT7: &str = "FT7";
pub const FT8: &str = "FT8";
pub const FT9: &str = "FT9";
pub const FLOAT_TEMPORARY_REGISTERS: [&str; 10] =
    [FT0, FT1, FT2, FT3, FT4, FT5, FT6, FT7, FT8, FT9];

pub const LO: &str = "LO"; // Special "LOW" register to store the lower part of a multiplication
pub const HI: &str = "HI"; // Special "HIGH" register to store the higher part of a multiplication
pub const SPECIAL_REGISTERS: [&str; 2] = [LO, HI];

pub const ALL_REGISTERS: [&str; REGISTERS_COUNT] = [
    ZERO, PID, FID, PC, SP, RA, A0, A1, A2, R0, R1, R2, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, T0,
    T1, T2, T3, T4, T5, T6, T7, T8, T9, FS0, FS1, FS2, FS3, FS4, FS5, FS6, FS7, FS8, FS9, FT0, FT1,
    FT2, FT3, FT4, FT5, FT6, FT7, FT8, FT9, LO, HI,
];

fn remove_dollar(string: &String) -> String {
    if string.starts_with('$') {
        string[1..].to_string()
    } else {
        string.to_string()
    }
}

pub fn get_register_number(register: &String) -> Result<usize, ParserError> {
    match remove_dollar(register).parse::<usize>() {
        Ok(s) => return Ok(s),
        _ => (),
    }
    REGISTERS_MAP
        .get_by_left(remove_dollar(register).to_uppercase().as_str())
        .map(|r| r.clone())
        .ok_or(ParserError::InvalidRegisterNameError(register.to_string()))
}

pub fn is_valid_register(register: &String) -> bool {
    if register.len() < 1 {
        return false;
    }
    let mut temp = register.clone();
    if register.starts_with("$") {
        temp.remove(0);
    }

    temp = temp.to_uppercase();

    let number: usize = match usize::from_str(temp.as_str()) {
        Ok(x) => x,
        Err(_) => REGISTERS_COUNT + 1,
    };
    REGISTERS_MAP.contains_left(&temp) || REGISTERS_MAP.contains_right(&number)
}

pub fn is_valid_register_by_number(register: &String) -> bool {
    if register.len() < 1 {
        return false;
    }
    let mut temp = register.clone();
    if register.starts_with("$") {
        temp.remove(0);
    }
    let index = temp.parse::<usize>();

    match index {
        Ok(s) => s < REGISTERS_COUNT,
        Err(_) => false,
    }
}

pub fn is_valid_register_number(register: usize) -> bool {
    REGISTERS_MAP.contains_right(&register)
}

#[derive(Debug)]
pub struct Registry {
    word_size: WordSize,
    registers: Vec<Register>,
}

impl Registry {
    pub fn new(word_size: &WordSize) -> Registry {
        let mut registers: Vec<Register> = Vec::new();
        for _ in 0..REGISTERS_COUNT {
            registers.push(Register::new(word_size));
        }
        Registry {
            word_size: word_size.clone(),
            registers,
        }
    }

    pub fn reset(&mut self) {
        for register in self.registers.iter_mut() {
            register.set_data(RawData::empty_data(&self.word_size))
        }
    }

    pub fn get_register_by_number(&self, register: usize) -> Result<&Register, ParserError> {
        if register >= REGISTERS_COUNT {
            Err(ParserError::InvalidRegisterNumberError(register))
        } else {
            Ok(&self.registers[register])
        }
    }

    pub fn get_register_by_number_mut(
        &mut self,
        register: usize,
    ) -> Result<&mut Register, ParserError> {
        if register >= REGISTERS_COUNT {
            Err(ParserError::InvalidRegisterNumberError(register))
        } else if register == ZERO_NUMBER {
            Err(ParserError::ImmutableZeroRegisterError)
        } else {
            Ok(&mut self.registers[register])
        }
    }

    pub fn get_register(&self, register: &str) -> Result<&Register, ParserError> {
        let no_dollar = if register.starts_with('$') {
            register[1..].to_uppercase()
        } else {
            register.to_uppercase()
        };
        match REGISTERS_MAP.get_by_left(no_dollar.as_str()) {
            None => Err(ParserError::InvalidRegisterNameError(no_dollar)),
            Some(r) => self.get_register_by_number(*r),
        }
    }

    pub fn get_register_mut(&mut self, register: &String) -> Result<&mut Register, ParserError> {
        let no_dollar = if register.starts_with('$') {
            register[1..].to_uppercase()
        } else {
            register.to_uppercase()
        };
        match REGISTERS_MAP.get_by_left(no_dollar.as_str()) {
            None => Err(ParserError::InvalidRegisterNameError(no_dollar)),
            Some(r) => self.get_register_by_number_mut(*r),
        }
    }

    pub fn get_pc(&self) -> &Register {
        self.get_register_by_number(PC_NUMBER).unwrap()
    }

    pub fn get_pc_mut(&mut self) -> &mut Register {
        self.get_register_by_number_mut(PC_NUMBER).unwrap()
    }

    pub fn get_sp(&self) -> &Register {
        self.get_register_by_number(SP_NUMBER).unwrap()
    }

    pub fn get_sp_mut(&mut self) -> &mut Register {
        self.get_register_by_number_mut(SP_NUMBER).unwrap()
    }

    pub fn get_fid(&self) -> &Register {
        self.get_register_by_number(FID_NUMBER).unwrap()
    }

    pub fn get_fid_mut(&mut self) -> &mut Register {
        self.get_register_by_number_mut(FID_NUMBER).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{simulation::registry, util::word_size::DEFAULT_WORD_SIZE};

    use super::*;

    #[test]
    fn test_registry() {
        let mut registry: Registry = Registry::new(&DEFAULT_WORD_SIZE);
        registry
            .get_register_mut(&String::from(registry::T0))
            .unwrap()
            .set_data(RawData::from(255i32));
        assert_eq!(
            registry
                .get_register(&String::from(registry::T0))
                .unwrap()
                .get_data()
                .int_value(),
            255
        );

        let k =
            registry.get_register_by_number_mut(get_register_number(&"$0".to_string()).unwrap());

        assert!(
            k.is_err_and(|e| e.to_string() == ParserError::ImmutableZeroRegisterError.to_string())
        );
    }
}
