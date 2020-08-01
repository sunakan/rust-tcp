use pnet::datalink::{self, NetworkInterface};
use std::env;

#[derive(Clone, Debug)]
struct PacketWithInterface {
    interface: NetworkInterface,
    packet: Vec<u8>,
}

fn main() {
    let network_interface_name = env::args().nth(1).unwrap();
    println!("Hello, world!");
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == network_interface_name)
        .expect("入力されたネットワークインターフェースが見つかりませんでした");
    println!("-----> {:?}", interface);
}
