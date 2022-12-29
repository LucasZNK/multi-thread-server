# Web Server
This is a simple web server that can handle HTTP requests and serve static files. It is designed to illustrate the concepts of thread pools, and how they can be used to improve the performance of a web server.

# How it works
The web server listens for incoming connections on a specified IP address and port. When a connection is received, it is passed to a worker thread in the thread pool for processing. The worker reads the incoming request, determines the appropriate response based on the request, and sends the response back to the client.

# Usage
To use this web server, you can either clone this repository and run the server locally, or you can use the pre-built binary in the bin directory.

To run the server locally, you will need to have Rust installed on your machine. Then, you can clone this repository and run the following command in the root directory:

``cargo run``

By default, the server will listen for incoming connections on localhost:7878. You can modify this by changing the bind address in the main function in src/main.rs.

# Customization
There are several aspects of the web server that can be customized. For example, you can change the number of worker threads in the thread pool by modifying the size parameter passed to the ThreadPool::new function in the main.rs file. You can also modify the behavior of the server by modifying the response_builder function in the main.rs file. This function is responsible for determining the appropriate response based on the incoming request.

# Future improvements
There are several ways in which this web server could be improved. Some potential improvements include:

Adding support for more HTTP methods (e.g. POST, PUT, DELETE)
Adding support for dynamic content (e.g. through the use of templates or a server-side scripting language)
Improving error handling and logging
Adding support for TLS/SSL to secure connections
