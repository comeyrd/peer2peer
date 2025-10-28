use std::io::{Write, Read};
use std::net::TcpStream;
use crate::config::{MessageSize, MESSAGE_SIZE_BYTES};

pub fn tcp_send_message(stream: &mut TcpStream, message: &str) {
    let message_bytes = message.as_bytes();
    let size = message_bytes.len() as MessageSize;
    
    // Write size first (4 bytes)
    stream.write_all(&size.to_be_bytes()).unwrap();
    
    // Write message
    stream.write_all(message_bytes).unwrap();
}

pub fn tcp_receive_message(stream: &mut TcpStream) -> String {
    // Read size first (4 bytes)
    let mut size_buffer = [0u8; MESSAGE_SIZE_BYTES];
    stream.read_exact(&mut size_buffer).unwrap();
    let size = MessageSize::from_be_bytes(size_buffer) as usize;
    
    // Read message
    let mut message_buffer = vec![0u8; size];
    stream.read_exact(&mut message_buffer).unwrap();
    
    return String::from_utf8(message_buffer).unwrap();
}