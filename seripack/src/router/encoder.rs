// Standard Uses

// Crate Uses
use crate::serialization::serializers::Serializer;

// External Uses


pub trait Encoder {
    fn serializers(&self) -> &Vec<Box<dyn Serializer>>;
    fn add_serializer(&mut self, serializer: Box<dyn Serializer>);

    fn encode(&self, data: &mut Vec<u8>) {
        for serializer in self.serializers() {
            serializer.serialize(data).unwrap()
        }
    }

    fn decode(&self, data: &mut Vec<u8> ) {
        for serializer in self.serializers().iter().rev() {
            serializer.deserialize(data).unwrap()
        }
    }
}
