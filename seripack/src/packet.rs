// Standard Uses
use std::rc::Rc;
use std::fmt::{Debug, Display, Formatter};

// Crate Uses

// External Uses
use downcast_rs::{Downcast, impl_downcast};
use enum_dispatch::enum_dispatch;


#[enum_dispatch]
pub trait Packet: Downcast {
    fn to_bytes(&self) -> Vec<u8> { todo!() }
}
impl_downcast!(Packet);


/*
pub trait PacketHeader: Packet {
    const HEADER: u8;
}
*/

pub trait PacketBuilder: Packet {
    fn from_bytes_boxed(_data: Vec<u8>) -> Box<dyn Packet> { todo!() }
    fn from_bytes_rc(_data: Vec<u8>) -> Rc<dyn Packet> { todo!() }
}

impl Display for Box<dyn Packet> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", "Header")
    }
}

impl Debug for Box<dyn Packet> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", "Packet TODO")
    }
}