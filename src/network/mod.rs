use pnet::datalink::Channel;
use std::sync::mpsc::Sender;

pub mod filters;
pub mod network_structures;

pub fn recive_packets<T>(
    interface: &pnet::datalink::NetworkInterface,
    filter: &dyn Fn(&[u8]) -> Option<T>,
    send_channel: Sender<T>,
) {

        let (_, mut reciver) = match pnet::datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unkown channel type"),
        Err(e) => panic!("Error happend {}", e),
    };

    loop {
        let message = reciver.next();
        if let Ok(buff) = message {
            if let Some(result) = filter(buff) {
                let result = send_channel.send(result);
                if result.is_err() {
                    break;
                }
            }
        }
    }
}
