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
    /*
    pub static ref PUSH: Instruction =
        instruction!(push, |simulator: Simulator, input: InputTarget| {
            let word_size = simulator.get_word_size().clone();
            let sp = simulator
                .get_registers_mut()
                .get_register_mut(&"$sp".into())?;
            let rd1 = RawData::from_int(
                sp.get_data().int_value() - word_size.value() as i64,
                &word_size,
            );

            let rd2 = input.get(simulator)?;

            println!("{:?} {:?}", rd1.int_value(), rd2.int_value());
            Ok(())
        });
    */
    pub static ref LOAD: Instruction =
        instruction!(load, |simulator: Simulator, 
                            output: InputOutputTarget,
                            input: InputTarget| {
            let memory = simulator.get_memory();
            let word = memory.read(input.get(simulator)?.int_value() as usize)?;
            output.set(simulator, word)
        });
    
    pub static ref STORE: Instruction =
        instruction!(store, |simulator: Simulator,
                            input1: InputTarget,
                            input2: InputTarget| {
            let address = input2.get(simulator)?.int_value() as usize;
            let data = input1.get(simulator)?;
            let memory = simulator.get_memory_mut();
            memory.write(address, &data)
        });

    pub static ref ALLOC: Instruction =
        instruction!(alloc, |simulator: Simulator,
                            output: InputOutputTarget,
                            input: InputTarget| {
            let memory = simulator.get_memory();
            let bytes = RawData::from_int(input.get(simulator)?.int_value(), memory.word_size());
            output.set(simulator, bytes)
        });
}

pub fn register_instructions() {
    // register_instruction(&PUSH);
    register_instruction(&LOAD);
    register_instruction(&STORE);
    register_instruction(&ALLOC);
}
