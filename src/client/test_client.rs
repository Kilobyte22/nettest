use std::net::TcpStream;
use std::io::{Result};
use time;
use client::connection::Connection;

const BUFFER_SIZE: usize = 1024 * 1024;

pub struct TestClient {
    con: Connection
}

// The intention is that the client is silent, and any output is done by who invokes it
impl TestClient {
    pub fn new(server: &str, port: u16) -> Result<TestClient> {
        Ok(TestClient {
            con: Connection::new(try!(TcpStream::connect((server, port))))
        })
    }

    pub fn test_ping(&mut self, times: u64) -> Result<f64> {
        let mut rtt = 0f64;
        for _ in 0..times {
            let start = time::precise_time_ns();
            try!(self.con.ping());
            let end = time::precise_time_ns();
            rtt += ((end - start) as f64) / 1_000_000f64;
        }
        Ok(rtt / (times as f64))
    }

    // Time is in milliseconds
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

    // Time is in milliseconds
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