use crate::instructions::targets::Target;

pub trait OutputTarget: Target {
    fn set(data: Vec<u8>); //TODO simulator injection
}
