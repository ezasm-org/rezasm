use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub mod input_output_target;
pub mod input_target;
pub mod output_target;

pub trait Target: Any {
    fn as_any(&self) -> &dyn Any;
}
