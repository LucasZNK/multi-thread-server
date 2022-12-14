use std::io::prelude::BufRead;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::{fs, thread};

fn main() {
    // Bind to the local IP address and port 7878
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => {
            println!("Connection established");
            listener
        }
        Err(e) => {
            println!("Failed to bind to address with this error {}", e);
            return;
        }
    };

    // Accept incoming connections
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("Failed to bind to address: {}", e);
                return;
            }
        };
        connection_manager(stream);
    }

    println!("Hello, world!");
}

fn connection_manager(mut stream: TcpStream) {
    // TODO: Modify to allow dinamyc sizes
    // TODO: Manage the unrwap calls to handle the errors;
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";

    match stream.read(&mut buffer) {
        Ok(_) => println!("Request Incoming"),
        Err(e) => {
            println!("Error:{}", e)
        }
    }

    let response = response_builder(&buffer, get);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn response_builder(buffer: &[u8], get: &[u8]) -> String {
    if buffer.starts_with(get) {
        match fs::read_to_string("index.html") {
            Ok(file) => {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    file.len(),
                    file
                )
            }
            Err(e) => {
                println!("Error loading file, {}", e);
                "".to_string()
            }
        }
    } else {
        match fs::read_to_string("404.html") {
            Ok(file) => {
                format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                    file.len(),
                    file
                )
            }
            Err(e) => {
                println!("Error loading file, {}", e);
                "".to_string()
            }
        }
    }
}
