use tokio::net::TcpStream;
use tokio_util::codec::{ Framed, LengthDelimitedCodec };
use futures::{ SinkExt, StreamExt };
use bincode;
use shared::RpcMessage;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let addr: String = if args.len() > 1 { args[1].clone() } else { "127.0.0.1:4000".to_string() };
    let socket: TcpStream = TcpStream::connect(addr).await?;
    let mut framed: Framed<TcpStream, LengthDelimitedCodec> = Framed::new(
        socket,
        LengthDelimitedCodec::new()
    );

    // Send Ping
    let msg: RpcMessage = RpcMessage::Ping;
    let bytes: Vec<u8> = bincode::serialize(&msg)?;
    framed.send(bytes.into()).await?;

    // Wait for Pong
    if let Some(Ok(bytes)) = framed.next().await {
        let resp: RpcMessage = bincode::deserialize(&bytes)?;
        println!("Server responded: {:?}", resp);
    }

    Ok(())
}
