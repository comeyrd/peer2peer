// Handles UDP NAT traversal on client side

use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;

pub fn contact_sever(
    socket: &UdpSocket,
    server_addr: &str,
) {
    // Set timeouts to avoid hanging forever
    socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    socket.set_write_timeout(Some(Duration::from_secs(5))).unwrap();

    // Parse server address
    let server: SocketAddr = server_addr.parse().unwrap();
    println!("Contacting registry server at: {}", server);

    // Send registration message to server
    let message = b"REGISTER";
    socket.send_to(message, server).unwrap();
    println!("Sent registration request to registry");

    // Wait for response
    let mut buf = [0u8; 1024];
    println!("Waiting for response from registry...");

    let (amt, _) = socket.recv_from(&mut buf).unwrap();
    let response = String::from_utf8_lossy(&buf[..amt]);
    println!("\n=== Registry Response ===");
    println!("{}", response.trim());
    println!("=========================\n");
}
