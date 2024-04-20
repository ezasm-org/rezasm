use std::any::Any;

/// A trait for downcasting to an `Any` type, both mutably and immutably.
pub trait AsAny {

    /// Returns a reference to a dynamic `Any` type.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to a dynamic `Any` type.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
