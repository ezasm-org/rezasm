use std::any::TypeId;
use std::collections::HashMap;
use std::iter::zip;
use std::process::Output;
use crate::error::EzasmError;
use crate::instructions::targets::input_output_target::{InputOutput, InputOutputTarget};
use crate::instructions::targets::input_target::{Input, InputTarget};
use crate::instructions::targets::Target;
use crate::parser::line::Line;
use crate::simulation::simulator::Simulator;

pub trait AbstractInstruction {
    fn execute(&self, simulator: &Simulator);
}

// Hashmap<String, Pair<Method, Array<Types>>>
// Hashmap<String, (Closure, [TypeID])>


/*

({|sim: &Simulator, [x: AT(X), y: AT(Y), z: AT(Z)]| => ...}, [Enum<X>, TypeID<Y>, TypeID<Z>])

 */

#[derive(Debug)]
pub enum ArgumentType {
    InputOutput(InputOutputTarget),
    Input(InputTarget),
}

impl ArgumentType {
    pub fn is_input_output(&self) -> bool {
        match self {
            ArgumentType::InputOutput(_) => true,
            ArgumentType::Input(_) => false,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            ArgumentType::InputOutput(_) => false,
            ArgumentType::Input(_) => true,
        }
    }

    pub fn get_input(&self) -> Option<Box<&dyn Input>> {
        match self {
            ArgumentType::InputOutput(x) => return None,
            ArgumentType::Input(x) => Some(Box::new(x)),
        }
    }

    pub fn get_input_output(&self) -> Option<Box<&dyn InputOutput>> {
        match self {
            ArgumentType::InputOutput(x) => Some(Box::new(x)),
            ArgumentType::Input(x) => return None,
        }
    }

    pub fn into_input(self) -> Option<Box<dyn Input>> {
        match self {
            ArgumentType::InputOutput(x) => return None,
            ArgumentType::Input(x) => Some(Box::new(x)),
        }
    }

    pub fn into_input_output(self) -> Option<Box<dyn InputOutput>> {
        match self {
            ArgumentType::InputOutput(x) => Some(Box::new(x)),
            ArgumentType::Input(x) => return None,
        }
    }

    pub fn get_type_id(&self) -> TypeId {
        match self {
            ArgumentType::InputOutput(_) => TypeId::of::<InputOutputTarget>(),
            ArgumentType::Input(_) => TypeId::of::<InputTarget>(),
        }
    }
}

type TInstructionFunction = dyn Fn(&Simulator, Vec<ArgumentType>) -> ();

struct Instruction {
    function: Box<TInstructionFunction>,
    types: Vec<TypeId>,
}

pub fn add(simulator: &Simulator, args: Vec<ArgumentType>) -> () {
    ()
}

pub fn something() {
    let mut instructions: HashMap<String, Instruction> = HashMap::new();
    let mut simulator = Simulator::new();
    let arg_types = [TypeId::of::<InputOutputTarget>(), TypeId::of::<InputTarget>(), TypeId::of::<InputTarget>()].to_vec();
    let t = Instruction{
        function: Box::new(add),
        types: arg_types,
    };
    //some parsing to get your args
    let line: Line = Line::new(&String::from("add"), ["$T0".to_string(), "1".to_string(), "1".to_string()].to_vec()).unwrap();
    let args = match line {
        Line::Instruction(_, args) => args,
        _ => Vec::new(),
    };

    let targets: Vec<ArgumentType> = args.iter().map(|k| {simulator.get_target(k).unwrap()}).collect();
    instructions.insert("add".to_string(), t);

    for (t, a) in zip(&instructions.get("add").unwrap().types, &targets) {
        println!("{:?} == {:?}", t, a.get_type_id());
    }

    println!("{:?}", targets);
    // instruction_name(simulator, targets)
}

