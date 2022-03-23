use std::thread;
use std::sync::{
    Arc,
    mpsc::{ Sender, Receiver, channel}
};
use std::time::Duration;

mod network;
mod http;
mod gui;
mod store;

use store::Store;

fn subscribe_to_interfaces(sx: Sender<network::network_structures::Layer3Packet>) {
    let interfaces = pnet::datalink::interfaces();
    for interface in interfaces {
        let sender = sx.clone();
        let int = interface.clone();
        thread::spawn(move|| {
            network::recive_packets::<network::network_structures::Layer3Packet>(&int, &network::filters::filter_ip, sender);
        });
    }
}

fn init_store(rx: Receiver<network::network_structures::Layer3Packet>) -> Arc<Store> {
    let store = Arc::new(Store::new());
    let recive_store = store.clone();
    thread::spawn(move || {
	    while let Ok(message) = rx.recv() {
            recive_store.register_packet(message);
	    }
    });

    store
}

fn main() {
    let (sx, rx) = channel();

    subscribe_to_interfaces(sx);
    let store = init_store(rx);
    gui::set_up_gui(store);
}
