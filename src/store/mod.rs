use std::collections::HashMap;
use std::sync::RwLock;
use crate::network::network_structures::Layer3Packet;
use crate::http::parse_bytes;

mod http;
mod transaction;

use transaction::Transactions;
use http::HttpStore;

pub struct Store {
    // TODO: add interfaces
    http: HttpStore,
    transactions: Transactions,
}


impl Store {

    pub fn new() -> Self {
	    let http = HttpStore::new();
        let transactions = Transactions::new();

	    Self{
	        http,
            transactions
	    }
    }

    pub fn get_destinations(&self) -> Vec<String> {
        self.transactions
            .get_destination_ips()
            .iter()
            .map(|ip| format!("{}", ip))
            .collect()
    }

    pub fn register_packet(&self, packet: Layer3Packet) {

        self.register_send_packet(packet);
    }

    fn register_send_packet(&self, packet: Layer3Packet) {
        //match parse_bytes(&packet.layer_5_data) {

        //}
        self.transactions.add_send_packet(packet);
    }

}
