use shared::{LOCAL_HOST, DEFAULT_PORT, MessageSize};
use std::io::Write;
use std::net::TcpStream;
use std::net::Shutdown;

fn main() {
    shared::hello_world();
    println!("Starting TCP client");

    let address = format!("{}:{}", LOCAL_HOST, DEFAULT_PORT);
    let mut stream = TcpStream::connect(address).unwrap();
    println!("TCP client connected");

    // Send a message
    let message = "Hello from client!";
    let message_bytes = message.as_bytes();
    let size = message_bytes.len() as MessageSize;

    // Write size first (4 bytes)
    stream.write_all(&size.to_be_bytes()).unwrap();
    
    // Write message
    stream.write_all(message_bytes).unwrap();

    println!("Message sent: {}", message);
    stream.shutdown(Shutdown::Write).expect("shutdown call failed");
}