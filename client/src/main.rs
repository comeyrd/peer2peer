use shared::{SERVER_IP, SERVER_PORT};
use std::net::{UdpSocket};
mod handlers;
use handlers::udp;

fn main() {
    println!("Starting TCP client");

    let local_address = "0.0.0.0:0"; // lets the OS pick the port
    let server_address = format!("{}:{}", SERVER_IP, SERVER_PORT);

    // Creates UDP socket
    let socket = UdpSocket::bind(local_address).unwrap();
    println!("Created socket on local address: {}", socket.local_addr().unwrap());

    // Tries to contact the server at the specified address and print the information received
    udp::contact_sever(&socket, &server_address);
}

/*
use shared::nat::tcp::{tcp_send_message};
use std::net::TcpStream;
let mut stream = TcpStream::connect(address).unwrap();
    println!("TCP client connected");

    // Send a message
    let message = "Hello from client!";
    tcp_send_message(&mut stream, message);
    println!("Message sent: {}", message);
*/