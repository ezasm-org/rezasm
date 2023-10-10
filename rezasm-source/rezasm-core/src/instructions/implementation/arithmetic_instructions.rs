use std::f64::NAN;

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
            // This may not work, requires further testing!
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
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            // Bug: negative values not accounted for!
            let k = value1 << value2;
            return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
        });
    pub static ref SRL: Instruction =
        instruction!(srl, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            // Bug: negative values not accounted for!
            let k = value1 >> value2;
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
    pub static ref DECF: Instruction =
        instruction!(decf, |simulator: Simulator,
                           output: InputOutputTarget| {
            let value1 = output.get(&simulator)?.float_value();
            let k = value1 - 1.0;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref INCF: Instruction =
        instruction!(incf, |simulator: Simulator,
                           output: InputOutputTarget| {
            let value1 = output.get(&simulator)?.float_value();
            let k = value1 + 1.0;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref ADDF: Instruction =
        instruction!(addf, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
            let value2 = input2.get(&simulator)?.float_value();
            let k = value1 + value2 ;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref SUBF: Instruction =
        instruction!(subf, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
            let value2 = input2.get(&simulator)?.float_value();
            let k = value1 - value2 ;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref MULF: Instruction =
        instruction!(mulf, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
            let value2 = input2.get(&simulator)?.float_value();
            let k = value1 * value2 ;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref DIVF: Instruction =
        instruction!(divf, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
            let value2 = input2.get(&simulator)?.float_value();
            
            if value2 == 0f64 {
                if value1 == 0f64 {
                    return output.set(simulator, RawData::from_float(NAN, simulator.get_word_size()));
                }   
                return Err(SimulatorError::DivideByZeroError);
            }
            let k = value1 / value2 ;
            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref MODF: Instruction =
        instruction!(modf, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget,
                           input2: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
            let value2 = input2.get(&simulator)?.float_value();
            
            if value2 == 0f64 {
                if value1 == 0f64 {
                    return output.set(simulator, RawData::from_float(NAN, simulator.get_word_size()));
                }   
                return Err(SimulatorError::DivideByZeroError);
            }

            // Used to determine the sign of the mod - note: this may not be necessary and am leaving it
            // here for now in case it is
            // let n = match (value1 < 0f64 || value2 < 0f64) && !(value1 < 0f64 && value2 < 0f64) {
            //     true => value1 - (f64::floor(value1/value2)) * value2,
            //     false => ...,
            // };

            let divvalue1 = f64::abs(value1);
            let divvalue2 = f64::abs(value2);

            let k = divvalue1 - f64::floor(divvalue1/divvalue2) * divvalue2;

            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref ITOF: Instruction =
        instruction!(itof, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget| {
            let value1 = input1.get(&simulator)?.int_value();
            
            let k = value1 as f64;

            return output.set(simulator, RawData::from_float(k, simulator.get_word_size()));
        });
    pub static ref FTOI: Instruction =
        instruction!(ftoi, |simulator: Simulator,
                           output: InputOutputTarget,
                           input1: InputTarget| {
            let value1 = input1.get(&simulator)?.float_value();
            
            // This still bugs, does not account properly for the 0 case!
            let k = f64::ceil(value1 - 1.0f64) as i64;

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
    register_instruction(&INC);
    register_instruction(&DEC);

    register_instruction(&ADDF);
    register_instruction(&SUBF);
    register_instruction(&MULF);
    register_instruction(&DIVF);
    register_instruction(&MODF);
    register_instruction(&DECF);
    register_instruction(&INCF);
    register_instruction(&ITOF);
    register_instruction(&FTOI);

}
