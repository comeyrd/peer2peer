// Pure logic for UDP hole punching - no I/O, just algorithms

use std::net::{UdpSocket};
use std::io::Result;

pub fn create_udp_socket(
    addr : &str,
) -> Result<UdpSocket> {
    let socket = UdpSocket::bind(addr)?;
    println!("Created socket on local address: {}", socket.local_addr()?);
    Ok(socket)
}
