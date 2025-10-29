use tokio::net::TcpListener;
use tokio_util::codec::{ Framed, LengthDelimitedCodec };
use futures::{ SinkExt, StreamExt };
use bincode;
use shared::RpcMessage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server listening on 127.0.0.1:4000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let mut framed = Framed::new(socket, LengthDelimitedCodec::new());

            while let Some(Ok(bytes)) = framed.next().await {
                match bincode::deserialize::<RpcMessage>(&bytes) {
                    Ok(msg) => {
                        println!("Received {:?} from {}", msg, addr);

                        if let RpcMessage::Ping = msg {
                            let resp = RpcMessage::Pong;
                            let resp_bytes = bincode::serialize(&resp).unwrap();
                            if let Err(e) = framed.send(resp_bytes.into()).await {
                                eprintln!("Failed to send Pong to {}: {}", addr, e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize from {}: {}", addr, e);
                        break;
                    }
                }
            }

            println!("Connection closed: {}", addr);
        });
    }
}
