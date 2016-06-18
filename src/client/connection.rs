use std::net::TcpStream;
use std::io::{Result, Write, Read};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

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
        try!(self.stream.write_u8(1u8));
        try!(self.stream.write_u64::<BigEndian>(time));
        Ok(())
    }

    pub fn ping(&mut self) -> Result<()> {
        try!(self.stream.write_u8(3u8));
        try!(self.stream.flush());
        let ptype = try!(self.stream.read_u8());
        loop {
            match ptype {
                3 => return Ok(()),
                _ => {
                    println!("Received unknown packet {}", ptype);
                }
            };
        }
    }

    pub fn send_upstream(&mut self) -> Result<()> {
        let buf = [0; BUFFER_SIZE];
        try!(self.stream.write_u8(0u8));
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
                0 => {
                    try!(self.stream.read_exact(&mut buf));
                    bytes += (BUFFER_SIZE as u64) + 1;
                },
                1 => {
                    // wat.
                },
                2 => {
                    return Ok(bytes + 1u64);
                },
                _ => {}
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
