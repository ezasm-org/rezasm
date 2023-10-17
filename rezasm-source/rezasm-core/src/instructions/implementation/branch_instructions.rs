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
  //branch on equal
  //input1 = left hand side
  //input2 = right hand side
  //label = destination
  pub static ref BEQ: Instruction = 
    instruction!(beq, |simulator: Simulator,
                       input1: InputTarget,
                       input2: InputTarget,
                       label: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 == value2 {
        return output.set(simulator, ); //set some address to label??
      } else {
        return ();
      }
    });
  pub static ref BNE: Instruction = 
    instruction!(bne, |simulator: Simulator,
                       input1: InputTarget,
                       input2: InputTarget,
                       label: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 != value2 {
        return output.set(simulator, ); //set some address to label??
      } else {
        return output.set(); //don't change anything
      }
    });
  pub static ref BLT: Instruction =
    instruction!(blt, |simulator: Simulator,
                       input1: InputTarget,
                       input2: InputTarget,
                       label: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 < value2 {
        return output.set(simulator, ); //set some address to label??
      } else {
        return output.set(); //don't change anything
      }
    });
  pub static ref BLE: Instruction =
    instruction!(ble, |simulator: Simulator,
                        input1: InputTarget,
                        input2: InputTarget,
                        label: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 <= value2 {
        return output.set(simulator, ); //set some address to label??
      } else {
        return output.set(); //don't change anything
      }
    });
  pub static ref BGT: Instruction =
    instruction!(bgt, |simulator: Simulator,
                        input1: InputTarget,
                        input2: InputTarget,
                        label: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 > value2 {
        return output.set(simulator, ); //set some address to label??
      } else {
        return output.set(); //don't change anything
      }
    });
  pub static ref BGE: Instruction =
    instruction!(bge, |simulator: Simulator,
                        input1: InputTarget,
                        input2: InputTarget,
                        label: InputTarget| {
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 >= value2 {
        return output.set(simulator, ); //set some address to label??
      } else {
        return output.set(); //don't change anything
      }
    });
}