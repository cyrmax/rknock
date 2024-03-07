use clap::arg;
use clap::Parser;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

/// Knocks specified ports. For now only TCP is supported
#[derive(Parser)]
struct CliArgsIP {
    /// IP-address of a destination server
    address: IpAddr,
    /// An array of pots to be knocked. At least one port is required
    #[arg(required = true)]
    ports: Vec<u16>,
    /// Delay between knocks in milliseconds. By default is equal to 200
    #[arg(short, long, default_value_t = 200)]
    delay: u64,
}

fn main() {
    let args = CliArgsIP::parse();
    knock(args.address, args.ports, args.delay);
}

fn knock(address: IpAddr, ports: Vec<u16>, delay: u64) {
    let dur = Duration::from_millis(delay);
    let timeout = Duration::from_millis(5);
    println!("Knocking address {}", address);
    for port in ports {
        println!("Knocking port {}", port);
        let stream = TcpStream::connect_timeout(&SocketAddr::new(address, port), timeout);
        sleep(dur);
    }
}
