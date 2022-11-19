// Relative Modules

// Standard Uses

// Crate Uses

// External Uses
use seripack::router::Router;
use lazy_static::lazy_static;


lazy_static! {
    static ref TEST_ROUTER: Router = Router::default();
}


#[test]
#[allow(unused)]
fn routing_macros() {

    #[derive(Default, Debug)]
    // #[derive(Packet(0))] // Implements traits and sets the packet Id to 0
    pub struct TestPacket {}

    #[allow(unused)]
    // #[route(header=TestPacket, router=TestPacket)]
    // #[route_auto(TEST_ROUTER)]
    // #[route2(TestPacket . TEST_ROUTER)]
    pub fn recv_test(packet: TestPacket) {
        println!("We got a sane test packet from routing! {:?}", packet);
    }

    let test = TestPacket::default();

    // TEST_ROUTER.route(test.to_bytes()).unwrap();
}

/*
#[allow(unused)]
#[test]
fn routing_macros_idea() {

    #[derive(Default, Debug)]
    #[derive(Packet(Id, 0))] // Implements traits and Packet Id
    pub struct TestPacket {}

    #[route_packet]
    pub fn recv_test(packet: TestPacket) {
        println!("We got a sane test packet from routing! {:?}", packet);
    }

    let test = TestPacket::default();

    // TEST_ROUTER.route(test.to_bytes()).unwrap();
}
*/
