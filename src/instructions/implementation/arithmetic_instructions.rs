use ezasm_core::instructions::instruction_field::InstructionField;
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
            let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
            Ok(())
        });
    pub static ref sub: InstructionField =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 - value2;
            let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
            Ok(())
        });
    pub static ref mul: InstructionField =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 * value2;
            let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
            Ok(())
        });
    pub static ref div: InstructionField =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 / value2;
            let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
            Ok(())
        });
}
