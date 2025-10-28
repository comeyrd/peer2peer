use shared::{LOCAL_HOST, SERVER_PORT};
mod handlers;
use handlers::udp;

// TODO

// Registration phase ==> hole punched and server keeps the address info
// Once both PC-A and PC-B addresses are available, forward them
// After this, the connection connot be reused for connecting the peers
// As only the registry server can pass through the hole

// Connection phase ==> second hole is punched specifically between peers
// First udp packet to arrive is dropped because the NAT rule does not exist
// Second udp packet to arrive is forwarded as the hole already exists, connection is established

fn main() {
    shared::hello_world();
    println!("Server starting");
    let address = format!("{}:{}", LOCAL_HOST, SERVER_PORT);
    udp::registry_server(&address).unwrap(); // answers clients by sending them back their address and port
}

/* 
let listener = TcpListener::bind(&address).expect("Could not bind");
println!("Server listening on {}", address);

for stream in listener.incoming() {
    let mut curr_stream = stream.unwrap();
    let peer_addr = curr_stream.peer_addr().unwrap();
    println!("Client connected: {}", peer_addr);

    let message = tcp_receive_message(&mut curr_stream);
    println!("Received message: {}", message);
*/ 