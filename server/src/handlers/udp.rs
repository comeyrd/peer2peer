// Handle UDP NAT traversal coordination on server

use std::net::{UdpSocket};
use std::io::Result;

// Registry server that responds with client's public address
pub fn registry_server(bind_addr: &str) -> Result<()> {
    let socket = UdpSocket::bind(bind_addr)?;
    println!("Registry server started");
    println!("Listening on: {}", socket.local_addr()?);
    println!("Waiting for clients to register...\n");
    
    let mut buf = [0u8; 1024];
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let message = String::from_utf8_lossy(&buf[..amt]);
                
                println!("Received {} bytes from {}", amt, src);
                println!("Message: {}", message.trim());
                
                // Send back the client's public address
                let response = format!("Your public address is : {}", src);
                socket.send_to(response.as_bytes(), src)?;
                
                println!("Sent \"{}\" to the client", response);
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}