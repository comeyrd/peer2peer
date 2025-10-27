use std::{ io::Write, net::{ TcpListener, TcpStream } };

fn handle_client(stream: TcpStream) {
    println!("{:?}", stream.peer_addr());
}
fn main() {
    println!("Hello, Server!");
    let address: String = "127.0.0.1:9999".to_string();

    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let address = stream.unwrap().peer_addr().unwrap();
        println!("Received : {:?}", address);
        let mut stream = TcpStream::connect(address).unwrap();
        println!("Connected to stream");
        let str = "Stream is good";
        let message_bytes = str.as_bytes(); // Convert string to bytes
        let size = message_bytes.len() as u64;
        stream.write_all(&size.to_be_bytes());
        stream.write_all(&message_bytes);
    }
    //Create connexion with server
    //look the port
    //close client connexion
    //create server at socket
    //server tries connecting to client socket
}
