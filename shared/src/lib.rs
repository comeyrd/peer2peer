use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub enum RpcMessage {
    Ping,
    Pong,
}
