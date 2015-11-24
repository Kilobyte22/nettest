#![feature(read_exact)]

extern crate time;
extern crate byteorder;
extern crate getopts;

mod server;
mod client;

use std::thread;
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
        launch_server();
    }

    if matches.opt_present("c") {
        has_done_stuff = true;
        run_client(&matches.opt_str("c").unwrap());
    }

    if !has_done_stuff {
        print_usage(&program, opts);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run_client(host: &str) {
    let mut c = client::TestClient::new(host, 5001);

    print!("Testing ping... ");
    stdout().flush().unwrap();
    println!("done, {:.*} ms", 2, c.test_ping(20).unwrap());

    print!("Testing download... ");
    stdout().flush().unwrap();
    println!("done, {:.*} mbit/s", 2, c.test_downstream(10_000).unwrap());

    print!("Testing upload... ");
    stdout().flush().unwrap();
    println!("done, {:.*} mbit/s", 2, c.test_upstream(10_000).unwrap());
}

fn launch_server() {
    println!("Listening...");
    let s = server::TestServer::new(5001, "0.0.0.0");
    s.listen();
}
