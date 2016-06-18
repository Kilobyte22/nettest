extern crate time;
extern crate byteorder;
extern crate getopts;

mod server;
mod client;
use client::test_client;
use server::test_server;
mod commands;

use std::io::{stdout, Write};
use getopts::Options;
use std::env;

static DEFAULT_HOST: &'static str = "0.0.0.0";
const DEFAULT_PORT:u16 = 5001;
const DEFAULT_TIME:u64 = 10;
const DEFAULT_PINGS:u64 = 20;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    if cfg!(debug_assertions) {
        println!("!! WARNING: You are running a not optimized version of nettest !!");
        println!("!! Please use the --release build switch for any serious tests !!");
        println!("");
    }

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    println!("{} v{}", program, VERSION.unwrap_or("unknown"));

    let mut opts = Options::new();

    opts.optflag("h", "help", "Shows this text");
    opts.optflag("s", "server", "Launches a server");
    opts.optopt("c", "client", "connects to a server", "HOST");
    opts.optopt("t", "time", &format!("time to test upload/download for in seconds (default: {})", DEFAULT_TIME), "TIME");
    opts.optopt("n", "num_pings", &format!("number of pings to perform (default: {})", DEFAULT_PINGS), "PINGS");
    opts.optopt("p", "port", &format!("the port to listen on and connect to (default: {})", DEFAULT_PORT), "PORT", );
    opts.optopt("b", "bind", &format!("Server bind address (default: {})", DEFAULT_HOST.to_string()), "ADDR");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { 
            // Unwrap is fine here because writing to stderr shouldn't fail
            writeln!(&mut std::io::stderr(), "{}", f.to_string()).unwrap();
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut has_done_stuff = false;

    if matches.opt_present("s") {
        has_done_stuff = true;
        let port = matches.opt_str("p").and_then(|p| p.parse::<u16>().ok()).unwrap_or(DEFAULT_PORT);
        let bind = matches.opt_str("b").unwrap_or(DEFAULT_HOST.to_string());
        let s = test_server::TestServer::new(port, &bind);
        s.listen();
    }

    if matches.opt_present("c") {
        has_done_stuff = true;
        let host = &matches.opt_str("c").unwrap();
        let port = matches.opt_str("p").and_then(|p| p.parse::<u16>().ok()).unwrap_or(DEFAULT_PORT);
        let time = matches.opt_str("t").and_then(|p| p.parse::<u64>().ok()).unwrap_or(DEFAULT_TIME);
        let pings = matches.opt_str("n").and_then(|n| n.parse::<u64>().ok()).unwrap_or(DEFAULT_PINGS);
        match run_client(host, port, time, pings) {
            Ok(_) => {}
            Err(x) => writeln!(&mut std::io::stderr(), "Error during test: {}", x).unwrap()
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

fn run_client(host: &str, port: u16, time: u64, pings: u64) -> Result<(), ::std::io::Error> {
    let mut c = try!(test_client::TestClient::new(host, port));

    print!("Testing ping... ({} times) ", pings);
    try!(stdout().flush());
    let ping_time = try!(c.test_ping(pings));
    println!("done, {:.*} ms", 2, ping_time);

    print!("Testing download... ({} seconds) ", time);
    try!(stdout().flush());
    let download_speed = try!(c.test_downstream(time * 1_000u64));
    println!("done, {}", format_speed(download_speed));

    print!("Testing upload... ({} seconds) ", time);
    try!(stdout().flush());
    let upload_speed = try!(c.test_upstream(time * 1_000u64));
    println!("done, {}", format_speed(upload_speed));

    Ok(())
}

fn format_speed(speed: f64) -> String {
    let mut speed = speed;
    let units = ["bit/s", "Kbit/s", "Mbit/s", "Gbit/s", "Tbit/s"];
    let mut idx = 0;
    while speed > 1024f64 && idx < 4 {
        idx += 1;
        speed /= 1024f64;
    }

    format!("{:.3} {}", speed, units[idx])

}
