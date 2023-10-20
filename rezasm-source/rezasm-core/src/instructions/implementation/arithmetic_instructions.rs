use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::util::error::SimulatorError;
use crate::util::raw_data::RawData;

lazy_static! {
    pub static ref ADD: Instruction =
        instruction!(add, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 + value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref SUB: Instruction =
        instruction!(sub, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 - value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref MUL: Instruction =
        instruction!(mul, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 * value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref DIV: Instruction =
        instruction!(div, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value2 == 0 {
                return Err(SimulatorError::DivideByZeroError);
            } else {
                let k = value1 / value2;
                return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
            }
        });
    pub static ref AND: Instruction =
        instruction!(and, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 & value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref OR: Instruction =
        instruction!(or, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 | value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref XOR: Instruction =
        instruction!(xor, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            let k = value1 ^ value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref NOT: Instruction =
        instruction!(not, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget| {
                let value1 = input1.get(&simulator)?.int_value();
                let k = !value1;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref MOD: Instruction =
        instruction!(_mod, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value2 == 0 {
                return Err(SimulatorError::DivideByZeroError);
            } else {
                let k = value1 % value2;
                return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
            }
        });
    pub static ref SLL: Instruction =
        instruction!(sll, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let maxshift = (simulator.get_word_size().value() as i64 * 8) - 1;
            let value1 = input1.get(&simulator)?.int_value();
            let negativeshift = input2.get(&simulator)?.int_value() < 0;
            let shift = input2.get(&simulator)?.int_value().abs().min(maxshift);

            let k = match negativeshift {
                true => value1 >> shift,
                false => value1 << shift,
            };
            
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref SRL: Instruction =
        instruction!(srl, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let maxshift = (simulator.get_word_size().value() as i64 * 8) - 1;
            let value1 = input1.get(&simulator)?.int_value();
            let negativeshift = input2.get(&simulator)?.int_value() < 0;
            let shift = input2.get(&simulator)?.int_value().abs().min(maxshift);

            let n = if value1 < 0 {-1} else {1};
            
            // Account for negative shifts
            let k = match negativeshift {
                true => n * (value1 << shift),
                false => n * (value1 >> shift),
            };

            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref SRA: Instruction =
        instruction!(sra, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let maxshift = (simulator.get_word_size().value() as i64 * 8) - 1;
            let value1 = input1.get(&simulator)?.int_value();
            let negativeshift = input2.get(&simulator)?.int_value() < 0;
            let shift = input2.get(&simulator)?.int_value().abs().min(maxshift);

            let k = match negativeshift {
                true => value1 << shift,
                false => value1 >> shift,
            };
                       
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref INC: Instruction =
        instruction!(inc, |simulator: Simulator,
                           output: InputOutputTarget| {
            let value1 = output.get(&simulator)?.int_value();
            let k = value1 + 1;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref DEC: Instruction =
        instruction!(dec, |simulator: Simulator,
                           output: InputOutputTarget| {
            let value1 = output.get(&simulator)?.int_value();
            let k = value1 - 1;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });

}

pub fn register_instructions() {
    register_instruction(&ADD);
    register_instruction(&SUB);
    register_instruction(&MUL);
    register_instruction(&DIV);
    register_instruction(&AND);
    register_instruction(&OR);
    register_instruction(&XOR);
    register_instruction(&MOD);
    register_instruction(&NOT);
    register_instruction(&SLL);
    register_instruction(&SRL);
    register_instruction(&SRA);
    register_instruction(&INC);
    register_instruction(&DEC);
}
