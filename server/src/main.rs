use shared::{LOCAL_HOST, DEFAULT_PORT, MessageSize, MESSAGE_SIZE_BYTES};
use std::io::{Read};
use std::net::{TcpListener};

fn main() {
    shared::hello_world();
    println!("Server starting");
    
    let address = format!("{}:{}", LOCAL_HOST, DEFAULT_PORT);
    let listener = TcpListener::bind(&address).expect("Could not bind");
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        println!("Client connected: {}", peer_addr);

        // Read message size (4 bytes)
        let mut size_buffer = [0u8; MESSAGE_SIZE_BYTES];
        stream.read_exact(&mut size_buffer).unwrap();
        let size = MessageSize::from_be_bytes(size_buffer) as usize;

        // Read the message
        let mut message_buffer = vec![0u8; size];
        stream.read_exact(&mut message_buffer).unwrap();
        let message = String::from_utf8(message_buffer).unwrap();
        
        println!("Received message: {}", message);
    }
    // use peer_addr and send back to client
}