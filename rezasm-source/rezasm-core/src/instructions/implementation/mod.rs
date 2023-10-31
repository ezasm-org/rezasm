pub mod arithmetic_instructions;
pub mod branch_instructions;
pub mod comparison_instructions;
pub mod float_arithmetic_instructions;
pub mod terminal_instructions;

pub fn register_instructions() {
    arithmetic_instructions::register_instructions();
    branch_instructions::register_instructions();
    comparison_instructions::register_instructions();
    float_arithmetic_instructions::register_instructions();
    terminal_instructions::register_instructions();
}
