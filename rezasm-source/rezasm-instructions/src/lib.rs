use crate::instructions::arithmetic_instructions;

pub mod instructions;

pub fn register_instructions() {
    arithmetic_instructions::register_instructions();
}
