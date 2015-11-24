#![feature(read_exact)]

extern crate time;
extern crate byteorder;

mod server;
mod client;

use std::thread;

fn main() {
    thread::spawn(|| {
        let s = server::TestServer::new(5001, "0.0.0.0");
        s.listen();
    });
    let c = client::TestClient::new("127.0.0.1", 5001);
    println!("Speed: {}", c.test_downstream(5000).unwrap());
}
