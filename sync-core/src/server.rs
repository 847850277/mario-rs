use log::info;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {
        //println!("todo start tcp server...");
        let addr = "127.0.0.1:8080";
        // start tcp server
        let listener = TcpListener::bind(addr).unwrap();
        info!("Listening on http://{}", addr);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("failed: {}", e);
                }
            }
        }
    }

    fn handle_connection(&self, mut tcp_stream: TcpStream) {
        //println!("todo handle connection...");
        let mut buffer = [0; 1024];
        tcp_stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer));

        // response
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        let response = format!("{}{}", response, "Hello World");
        tcp_stream.write(response.as_bytes()).unwrap();
    }
}
