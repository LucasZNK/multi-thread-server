use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::{fs, thread};

use web_server::ThreadPool;

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

    let thread_pool = ThreadPool::new(4);

    // Accept incoming connections
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("Failed to bind to address: {}", e);
                return;
            }
        };
        thread_pool.execute(|| {
            connection_manager(stream);
        });
    }
}

fn connection_manager(mut stream: TcpStream) {
    // TODO: Modify to allow dynamic sizes
    // TODO: Manage the unwrap calls to handle the errors;
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";

    // Read incoming request
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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Determine status line and file name based on request
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Read file and build response string
    match fs::read_to_string(file_name) {
        Ok(content) => {
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                content.len(),
                content
            )
        }
        Err(e) => {
            println!("Error loading file, {}", e);
            "".to_string()
        }
    }
}
