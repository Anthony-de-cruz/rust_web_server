use std::net::SocketAddr;

const DEFAULT_THREAD_COUNT: usize = 5;

pub struct Config {
    pub socket_addr: SocketAddr,
    pub thread_count: usize,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            // Default thread count
            3 => {
                let socket_addr = match Config::build_socket(args[1].clone(), args[2].clone()) {
                    Ok(n) => n,
                    Err(e) => return Err("Invalid ip or port"),
                };
                return Ok(Config {
                    socket_addr,
                    thread_count: DEFAULT_THREAD_COUNT,
                });
            }
            // Custom thread count
            4 => {
                let socket_addr = match Config::build_socket(args[1].clone(), args[2].clone()) {
                    Ok(n) => n,
                    Err(e) => return Err("Invalid ip or port"),
                };

                let thread_count = match args[3].parse::<usize>() {
                    Ok(n) => n,
                    Err(e) => return Err("Invalid thread count"),
                };
                return Ok(Config {
                    socket_addr,
                    thread_count,
                });
            }
            _ => Err("Invalid number of arguments"),
        }
    }

    fn build_socket(ip_addr: String, port: String) -> Result<SocketAddr, &'static str> {
        unimplemented!();
    }
}
