use std::net::IpAddr;

pub struct Layer3Packet {
    pub layer_3_sender_address: IpAddr,
    pub layer_3_reciver_address: IpAddr,
    pub layer_4_sender_address: u32,
    pub layer_4_reciver_address: u32,
    pub layer_5_data: Vec<u8>,
}


impl Layer3Packet {

}
