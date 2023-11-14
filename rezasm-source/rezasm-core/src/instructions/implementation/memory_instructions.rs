use lazy_static::lazy_static;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::instructions::targets::output_target::Output;
use crate::simulation::registry;
use crate::util::raw_data::RawData;

lazy_static! {
    pub static ref PUSH: Instruction =
        instruction!(push, |simulator: Simulator, input: InputTarget| {
            let ws = simulator.get_word_size().clone();
            let data = input.get(simulator)?;
            let sp = simulator
                .get_registers_mut()
                .get_register_mut(&registry::SP.into())?
                .get_data()
                .int_value()
                - ws.value() as i64;
            simulator
                .get_registers_mut()
                .get_register_mut(&registry::SP.into())?
                .set_data(RawData::from_int(sp, &ws));
            simulator.get_memory_mut().write(sp as usize, &data)
        });
    pub static ref POP: Instruction =
        instruction!(pop, |simulator: Simulator, output: InputOutputTarget| {
            let ws = simulator.get_word_size().clone();
            let wsv = ws.value() as i64;
            let sp = simulator
                .get_registers_mut()
                .get_register_mut(&registry::SP.into())?
                .get_data()
                .int_value();
            output.set(simulator, simulator.get_memory().read(sp as usize)?)?;
            simulator
                .get_registers_mut()
                .get_register_mut(&registry::SP.into())?
                .set_data(RawData::from_int(sp + wsv, &ws));
            Ok(())
        });
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
            let offset = input.get(simulator)?.int_value() as usize;
            let memory = simulator.get_memory_mut();
            let heap_pointer = memory.current_heap_pointer();
            memory.set_heap_pointer(heap_pointer + offset)?;
            let bytes = RawData::from_int(heap_pointer as i64, simulator.get_word_size());
            output.set(simulator, bytes)
        });
    pub static ref MOVE: Instruction =
        instruction!(_move, |simulator: Simulator,
                             output: InputOutputTarget,
                             input: InputTarget| {
            output.set(simulator, input.get(simulator)?)
        });
}

pub fn register_instructions() {
    register_instruction(&PUSH);
    register_instruction(&POP);
    register_instruction(&LOAD);
    register_instruction(&STORE);
    register_instruction(&ALLOC);
    register_instruction(&MOVE);
}
