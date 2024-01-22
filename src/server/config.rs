use std::{net::SocketAddr, str::FromStr};

pub const DEFAULT_THREAD_COUNT: usize = 5;

pub struct Config {
    pub socket_addr: SocketAddr,
    pub thread_count: usize,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
        match args.len() {
            // Default thread count
            3 => {
                let socket_addr =
                    match SocketAddr::from_str(format!("{}:{}", args[1], args[2]).as_str()) {
                        Ok(n) => n,
                        Err(err) => return Err(err)?,
                    };
                return Ok(Config {
                    socket_addr,
                    thread_count: DEFAULT_THREAD_COUNT,
                });
            }
            // Custom thread count
            4 => {
                let socket_addr =
                    match SocketAddr::from_str(format!("{}:{}", args[1], args[2]).as_str()) {
                        Ok(n) => n,
                        Err(_) => return Err("Invalid ip or port")?,
                    };

                let thread_count = match args[3].parse::<usize>() {
                    Ok(n) => n,
                    Err(err) => return Err(err)?,
                };
                return Ok(Config {
                    socket_addr,
                    thread_count,
                });
            }
            _ => Err("Invalid number of arguments")?,
        }
    }
}
