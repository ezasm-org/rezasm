pub mod arithmetic_instructions;
pub mod branch_instructions;
pub mod comparison_instructions;
pub mod float_arithmetic_instructions;
pub mod function_instructions;
pub mod memory_instructions;
pub mod terminal_output_instructions;

pub fn register_instructions() {
    arithmetic_instructions::register_instructions();
    branch_instructions::register_instructions();
    comparison_instructions::register_instructions();
    float_arithmetic_instructions::register_instructions();
    terminal_output_instructions::register_instructions();
    function_instructions::register_instructions();
    memory_instructions::register_instructions();
}
