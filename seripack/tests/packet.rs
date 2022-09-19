// Relative Modules

// Standard Uses

// Crate Uses

// External Uses
use seripack::packet::Packet;
use seripack::router::{PacketId};
// use seripack::serialization::standard::Standard;
// use lazy_static::lazy_static;

/*
lazy_static! {
    static ref SERIALIZATION: Standard = Standard::default()
        .add_set();

    static ref TEST_ROUTER: Router = Router::new()
        .add_mutator(SERIALIZATION);
}
*/

#[test]
fn test_routing() {

    #[derive(Debug)]
    pub struct TestPacket {}
    impl TestPacket { const HEADER: PacketId = 0; }
    impl Packet for TestPacket {}

    // TODO: Test routing for struct associated functions

    // #[route(header=TestPacket, router=TestPacket)]
    // #[route_auto(TEST_ROUTER)]
    // #[route2(TestPacket . TEST_ROUTER)]
    pub fn recv_test(packet: TestPacket) {
        println!("We got a sane test packet from routing! {:?}", packet);
    }

    pub fn recv_test_manual(pure_packet: Box<dyn Packet>) {
        let packet = pure_packet.downcast_ref::<TestPacket>().unwrap();
        println!("We got a sane test packet from routing! {:?}", packet);
    }

}
