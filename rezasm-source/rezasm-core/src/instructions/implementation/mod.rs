pub mod arithmetic_instructions;
pub mod float_arithmetic_instructions;

pub fn register_instructions() {
    arithmetic_instructions::register_instructions();
    float_arithmetic_instructions::register_instructions();
}
