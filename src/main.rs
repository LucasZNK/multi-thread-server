use std::io::prelude::BufRead;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

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
    let mut buffer = Vec::new();

    thread::spawn(move || match stream.read_to_end(&mut buffer) {
        Ok(result) => {
            println!("Read {} bytes from stream", result);
            println!("Request:{}", String::from_utf8_lossy(&buffer[..]));
        }
        Err(e) => {
            println!("Error reading from stream: {}", e);
        }
    });

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";

    // Send the response back to the client
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Error writing to stream: {}", e),
    }
}
