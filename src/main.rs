pub mod server;

use server::config::Config;
use server::thread_pool::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let listener = TcpListener::bind(config.socket_addr).unwrap();
    let pool = ThreadPool::new(config.thread_count);

    for stream in listener.incoming() {
        match stream {
            Ok(k) => {
                pool.execute(|| {
                    handle_connection(k);
                });
            }
            Err(err) => {
                eprintln!("TCP stream error: {err}");
            }
        }
    }
    println!("Server shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Problem with file system: {err}");
        eprintln!("             Status line: {status_line}");
        eprintln!("               File name: {filename}");
        process::exit(1);
    });

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
