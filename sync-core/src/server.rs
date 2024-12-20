use crate::service::Service;
use log::info;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[deny(clippy::unused_io_amount)]
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
        let router = self.service.get_routes();
        // method str to Http Method
        let method = match method {
            "GET" => http::Method::GET,
            "POST" => http::Method::POST,
            "PUT" => http::Method::PUT,
            "DELETE" => http::Method::DELETE,
            _ => http::Method::GET,
        };
        let handler = router.route(path, &method);
        let handler_response = handler.handler.call();
        //info
        info!("Response: {:?}", handler_response);
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{:?}", handler_response);
        tcp_stream.write_all(response.as_bytes()).unwrap();
    }
}
