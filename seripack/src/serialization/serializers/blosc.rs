// Standard Uses

// Crate Uses
// use crate::serialization::serializers::Serializer;

// External Uses
// use anyhow::Result;
// use blosc::{Buffer, Context};

/*
pub struct Blosc {}

impl Serializer for Blosc {
    /*
    fn new() -> Self {
        Self {
            ctx: blosc::Context::new()
        }
    }
    */

    fn serialize(&self, message: &mut Vec<u8>) -> Result<()> {
        let ctx = blosc::Context::new();
        let compressed = ctx.compress(&message);
        *message = Vec::try_from(compressed).unwrap();

        Ok(())
    }

    fn deserialize(&self, message: &mut Vec<u8>) -> Result<()> {
        let ctx = Buffer::into(message);
        *message = blosc::decompress().unwrap();

        Ok(())
    }
}
*/
