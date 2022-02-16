use std::thread;
use std::sync::mpsc::{ Sender, Receiver, channel};
use std::time::Duration;

mod network;
mod http;
mod gui;


fn subscribe_to_interfaces(sx: Sender<network::network_structures::Layer3Packet>) {
    let interfaces = pnet::datalink::interfaces();
    for interface in interfaces {
	if interface.name == "lo" {
	let sender = sx.clone();
	let int = interface.clone();
	thread::spawn(move|| {
	    network::recive_packets::<network::network_structures::Layer3Packet>(&int, &network::filters::filter_ip, sender);
	});
	}
	println!("{:?}", interface);
    }
}

fn init_store(rx: Receiver<network::network_structures::Layer3Packet>) {

    thread::spawn(move || {
	while let Ok(message) = rx.recv() {
              http::from_raw(&message.layer_5_data);   
	}
    });
}

fn main() {

    let (sx, rx) = channel();
    //
    subscribe_to_interfaces(sx);
    init_store(rx);
    let one_minute = Duration::new(60,0);
    std::thread::sleep(one_minute);

}
