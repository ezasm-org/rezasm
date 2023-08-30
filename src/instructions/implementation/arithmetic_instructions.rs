use ezasm_core::instructions::instruction_field::InstructionField;
use ezasm_core::instructions::instruction_registry::register_instruction;
use ezasm_macro::instruction;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref add: InstructionField =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 + value2;
            output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
        });
    pub static ref sub: InstructionField =
        instruction!(sub, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 - value2;
            output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
        });
    pub static ref mul: InstructionField =
        instruction!(mul, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 * value2;
            output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
        });
    pub static ref div: InstructionField =
        instruction!(div, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 / value2;
            output.set(simulator, RawData::from_int(k, simulator.get_word_size()))
        });
}

pub fn register_instructions() {
    register_instruction(&add);
    register_instruction(&sub);
    register_instruction(&mul);
    register_instruction(&div);
}
