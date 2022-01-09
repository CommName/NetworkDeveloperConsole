use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use std::str;

use super::network_structures;

pub fn filter_ip(bytes: &[u8]) -> Option<i32> {
    if let Some(ipv4) = Ipv4Packet::new(&bytes[14..]) {
	if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
	    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
		if let Ok(body) =  str::from_utf8(tcp.payload()) {
                    println!("{}", body);
		}
	    }
	    
	}
    }
	
    None
}
