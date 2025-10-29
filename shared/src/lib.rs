use serde::{ Serialize, Deserialize };

type TopicId = u8;
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Register(TopicId),
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerResponse {
    PeerInfo(PeerInfoData),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PingPongMessage {
    Ping,
    Pong,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RpcMessage {
    Server(ServerMessage),
    Response(ServerResponse),
    PingPong(PingPongMessage),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PeerInfoData {
    pub addr: String,
    pub serve: bool,
}
