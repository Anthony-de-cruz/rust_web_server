use std::{net::SocketAddr, str::FromStr};

use clap::Parser;

pub const DEFAULT_THREAD_COUNT: usize = 4;

pub struct Config {
    pub socket_addr: SocketAddr,
    pub thread_count: usize,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // The server socket
    #[arg(short, long, default_value_t = String::from("127.0.0.1:7878"))]
    socket: String,
    // Thread pool size (fixed)
    #[arg(short, long, default_value_t = DEFAULT_THREAD_COUNT)]
    threads: usize,
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn std::error::Error>> {
        let args = Args::parse();
        if args.threads < 1 {
            Err(format!(
                "Too few threads: {}, must be at least 1",
                args.threads
            ))?
        }
        match SocketAddr::from_str(&args.socket) {
            Ok(n) => Ok(Config {
                socket_addr: n,
                thread_count: args.threads,
            }),
            Err(_) => Err(format!(
                "Invalid socket: {}, must be valid IPv4 or IPv6",
                &args.socket
            ))?,
        }
    }
}
