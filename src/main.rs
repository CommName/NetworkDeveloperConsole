use std::thread;
use std::sync::mpsc::channel;
use std::time::Duration;
mod network;

fn main() {
    let interfaces = pnet::datalink::interfaces();
    let (sx, rx) = channel();
    for interface in interfaces {
	if interface.name == "lo" {
	let sender = sx.clone();
	let int = interface.clone();
	thread::spawn(move|| {
	    network::recive_packets::<i32>(&int, &network::filters::filter_ip, sender);
	});
	}
	println!("{:?}", interface);
    }

    


    let one_minute = Duration::new(60,0);
    std::thread::sleep(one_minute);

}
