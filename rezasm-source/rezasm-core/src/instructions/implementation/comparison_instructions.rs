use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::util::raw_data::RawData;

lazy_static! {
  pub static ref SEQ: Instruction =
    instruction!(seq, |simulator: Simulator,
                       output: InputOutputTarget,
                       input1: InputTarget,
                       input2: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 == value2 {
        return output.set(simulator, RawData::from_int(1, simulator.get_word_size()));
      } else {
        return output.set(simulator, RawData::from_int(0, simulator.get_word_size()));
      }
    });
  pub static ref SNE: Instruction =
    instruction!(sne, |simulator: Simulator,
                        output: InputOutputTarget,
                        input1: InputTarget,
                        input2: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 != value2 {
        return output.set(simulator, RawData::from_int(1, simulator.get_word_size()));
      } else {
        return output.set(simulator, RawData::from_int(0, simulator.get_word_size()));
      }
    });
  pub static ref SLT: Instruction =
    instruction!(slt, |simulator: Simulator,
                        output: InputOutputTarget,
                        input1: InputTarget,
                        input2: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 < value2 {
        return output.set(simulator, RawData::from_int(1, simulator.get_word_size()));
      } else {
        return output.set(simulator, RawData::from_int(0, simulator.get_word_size()));
      }
    });
  pub static ref SLE: Instruction =
    instruction!(sle, |simulator: Simulator,
                        output: InputOutputTarget,
                        input1: InputTarget,
                        input2: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 <= value2 {
        return output.set(simulator, RawData::from_int(1, simulator.get_word_size()));
      } else {
        return output.set(simulator, RawData::from_int(0, simulator.get_word_size()));
      }
    });
  pub static ref SGT: Instruction =
    instruction!(sgt, |simulator: Simulator,
                        output: InputOutputTarget,
                        input1: InputTarget,
                        input2: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 > value2 {
        return output.set(simulator, RawData::from_int(1, simulator.get_word_size()));
      } else {
        return output.set(simulator, RawData::from_int(0, simulator.get_word_size()));
      }
    });
  pub static ref SGE: Instruction =
    instruction!(sge, |simulator: Simulator,
                        output: InputOutputTarget,
                        input1: InputTarget,
                        input2: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 >= value2 {
        return output.set(simulator, RawData::from_int(1, simulator.get_word_size()));
      } else {
        return output.set(simulator, RawData::from_int(0, simulator.get_word_size()));
      }
    });
}

pub fn register_instructions() {
  register_instruction(&SEQ);
  register_instruction(&SNE);
  register_instruction(&SLT);
  register_instruction(&SLE);
  register_instruction(&SGT);
  register_instruction(&SGE);
}
