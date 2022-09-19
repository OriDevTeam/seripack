// Relative Modules
mod blosc;

// Standard Uses

// Crate Uses
use crate::router::encoder::Encoder;

// External Uses
use anyhow::Result;


pub trait Serializer {
    fn encoders(&self) -> &Vec<Box<dyn Encoder>>;

    fn serialize(&self, message: &mut Vec<u8>) -> Result<()>;
    fn deserialize(&self, message: &mut Vec<u8>) -> Result<()>;
}

