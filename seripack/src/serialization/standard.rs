// Standard Uses

// Crate Uses
use crate::router::encoder::Encoder;
use crate::serialization::serializers::Serializer;

// External Uses
use anyhow::Result;


#[derive(Default)]
pub struct Standard {
    encoders: Vec<Box<dyn Encoder>>
}

impl Serializer for Standard {
    fn encoders(&self) -> &Vec<Box<dyn Encoder>> {
        &self.encoders
    }

    fn serialize(&self, _message: &mut Vec<u8>) -> Result<()> {
        todo!()
    }

    fn deserialize(&self, _message: &mut Vec<u8>) -> Result<()> {
        todo!()
    }
}

