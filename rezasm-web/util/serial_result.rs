use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::mem::size_of_val;

pub enum SerialResult<T: Serialize, E: Serialize> {
    Ok(T),
    Err(E),
}

impl<T: Serialize, E: Serialize> Serialize for SerialResult<T, E>
where
    T: Serialize,
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerialResult::Ok(t) => {
                let mut serial_struct =
                    serializer.serialize_struct("SerialResult", size_of_val(&self))?;
                serial_struct.serialize_field("data", t)?;
                serial_struct.end()
            }
            SerialResult::Err(e) => {
                let mut serial_struct =
                    serializer.serialize_struct("SerialResult", size_of_val(&self))?;
                serial_struct.serialize_field("error", e)?;
                serial_struct.end()
            }
        }
    }
}
