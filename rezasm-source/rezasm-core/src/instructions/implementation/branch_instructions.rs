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

/*
Questions:
- Why can label.get(&simulator) be passed into output.set() when set() is looking for RawData and label.get(&simulator) returns Result<RawData, SimulatorError>
- Based on whatever was said to the previous question, does returning () get recognized as Result<(), SimulatorError>
- How do I even test anything I've made
*/

lazy_static! {
  pub static ref BEQ: Instruction = 
    instruction!(beq, |simulator: Simulator,
                       input1: InputTarget,
                       input2: InputTarget,
                       label: InputTarget| {
      let output = InputOutputTarget::RegisterInputOutput(simulator.get_registers_mut().get_pc_mut());
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 == value2 {
        return output.set(simulator, label.get(&simulator)); //set PC to label
      } else {
        return (); //do nothing
      }
    });
  pub static ref BNE: Instruction = 
    instruction!(bne, |simulator: Simulator,
                       input1: InputTarget,
                       input2: InputTarget,
                       label: InputTarget| {
      let output = InputOutputTarget::RegisterInputOutput(simulator.get_registers_mut().get_pc_mut());
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 != value2 {
        return output.set(simulator, label.get(&simulator));
      } else {
        return ();
      }
    });
  pub static ref BLT: Instruction =
    instruction!(blt, |simulator: Simulator,
                       input1: InputTarget,
                       input2: InputTarget,
                       label: InputTarget| {
      let output = InputOutputTarget::RegisterInputOutput(simulator.get_registers_mut().get_pc_mut());
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 < value2 {
        return output.set(simulator, label.get(&simulator));
      } else {
        return ();
      }
    });
  pub static ref BLE: Instruction =
    instruction!(ble, |simulator: Simulator,
                        input1: InputTarget,
                        input2: InputTarget,
                        label: InputTarget| {
      let output = InputOutputTarget::RegisterInputOutput(simulator.get_registers_mut().get_pc_mut());      
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 <= value2 {
        return output.set(simulator, label.get(&simulator));
      } else {
        return ();
      }
    });
  pub static ref BGT: Instruction =
    instruction!(bgt, |simulator: Simulator,
                        input1: InputTarget,
                        input2: InputTarget,
                        label: InputTarget| {
      let output = InputOutputTarget::RegisterInputOutput(simulator.get_registers_mut().get_pc_mut());
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 > value2 {
        return output.set(simulator, label.get(&simulator));
      } else {
        return ();
      }
    });
  pub static ref BGE: Instruction =
    instruction!(bge, |simulator: Simulator,
                        input1: InputTarget,
                        input2: InputTarget,
                        label: InputTarget| {
      let output = InputOutputTarget::RegisterInputOutput(simulator.get_registers_mut().get_pc_mut());
      let value1 = input1.get(&simulator)?.int_value();
      let value2 = input2.get(&simulator)?.int_value();
      if value1 >= value2 {
        return output.set(simulator, label.get(&simulator));
      } else {
        return ();
      }
    });
}

pub fn register_instructions() {
  register_instruction(&BEQ);
  register_instruction(&BNE);
  register_instruction(&BLT);
  register_instruction(&BLE);
  register_instruction(&BGT);
  register_instruction(&BGE);
}