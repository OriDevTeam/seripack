// Standard Uses

// Crate Uses

// External Uses
use downcast_rs::{Downcast, impl_downcast};
use enum_dispatch::enum_dispatch;


#[enum_dispatch]
pub trait Packet: Downcast {
    /// Clones structure data and casts it into a byte vector
    fn to_bytes(&self) -> Vec<u8>;
}
impl_downcast!(Packet);

pub trait Header: Packet { const HEADER: u8; }


pub trait Builder: Packet {
    /// Create boxed packet instance from bytes
    fn from_bytes_boxed(data: Vec<u8>) -> Box<dyn Packet>;
}

