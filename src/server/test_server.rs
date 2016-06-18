use std::net::{TcpListener, TcpStream};
use std::thread;
use server::connection::Connection;

pub struct TestServer {
    listener: TcpListener,
    listen_address: String,
    listen_port: u16
}

impl TestServer {
    pub fn new(port: u16, address: &str) -> TestServer {
        TestServer {
            listener: TcpListener::bind((address as &str, port)).unwrap(),
            listen_address: address.to_string(),
            listen_port: port
        }
    }

    pub fn listen(self) {
        info!("Listening on host: {} port {}", self.listen_address, self.listen_port);

        for stream in self.listener.incoming() {
            self.new_connection(stream.unwrap());
        }
    }

    fn new_connection(&self, stream: TcpStream) {
        match stream.peer_addr() {
            Ok(addr) => {
                info!("Incoming connection from {}", addr);
                let addr_ = addr.clone();
                thread::spawn(move || {
                    let mut con = Connection::new(stream);
                    match con.handle() {
                        Ok(_) => info!("Connection from {} closed", addr_),
                        Err(x) => error!("Error while reading from connection from {}: {}", addr_, x)
                    };
                });
            },
            Err(x) => {
                error!("Could not retrieve peer address: {}", x)
            }
        }
    }
}