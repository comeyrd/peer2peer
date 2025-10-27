use std::env;
use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
fn main() {
    println!("TCP reverse client");
    let address: String = "74.208.133.17:9999".to_string();
    let mut stream = TcpStream::connect(address).unwrap();
    let my_socket = stream.local_addr().unwrap();
    println!("Connected");
    stream.shutdown(std::net::Shutdown::Both);

    let listener = TcpListener::bind(my_socket).unwrap();
    println!("Server launched");
    for stream in listener.incoming() {
        let mut str = stream.unwrap();
        let address = str.peer_addr().unwrap();
        println!("Received connexion : {:?}", address);
        let mut size_buffer = [0; 8];
        str.read_exact(&mut size_buffer);
        let size = u64::from_be_bytes(size_buffer) as usize;

        // Read the message of the given size
        let mut message_buffer = vec![0; size];
        str.read_exact(&mut message_buffer);
        let message = String::from_utf8(message_buffer).expect("Invalid UTF-8");
        println!("Received: {} ({} bytes)", message, size);
    }
    //stream.write(buf)

    //let args: Vec<String> = env::args().collect();

    //Create connexion with server
    //look the port
    //close client connexion
    //create server at socket
    //server tries connecting to client socket
}
