use lazy_static::lazy_static;
use crate::instruction_field;
use crate::instructions::instruction_field::InstructionField;
use crate::simulation::simulator::Simulator;
use crate::util::raw_data::RawData;

lazy_static! {
    pub static ref add: InstructionField = instruction_field!(add, |simulator: &mut Simulator, output: InputOutputTarget, input1: InputTarget, input2: InputTarget| -> (()) {
        let value1 = input1.get(&simulator)?.int_value();
        let value2 = input2.get(&simulator)?.int_value();
        let k = value1 + value2;
        let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
    });

    pub static ref sub: InstructionField = instruction_field!(add, |simulator: &mut Simulator, output: InputOutputTarget, input1: InputTarget, input2: InputTarget| -> (()) {
        let value1 = input1.get(&simulator)?.int_value();
        let value2 = input2.get(&simulator)?.int_value();
        let k = value1 - value2;
        let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
    });

    pub static ref mul: InstructionField = instruction_field!(add, |simulator: &mut Simulator, output: InputOutputTarget, input1: InputTarget, input2: InputTarget| -> (()) {
        let value1 = input1.get(&simulator)?.int_value();
        let value2 = input2.get(&simulator)?.int_value();
        let k = value1 * value2;
        let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
    });

    pub static ref div: InstructionField = instruction_field!(add, |simulator: &mut Simulator, output: InputOutputTarget, input1: InputTarget, input2: InputTarget| -> (()) {
        let value1 = input1.get(&simulator)?.int_value();
        let value2 = input2.get(&simulator)?.int_value();
        let k = value1 / value2;
        let _ = output.set(simulator, RawData::from_int(k, simulator.get_word_size()))?;
    });
}