use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};
use std::net::TcpStream;
use time;

const BUFFER_SIZE: usize = 1024 * 1024;

pub struct TestClient {
    //server: String,
    //port: u16,
    con: Connection,
}

impl TestClient {
    pub fn new(server: &str, port: u16) -> Result<TestClient> {
        Ok(TestClient {
            //server: server.to_string(),
            //port: port,
            con: Connection::new(try!(TcpStream::connect((server, port)))),
        })
    }

    pub fn test_upstream(&mut self, time: u64) -> Result<f64> {
        let start = time::precise_time_ns();
        let mut bytes = 0u64;
        while (time::precise_time_ns() - start) / 1_000_000 < time {
            try!(self.con.send_upstream());
            bytes += (BUFFER_SIZE as u64) + 1u64;
        }
        let end = time::precise_time_ns();
        let bits = bytes * 8;
        let duration = ((end - start) as f64) / 1_000_000_000f64;
        let bps = bits as f64 / duration;
        Ok(bps)
    }

    pub fn test_ping(&mut self, times: u32) -> Result<f64> {
        let mut rtt = 0f64;
        for _ in 0..times {
            let start = time::precise_time_ns();
            try!(self.con.ping());
            let end = time::precise_time_ns();
            rtt += ((end - start) as f64) / 1_000_000f64;
        }
        Ok(rtt / (times as f64))
    }

    pub fn test_downstream(&mut self, time: u64) -> Result<f64> {
        try!(self.con.request_downstream(time));
        let start = time::precise_time_ns();
        let bytes = try!(self.con.process_request());
        let end = time::precise_time_ns();
        let bits = bytes * 8;
        let duration = ((end - start) as f64) / 1_000_000_000f64;
        let bps = bits as f64 / duration;
        Ok(bps)
    }
}

struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection { stream: stream }
    }

    fn request_downstream(&mut self, time: u64) -> Result<()> {
        try!(self.stream.write_u8(1u8));
        try!(self.stream.write_u64::<BigEndian>(time));
        Ok(())
    }

    fn ping(&mut self) -> Result<()> {
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

    fn send_upstream(&mut self) -> Result<()> {
        let buf = [0; BUFFER_SIZE];
        try!(self.stream.write_u8(0u8));
        try!(self.stream.write(&buf));
        try!(self.stream.flush());
        Ok(())
    }

    fn process_request(&mut self) -> Result<u64> {
        let mut buf = [0u8; BUFFER_SIZE];
        let mut bytes = 0u64;
        loop {
            let ptype = try!(self.stream.read_u8());
            match ptype {
                0 => {
                    try!(self.stream.read_exact(&mut buf));
                    bytes += (BUFFER_SIZE as u64) + 1;
                }
                1 => {
                    // wat.
                }
                2 => {
                    return Ok(bytes + 1u64);
                }
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
