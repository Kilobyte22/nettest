// Commands that are passed over the channel between the client and the server
pub const SEND_PAYLOAD:u8 = 0;
pub const REQUEST_PAYLOAD:u8 = 1;
pub const END_TEST:u8 = 2;
pub const PING_TEST:u8 = 3;
pub const DISCONNECT:u8 = 255;