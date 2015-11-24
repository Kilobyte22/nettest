use std::net::TcpStream;
use std::io::{Result, Write, Read};
use time;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

const BUFFER_SIZE: usize = 1024;

pub struct TestClient {
    server: String,
    port: u16,
    con: Connection
}

impl TestClient {
    pub fn new(server: &str, port: u16) -> TestClient {
        let mut client = TestClient {
            server: server.to_string(),
            port: port,
            con: Connection::new(TcpStream::connect((server, port)).unwrap())
        };
    }

    pub fn test_upstream() {

    }

    pub fn test_downstream(&self, time: u64) -> Result<f64> {
        self.con.request_downstream(time);
        let start = time::precise_time_ns();
        let bytes = try!(self.con.process_request());
        let end = time::precise_time_ns();
        let bits = bytes * 8;
        let duration = ((end - start) as f64) / 1_000_000f64;
        let bps = bits as f64 / duration;
        let mbit = bps / (1024f64 * 1024f64);
        Ok(mbit)
    }
}

struct Connection {
    stream: TcpStream
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: stream
        }
    }

    fn request_downstream(&self, time: u64) -> Result<()> {
        try!(self.stream.write_u8(1u8));
        try!(self.stream.write_u64(time));
        Ok(())
    }

    fn process_request(&self) -> Result<u64> {
        loop {
            let ptype = try!(self.stream.read_u8());
            let mut buf = [0u8; BUFFER_SIZE];
            let mut bytes = 0u64;
            match ptype {
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
}
