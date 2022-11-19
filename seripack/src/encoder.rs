// Relative Modules

// Standard Uses

// Crate Uses

// External Uses


pub trait Encoder {
    fn encode(&self, data: &mut Vec<u8>);
    fn decode(&self, data: &mut Vec<u8>);
}

