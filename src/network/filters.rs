use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use std::net::IpAddr;

use super::network_structures::Layer3Packet;

pub fn filter_ip(bytes: &[u8]) -> Option<Layer3Packet> {
    if let Some(ipv4) = Ipv4Packet::new(&bytes[14..]) {
        if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
            if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                let mut data = Vec::new();
                Vec::extend_from_slice(&mut data, tcp.payload());
                return Some(Layer3Packet {
                    layer_3_sender_address: IpAddr::V4(ipv4.get_source()),
                    layer_3_reciver_address: IpAddr::V4(ipv4.get_destination()),
                    layer_4_reciver_address: tcp.get_source(),
                    layer_4_sender_address: tcp.get_destination(),
                    layer_5_data: data,
                });
            }
        }
    }

    None
}
