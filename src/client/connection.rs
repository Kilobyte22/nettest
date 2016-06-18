use std::net::TcpStream;
use std::io::{Result, Write, Read};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use commands::PING_TEST;
use commands::REQUEST_PAYLOAD;
use commands::SEND_PAYLOAD;
use commands::END_TEST;

const BUFFER_SIZE: usize = 1024 * 1024;

pub struct Connection {
    stream: TcpStream
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: stream
        }
    }

    pub fn request_downstream(&mut self, time: u64) -> Result<()> {
        try!(self.stream.write_u8(REQUEST_PAYLOAD));
        try!(self.stream.write_u64::<BigEndian>(time));
        Ok(())
    }

    pub fn ping(&mut self) -> Result<()> {
        try!(self.stream.write_u8(PING_TEST));
        try!(self.stream.flush());
        let ptype = try!(self.stream.read_u8());
        loop {
            match ptype {
                PING_TEST => return Ok(()),
                _ => {
                    error!("Received unknown packet {}", ptype);
                }
            };
        }
    }

    pub fn send_upstream(&mut self) -> Result<()> {
        let buf = [0; BUFFER_SIZE];
        try!(self.stream.write_u8(SEND_PAYLOAD));
        try!(self.stream.write(&buf));
        try!(self.stream.flush());
        Ok(())
    }

    pub fn process_request(&mut self) -> Result<u64> {
        let mut buf = [0u8; BUFFER_SIZE];
        let mut bytes = 0u64;
        loop {
            let command = try!(self.stream.read_u8());
            match command {
                SEND_PAYLOAD => {
                    try!(self.stream.read_exact(&mut buf));
                    bytes += (BUFFER_SIZE as u64) + 1;
                },
                END_TEST => {
                    return Ok(bytes + 1u64);
                },
                _ => {
                    error!("Unexpected command {}", command)
                }
            }
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        try!(self.stream.write_u8(255));
        Ok(())
    }
}

impl Drop for Connection {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.shutdown();
    }
}
