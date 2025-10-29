use tokio::net::TcpListener;
use tokio_util::codec::{ Framed, LengthDelimitedCodec };
use futures::{ SinkExt, StreamExt };
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::sync::{ Arc };
use bincode;
use shared::{ RpcMessage, ServerMessage, ServerResponse, PingPongMessage };

// Struct to store each peer's sender and address
struct Peer {
    tx: mpsc::Sender<RpcMessage>,
    addr: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server listening on 127.0.0.1:4000");

    // Keep track of two peers
    let peers: Arc<Mutex<Vec<Peer>>> = Arc::new(Mutex::new(Vec::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New peer: {}", addr);

        let peers = peers.clone();
        tokio::spawn(async move {
            handle_peer(socket, addr, peers).await;
        });
    }
}
async fn handle_peer(
    socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    peers: Arc<Mutex<Vec<Peer>>>
) {
    let framed = Framed::new(socket, LengthDelimitedCodec::new());

    // Split framed into reader and writer
    let (mut writer, mut reader) = framed.split();

    // Queue pour envoyer des messages
    let (tx, mut rx) = mpsc::channel::<RpcMessage>(10);

    // Writer task possède maintenant juste le writer
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let bytes = bincode::serialize(&msg).unwrap();
            let _ = writer.send(bytes.into()).await;
        }
    });

    // Main loop utilise maintenant juste le reader
    while let Some(Ok(bytes)) = reader.next().await {
        if let Ok(msg) = bincode::deserialize::<RpcMessage>(&bytes) {
            match msg {
                RpcMessage::Server(ServerMessage::Register(_)) => {
                    println!("Peer registered: {}", addr);
                    let mut locked = peers.lock().await;
                    locked.push(Peer { tx: tx.clone(), addr });

                    if locked.len() == 2 {
                        let peer1_tx = locked[0].tx.clone();
                        let peer2_tx = locked[1].tx.clone();
                        let peer1_addr = locked[0].addr;
                        let peer2_addr = locked[1].addr;
                        locked.clear();
                        drop(locked);
                        let _ = peer1_tx.send(
                            RpcMessage::Response(
                                ServerResponse::PeerInfo(shared::PeerInfoData {
                                    addr: peer2_addr.to_string(),
                                    serve: true,
                                })
                            )
                        ).await;

                        let _ = peer2_tx.send(
                            RpcMessage::Response(
                                ServerResponse::PeerInfo(shared::PeerInfoData {
                                    addr: peer1_addr.to_string(),
                                    serve: false,
                                })
                            )
                        ).await;

                        println!("Sent PeerInfo to both peers. Closing.");

                        break;
                    }
                }
                RpcMessage::PingPong(pp) =>
                    match pp {
                        PingPongMessage::Ping => {
                            let _ = tx.send(RpcMessage::PingPong(PingPongMessage::Pong)).await;
                            println!("Received Ping! -> sending pong");
                        }
                        PingPongMessage::Pong => {
                            println!("Received Pong!");
                        }
                    }
                _ => {}
            }
        } else {
            println!("Received invalid message, ignoring");
        }
    }
}
