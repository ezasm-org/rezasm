pub mod arithmetic_instructions;
pub mod branch_instructions;

pub fn register_instructions() {
    arithmetic_instructions::register_instructions();
    branch_instructions::register_instructions();
}
