use lazy_static::lazy_static;

use crate::simulation::registry;

use crate::instructions::instruction::instruction;
use crate::instructions::instruction::Instruction;
use crate::instructions::instruction_registry::register_instruction;

use crate::instructions::targets::input_output_target::InputOutputTarget;
use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::input_target::InputTarget;
use crate::simulation::transform::transformable::Transformable;
use crate::simulation::transform::transformation::Transformation;
use crate::simulation::transform::transformation_sequence::TransformationSequence;

lazy_static! {
    pub static ref BEQ: Instruction =
        instruction!(beq, |simulator: Simulator,
                           input1: InputTarget,
                           input2: InputTarget,
                           label: InputTarget| {
            let pc_num = registry::get_register_number(&registry::PC.to_string()).unwrap();
            let output = InputOutputTarget::RegisterInputOutput(pc_num);
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value1 == value2 {
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    label.get(simulator)?
                    );
                return Ok(TransformationSequence::new_single(transformation));
            } else {
                return Ok(TransformationSequence::new_empty());
            }
        });
    pub static ref BNE: Instruction =
        instruction!(bne, |simulator: Simulator,
                           input1: InputTarget,
                           input2: InputTarget,
                           label: InputTarget| {
            let pc_num = registry::get_register_number(&registry::PC.to_string()).unwrap();
            let output = InputOutputTarget::RegisterInputOutput(pc_num);
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value1 != value2 {
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    label.get(simulator)?
                    );
                return Ok(TransformationSequence::new_single(transformation));
            } else {
                return Ok(TransformationSequence::new_empty());
            }
        });
    pub static ref BLT: Instruction =
        instruction!(blt, |simulator: Simulator,
                           input1: InputTarget,
                           input2: InputTarget,
                           label: InputTarget| {
            let pc_num = registry::get_register_number(&registry::PC.to_string()).unwrap();
            let output = InputOutputTarget::RegisterInputOutput(pc_num);
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value1 < value2 {
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    label.get(simulator)?
                    );
                return Ok(TransformationSequence::new_single(transformation));
            } else {
                return Ok(TransformationSequence::new_empty());
            }
        });
    pub static ref BLE: Instruction =
        instruction!(ble, |simulator: Simulator,
                           input1: InputTarget,
                           input2: InputTarget,
                           label: InputTarget| {
            let pc_num = registry::get_register_number(&registry::PC.to_string()).unwrap();
            let output = InputOutputTarget::RegisterInputOutput(pc_num);
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value1 <= value2 {
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    label.get(simulator)?
                    );
                return Ok(TransformationSequence::new_single(transformation));
            } else {
                return Ok(TransformationSequence::new_empty());
            }
        });
    pub static ref BGT: Instruction =
        instruction!(bgt, |simulator: Simulator,
                           input1: InputTarget,
                           input2: InputTarget,
                           label: InputTarget| {
            let pc_num = registry::get_register_number(&registry::PC.to_string()).unwrap();
            let output = InputOutputTarget::RegisterInputOutput(pc_num);
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value1 > value2 {
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    label.get(simulator)?
                    );
                return Ok(TransformationSequence::new_single(transformation));
            } else {
                return Ok(TransformationSequence::new_empty());
            }
        });
    pub static ref BGE: Instruction =
        instruction!(bge, |simulator: Simulator,
                           input1: InputTarget,
                           input2: InputTarget,
                           label: InputTarget| {
            let pc_num = registry::get_register_number(&registry::PC.to_string()).unwrap();
            let output = InputOutputTarget::RegisterInputOutput(pc_num);
            let value1 = input1.get(&simulator)?.int_value();
            let value2 = input2.get(&simulator)?.int_value();
            if value1 >= value2 {
                let transformation = Transformation::new(
                    Transformable::InputOutputTransformable(output),
                    output.get(simulator)?,
                    label.get(simulator)?
                    );
                return Ok(TransformationSequence::new_single(transformation));
            } else {
                return Ok(TransformationSequence::new_empty());
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
