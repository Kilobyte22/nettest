# nettest - a simple speedtest

## Compiling
Install rust nightly, then run
```bash
cargo build --release
```

## Installing
```bash
sudo cp target/release/nettest /usr/local/bin
```

## Usage
First, run the server. The server needs to allow incoming connections on port 5001  
```bash
nettest -s
```
Then connect to the server using  
```bash
nettest -c <ip or hostname of server>
```

## Backstory
Once upon a time there was a poor student (me) in the library. Just for the fun of it he tried doing a speedtest, however the speedtest servers were not able to cope with the high speed. So he resorted to using iperf for upstream and hack consisiting of netcat, pv, dd and unix-shell-fu for downstream (iperf does not allow downstream testing behind a firewall that blocks incoming connections). This tool has been made to allow both downstream and upstream testing through a firewall
