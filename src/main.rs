#![feature(read_exact)]

extern crate time;
extern crate byteorder;
extern crate getopts;

mod server;
mod client;

use std::io::{stdout, Write};
use getopts::Options;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "Shows this text");
    opts.optflag("s", "server", "Launches a server");
    opts.optopt("c", "client", "connects to a server", "SERVER_IP");
    opts.optopt("t", "time", "time to test for in seconds (default: 10)", "TIME");
    opts.optopt("p", "port", "the port to listen on and connect to (default: 5001)", "PORT");
    opts.optopt("b", "bind", "Server bind address (default: \"0.0.0.0\")", "ADDR");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut has_done_stuff = false;

    if matches.opt_present("s") {
        has_done_stuff = true;
        let port = matches.opt_str("p").and_then(|p| p.parse::<u16>().ok()).unwrap_or(5001);
        let bind = matches.opt_str("b").unwrap_or("0.0.0.0".to_string());
        launch_server(port, &bind);
    }

    if matches.opt_present("c") {
        has_done_stuff = true;
        let host = &matches.opt_str("c").unwrap();
        let port = matches.opt_str("p").and_then(|p| p.parse::<u16>().ok()).unwrap_or(5001);
        let time = matches.opt_str("t").and_then(|p| p.parse::<u64>().ok()).unwrap_or(10u64);
        match run_client(host, port, time) {
            Ok(_) => {}
            Err(x) => println!("Error during test: {}", x)
        };
    }

    if !has_done_stuff {
        print_usage(&program, opts);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run_client(host: &str, port: u16, time: u64) -> Result<(), ::std::io::Error> {
    let mut c = try!(client::TestClient::new(host, port));

    print!("Testing ping... ");
    try!(stdout().flush());
    println!("done, {:.*} ms", 2, try!(c.test_ping(20)));

    print!("Testing download... ");
    try!(stdout().flush());
    println!("done, {:.*} mbit/s", 2, try!(c.test_downstream(time * 1_000u64)));

    print!("Testing upload... ");
    try!(stdout().flush());
    println!("done, {:.*} mbit/s", 2, try!(c.test_upstream(time * 1_000u64)));

    Ok(())
}

fn launch_server(port: u16, listen: &str) {
    println!("Listening...");
    let s = server::TestServer::new(port, listen);
    s.listen();
}
