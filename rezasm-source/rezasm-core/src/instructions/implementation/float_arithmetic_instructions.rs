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

        // This has a "feature" due to lack of precision in float representations for some floats... This is
        // not a particularly fixable thing without getting too complicated.
        let k = value1 as i64;

        return output.set(simulator, RawData::from_int(k, simulator.get_word_size()));
    });
}

pub fn register_instructions() {
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
