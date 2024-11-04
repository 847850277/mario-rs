use crate::service::Service;
use log::info;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    pub service: Service,
}

impl Server {
    pub fn new(service: Service) -> Self {
        Self { service }
    }

    pub fn start(&self) {
        let addr = "127.0.0.1:8080";
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
        let mut buffer = [0; 1024];
        tcp_stream.read(&mut buffer).unwrap();
        info!("Request: {}", String::from_utf8_lossy(&buffer));
        // parse request
        let request = String::from_utf8_lossy(&buffer);
        let request = request.split_whitespace().collect::<Vec<&str>>();
        let method = request[0];
        let path = request[1];
        let _version = request[2];
        info!("Method: {}", method);
        info!("Path: {}", path);
        info!("Version: {}", _version);
        // find route
        let route = self
            .service
            .routes
            .iter()
            .find(|r| r.method == method && r.path == path);
        match route {
            Some(route) => {
                let response_body = (route.handler)();
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response_body);
                tcp_stream.write(response.as_bytes()).unwrap();
            }
            None => {
                let response_body = "Not Found Route";
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response_body);
                tcp_stream.write(response.as_bytes()).unwrap();
            }
        }
    }
}