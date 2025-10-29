use tokio::net::TcpStream;
use tokio_util::codec::{ Framed, LengthDelimitedCodec };
use futures::{ SinkExt, StreamExt };
use bincode;
use shared::{ RpcMessage, PingPongMessage, ServerMessage, ServerResponse };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:4000").await?;
    let mut framed = Framed::new(socket, LengthDelimitedCodec::new());

    // 1. Send Ping
    let ping = RpcMessage::PingPong(PingPongMessage::Ping);
    let bytes = bincode::serialize(&ping)?;
    framed.send(bytes.into()).await?;
    println!("Sent Ping, waiting for Pong...");

    // 2. Wait for Pong
    while let Some(Ok(bytes)) = framed.next().await {
        if let Ok(msg) = bincode::deserialize::<RpcMessage>(&bytes) {
            match msg {
                RpcMessage::PingPong(PingPongMessage::Pong) => {
                    println!("Received Pong!");
                    break; // On sort de la boucle
                }
                _ => {
                    println!("Received unexpected message, ignoring");
                }
            }
        }
    }

    // 3. Send Register
    let register = RpcMessage::Server(ServerMessage::Register(1)); // topic_id = 1 par exemple
    let bytes = bincode::serialize(&register)?;
    framed.send(bytes.into()).await?;
    println!("Sent Register, waiting for PeerInfo...");

    // 4. Wait for PeerInfo
    while let Some(Ok(bytes)) = framed.next().await {
        if let Ok(msg) = bincode::deserialize::<RpcMessage>(&bytes) {
            match msg {
                RpcMessage::Response(ServerResponse::PeerInfo(peer_info)) => {
                    println!("Received PeerInfo!");
                    println!("  Peer address: {}", peer_info.addr);
                    println!("  Should serve: {}", peer_info.serve);
                    break; // On sort de la boucle
                }
                _ => {
                    println!("Received unexpected message, ignoring");
                }
            }
        }
    }

    println!("Done!");
    Ok(())
}
