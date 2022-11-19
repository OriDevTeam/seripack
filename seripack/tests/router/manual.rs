// Relative Modules

// Standard Uses
use std::mem;
use std::mem::size_of;

// Crate Uses

// External Uses
use seripack::packet::{Builder, Packet};
use seripack::router::{Router, PacketId};
use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;


lazy_static! {
    static ref TEST_ROUTER: Router = {
        let router = Router::default();

        router
    };
}


#[derive(Pod, Zeroable)]
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct TestPacket {}
impl TestPacket { const HEADER: PacketId = 1; }

impl Packet for TestPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = bytemuck::cast::<Self, [u8; mem::size_of::<Self>()]>(
            self.clone()
        ).to_vec();

        bytes.insert(0, Self::HEADER);

        bytes
    }
}

#[allow(unused)]
impl Builder for TestPacket {
    fn from_bytes_boxed(data: Vec<u8>) -> Box<dyn Packet> {
        let packet: &TestPacket = bytemuck::from_bytes(data.as_slice());

        Box::new(*packet)
    }
}


#[allow(unused)]
pub fn recv_test_manual(pure_packet: Box<dyn Packet>) {
    let packet = pure_packet.downcast_ref::<TestPacket>().unwrap();
    println!("We got a sane test packet from routing! {:?}", packet);
}


#[test]
fn routing_scoped() {
    let mut router = Router::default();

    router.add_builder(
        TestPacket::HEADER,
        (size_of::<TestPacket>(), TestPacket::from_bytes_boxed)
    );

    router.add_receiver(TestPacket::HEADER, recv_test_manual);

    let test = TestPacket::default();
    let test_bytes = test.to_bytes();

    router.route(test_bytes).unwrap();
}

