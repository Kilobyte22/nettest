use std::net::{TcpStream};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::io::{Result, Write, Read};
use time;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use commands::PING_TEST;
use commands::REQUEST_PAYLOAD;
use commands::SEND_PAYLOAD;
use commands::END_TEST;
use commands::DISCONNECT;

const BUFFER_SIZE: usize = 1024 * 1024;

pub struct Connection {
    stream: TcpStream,
    sender_commander: Sender<u64>
}

impl Connection {

    pub fn new(stream: TcpStream) -> Connection {
        let (tx, rx) = channel::<u64>();
        let s = stream.try_clone().unwrap();
        thread::spawn(|| {
            let peer_addr =  s.peer_addr().unwrap().clone();
            match Connection::sender_runner(rx, s) {
                Ok(_) => {}
                Err(x) => println!("Error while writing to connection from {}: {}", peer_addr, x)
            };
        });
        Connection {
            stream: stream,
            sender_commander: tx
        }
    }

    pub fn handle(&mut self) -> Result<()> {
        loop {
            let command = try!(self.stream.read_u8());
            match command {
                SEND_PAYLOAD => {
                    println!("Received command SEND_PLAYLOAD");
                    let mut sink = [0; BUFFER_SIZE];
                    try!(self.stream.read_exact(&mut sink));
                },
                REQUEST_PAYLOAD => {
                    println!("Received command REQUEST_PLAYLOAD");
                    let ms = try!(self.stream.read_u64::<BigEndian>());
                    self.sender_commander.send(ms).unwrap();
                },
                PING_TEST => {
                    println!("Received command PING_TEST");
                    try!(self.stream.write_u8(PING_TEST));
                },
                DISCONNECT => {
                    println!("Received command DISCONNECT");
                    return Ok(());
                },
                _ => {
                    println!("Unexpected command {}", command)
                }
            };
        }
    }

    fn sender_runner(rx: Receiver<u64>, mut stream: TcpStream) -> Result<()> {
        let buf = [0u8; BUFFER_SIZE];

        loop {
            match rx.recv() {
                Ok(time) => {
                    let start = time::precise_time_ns();
                    loop {
                        try!(stream.write_u8(SEND_PAYLOAD));
                        try!(stream.write(&buf));
                        try!(stream.flush());

                        if (time::precise_time_ns() - start) / 1_000_000 >= time {
                            break;
                        }
                    }
                    try!(stream.write_u8(END_TEST))
                },
                Err(_) => return Ok(())
            }
        }
    }
}
