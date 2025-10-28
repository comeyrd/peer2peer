pub const LOCAL_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 9999;

// Define the size type used throughout the protocol
pub type MessageSize = u32;
pub const MESSAGE_SIZE_BYTES: usize = std::mem::size_of::<MessageSize>();