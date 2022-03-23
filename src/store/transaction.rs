use std::sync::RwLock;
use std::collections::HashMap;
use crate::network::network_structures::Layer3Packet;
use std::net::IpAddr;

// TODO change ports to tree
pub struct Transactions {
    transactions: RwLock<HashMap<IpAddr,HashMap<u16, Transaction>>>
}

impl Transactions {

    pub fn new() -> Self {
        let transactions = RwLock::new(HashMap::new()); 
        Self {
            transactions
        }
    }

    pub fn get_destination_ips(&self) -> Vec<IpAddr> {
        self.transactions
            .read()
            .unwrap()
            .keys()
            .into_iter()
            .map(|ip| ip.clone())
            .collect()
    }

    pub fn add_send_packet(&self, packet: Layer3Packet) {
        let mut transactions = self.transactions.write().unwrap();

        if !transactions.contains_key(&packet.layer_3_sender_address) {
            transactions.insert(packet.layer_3_sender_address.clone(), HashMap::new());
        }

        transactions.get_mut(&packet.layer_3_sender_address)
            .unwrap()
            .insert(packet.layer_4_sender_address, Transaction::new(packet.layer_5_data));
    }

    pub fn add_recv_packet(&self, packet: Layer3Packet) {
        
    }
}

pub struct Transaction {
    bytes: RwLock<Vec<u8>>
}

impl Transaction {

    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: RwLock::new(bytes)
        }
    }

    pub fn append(self, mut bytes: Vec<u8>) {
        //self.bytes.write().unwrap().append(&mut bytes);
    }
}