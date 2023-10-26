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
            Ok(())
        });
}

pub fn register_instructions() {
    register_instruction(&PUSH)
}
