use core::fmt;
use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

pub struct Config {
    server_name:  String,
    port:        String,
}

pub fn new_config(name: String, port: String) -> Config {
    let c = Config{
        server_name: name.to_string(),
        port: port.to_string(),
    };

    c
}

pub fn request_handler(mut stream: TcpStream){
        let bufer_reader = BufReader::new(&mut stream);

        let http_request: Vec<_> = bufer_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        println!("Request: {:#?}", http_request);

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        println!("Sending response: {:#?}", response);
        stream.write_all(response.as_bytes()).unwrap();
}

pub fn connect(config: Config) {
    let listener = TcpListener::bind(config.to_string()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        request_handler(stream);
    }
}

impl fmt::Display for Config {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.server_name, self.port)
    }
}

