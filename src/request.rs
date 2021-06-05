// request.rs
// contains struct definitions and implementations for handling HTTP headers / requests

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,

    // this is from C implementation. In this implementation, use
    // std::collections::LinkedList

    // next: Header,
}

impl Header {
    fn new(name: String, value: String) -> Header {
        Header {
            name: name,
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub tcp_stream: Option<std::net::TcpStream>,
    pub host: Option<std::net::SocketAddr>,
    pub method: String,
    pub uri: String,
    pub headers: LinkedList<Header>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            tcp_stream: None,
            host: None,
            method: "".to_string(),
            uri: "".to_string(),
            headers: LinkedList::new(),
        }
    }
    // TODO: return request or not
    pub fn accept_request(&mut self, tcp_listener: &std::net::TcpListener) -> Result<(), Box<dyn Error>> {
        match tcp_listener.accept() {
            Ok((_socket, addr)) => {
                log::info!("Accepted request addr: {:?}", addr);
                self.tcp_stream = Some(_socket);
                self.host = Some(addr);
                Ok(())
            },
            Err(e) => {
                log::error!("Unable to accept connection: {}", e);
                Err(Box::new(e)) // todo - am I propogating errors correctly / in the "rust" way ?
            }
        }

    }

    // Parse a request for method, uri, and headers
    pub fn parse_request(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 4096];
        self.tcp_stream.as_ref().unwrap().read(&mut buffer).unwrap(); // how to read stream

        let mut tmp = String::from_utf8_lossy(&buffer[..]);
        log::debug!("tmp: {:?}", tmp);

        // 1. split by \r\n
        let mut nl = tmp.split("\r\n");
        // 2. Get first line containing method and uri
        let nl2 = nl.next().unwrap();
        // 3. split by whitespace, etract uri, method
        let mut split = nl2.split_whitespace();
        let method = split.next();
        let uri = split.next();
        log::debug!("method: {:?}", method);
        log::debug!("uri: {:?}", uri);
        self.method = method.unwrap().to_string();
        self.uri = uri.unwrap().to_string();

        for line in nl {
            let mut headers = line.replace(" ", "");
            let mut htest = headers.split(":").collect::<Vec<&str>>();

            if htest[0].len() == 0 {
                break
            }

            let header = Header::new(htest[0].to_string(), htest[1].to_string());
            self.headers.push_back(header);
        }

        log::debug!("header ll: {:?}", self.headers);

        Ok(())
    }

}
