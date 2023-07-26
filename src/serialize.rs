use alloc::{string::String, fmt};

#[derive(Clone)]
pub enum SerializeErr {
    FailedJsonEncode(String),
    CannotSerialize(String),
    #[cfg(feature = "hematite-nbt")]
    NbtError(std::rc::Rc<nbt::Error>)
}

impl fmt::Display for SerializeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SerializeErr::*;
        match self {
            FailedJsonEncode(data) => {
                f.write_fmt(format_args!("failed to serialize json: {:?}", data))
            }
            CannotSerialize(message) => {
                f.write_fmt(format_args!("cannot serialize value, invalid representation: {:?}", message))
            }
            #[cfg(feature = "hematite-nbt")]
            NbtError(err) => {
                f.write_fmt(format_args!("nbt error: {:?}", err))
            }
        }
    }
}

impl fmt::Debug for SerializeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn fmt::Display>::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SerializeErr {}

pub type SerializeResult = Result<(), SerializeErr>;

pub trait Serialize: Sized {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult;
}

pub trait Serializer: Sized {
    fn serialize_bytes(&mut self, data: &[u8]) -> SerializeResult;

    fn serialize_byte(&mut self, byte: u8) -> SerializeResult {
        self.serialize_bytes(&[byte])
    }

    fn serialize_other<S: Serialize>(&mut self, other: &S) -> SerializeResult {
        other.mc_serialize(self)
    }
}
