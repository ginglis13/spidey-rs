// request.rs
// contains struct definitions and implementations for handling HTTP headers / requests

use std::fs;
use std::fs::File;
use std::error::Error;
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::Read;
use std::collections::LinkedList;

#[derive(Copy, Clone, PartialEq)]
enum HttpStatus {
    OK,
    BAD_REQUEST,
    NOT_FOUND,
    INTERNAL_SERVER_ERROR,
}

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
    pub path: PathBuf,
    pub headers: LinkedList<Header>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            tcp_stream: None,
            host: None,
            method: "".to_string(),
            uri: "".to_string(),
            path: PathBuf::new(),
            headers: LinkedList::new(),
        }
    }

    pub fn handle_request(&mut self, root_path: String) {
        self.parse_request(); /* TODO: if error, return HTTP_STATUS_BAD_REQUEST */

        match self.determine_request_path(root_path) {
            Ok(_) => {
                log::info!("Request to path {:?}", self.path);
                // Determine filetype. PathBuf type kinda does it for us!
                if self.path.is_dir() {
                    self.handle_browse_request();
                } else if self.path.is_file() {
                    self.handle_file_request();
                } else {
                    /* HTTP_STATUS_BAD_REQUEST */
                    self.handle_error(HttpStatus::BAD_REQUEST);
                    return;
                }
            },
            Err(e) => {
                log::error!("Bad path given: {}", e); 
                self.handle_error(HttpStatus::NOT_FOUND);
                return;
            },
        }

        
    }

    fn determine_request_path(&mut self, root_path: String) -> Result<(), Box<dyn Error>> {
        let p = PathBuf::from(format!("{}/{}", root_path, self.uri));
        self.path = fs::canonicalize(&p)?;

        Ok(())
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
    fn parse_request(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 4096];
        self.tcp_stream.as_ref().unwrap().read(&mut buffer).unwrap(); // how to read stream

        let tmp = String::from_utf8_lossy(&buffer[..]);

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
            let headers = line.replace(" ", "");
            let htest = headers.split(":").collect::<Vec<&str>>();

            if htest[0].len() == 0 {
                break
            }

            let header = Header::new(htest[0].to_string(), htest[1].to_string());
            self.headers.push_back(header);
        }

        log::debug!("header ll: {:?}", self.headers);

        Ok(())
    }
    
    fn handle_browse_request(&self) -> Result<(), Box<dyn Error>> {
        let dir = fs::read_dir(self.path.to_str().unwrap()).unwrap(); // Returns option -> make fn signature option??

        let mut stream = self.tcp_stream.as_ref().unwrap();

        // Write HTTP Header with OK Status and text/html Content-Type
        stream.write(&"HTTP/1.0 200 OK\r\n".as_bytes())?;
        stream.write(&"Content-Type: text/html\r\n".as_bytes())?;
        stream.write(&"\r\n".as_bytes())?;

        // Head tag
        stream.write(&"<head>\n".as_bytes());
        stream.write(&"<link href=\"css/bootstrap.min.css\" rel=\"stylesheet\">".as_bytes())?;
        stream.write(&"<link href=\"css/custom.css\" rel=\"stylesheet\">".as_bytes())?;
        stream.write(&"</head>\n".as_bytes())?;

        // Website heading
        stream.write(&"<body>\n".as_bytes())?;
        stream.write(&"<div class=\"jumbotron\">\n".as_bytes())?;
        stream.write(&"<div class=\"container\">\n".as_bytes())?;
        stream.write(&"<h1 style=\"text-align: center; color: white\">Gourd Experts</h1>".as_bytes())?;
        stream.write(&"<h4 style=\"text-align: center; color: white\">Burger, Gallahue, Inglis</h4>".as_bytes())?;
        stream.write(&"</div>\n".as_bytes())?;
        stream.write(&"</div>\n".as_bytes())?;

        // For each entry in directory, emit HTML list item
        stream.write(&"<div class=\"list-group container\">\n".as_bytes())?;

        for entry in dir {
            let entry = entry.unwrap();
            let entry_str = entry.path();
            let entry_name = entry.file_name();

            log::info!("Path: {}", entry_str.display());
            log::info!("Name: {:?}", entry_name);

            // Create the href
            stream.write(&"<a href=".as_bytes())?;

            // Write uri to href
            stream.write(self.uri.as_bytes())?;

            // Check root dir and relative dirs
            if self.uri != String::from("/") {
                stream.write(&"/".as_bytes())?;
            }

            // Add path to href, close href, add label
            stream.write(format!("{} ", entry_name.to_str().unwrap()).as_bytes())?;
            stream.write(&"class=\"list-group-item list-group-item-action\">".as_bytes())?;
            stream.write(format!("{}", entry_name.to_str().unwrap()).as_bytes())?;
            stream.write(&"</a>".as_bytes())?;
        }

        stream.write(&"</div>\n".as_bytes())?;
        stream.write(&"</body>\n".as_bytes())?;

        Ok(())
    }

    fn handle_file_request(&mut self) -> Result<(), Box<dyn Error>> {

        let mut stream = self.tcp_stream.as_ref().unwrap();

        /* Determine mimetype */
        //let mimetype = determine_mimetype(self.path);
        // TODO: Actually determine mimetype - for now, text/plain
        let mimetype = String::from("text/plain");

        /* Write HTTP Headers with OK status and determined Content-Type */
        stream.write(&"HTTP/1.0 200 OK\n".as_bytes());
        stream.write(&"Content-Type: ".as_bytes());
        stream.write(format!("{}\n", mimetype).as_bytes());
        stream.write(&"\r\n".as_bytes());

        let mut file = File::open(self.path.to_str().unwrap())?;
        
        let mut buffer = [0; 4096];

        loop {
            match file.read(&mut buffer) {
                Ok(nread) => {
                    if nread == 0 {
                        break; 
                    }
                    
                    // Write only the amount of bytes read! fixes binary 
                    // looking issue when extra 0 characters written to stream
                    let nwritten = stream.write(&buffer[..nread]).unwrap();

                    if nread != nwritten {
                        // TODO: this handle_error is a mutable borrow. Cannot occur due to
                        // immutable borrows referencing stream
                        // self.handle_error(HttpStatus::INTERNAL_SERVER_ERROR);
                        // e not in scope (duh) -> just return Ok() since handle_error displays
                        // 500 page? That would differ from below Err(e) case below...
                        // return Err(Box::new(x)); 
                        log::error!("UNEQUAL BYTES... read: {} written: {}", nread, nwritten);
                    }
                },
                Err(e) => { // Read error can occur 
                    self.handle_error(HttpStatus::INTERNAL_SERVER_ERROR);
                    return Err(Box::new(e));
                },

            }  
    

        }
        Ok(())

    }

    fn handle_error(&mut self, status: HttpStatus) {
        let status_string = http_status_string(status);
        
        let mut stream = self.tcp_stream.as_ref().unwrap();

        /* Write HTTP Header */
        stream.write(format!("HTTP/1.0 {}\r\n", status_string).as_bytes());
        stream.write(&"Content-Type: text/html\r\n".as_bytes());
        stream.write(&"\r\n".as_bytes());
    
        // Head tag
        stream.write(&"<head>\n".as_bytes());
        stream.write(&"<link href=\"css/bootstrap.min.css\" rel=\"stylesheet\">".as_bytes());
        stream.write(&"<link href=\"css/custom.css\" rel=\"stylesheet\">".as_bytes());
        stream.write(&"</head>\n".as_bytes());
    
    
        // Website heading
        stream.write(&"<body>\n".as_bytes());
        stream.write(&"<div class=\"jumbotron\">\n".as_bytes());
        stream.write(&"<div class=\"container\">\n".as_bytes());
        stream.write(&"<h1 style=\"text-align: center; color: white\">Gourd Experts</h1>".as_bytes());
        stream.write(&"<h4 style=\"text-align: center; color: white\">Burger, Gallahue, Inglis</h4>".as_bytes());
        stream.write(&"</div>\n".as_bytes());
        stream.write(&"</div>\n".as_bytes());
    
        /* Write HTML Description of Error*/
        log::info!("STATUS STR: {}", status_string);
        stream.write(format!("<h1>{}</h1>", status_string).as_bytes());

        // Tbt to when this sophomore yr proj was gourd themed.
        if status == HttpStatus::NOT_FOUND {
            stream.write(&"<p>The gourd you seek could not be foud. Do not give up hope, there are plenty of gourds in the sea.</p>".as_bytes());
        }
        else if status == HttpStatus::BAD_REQUEST {
           stream.write(&"<p>Bad gourd request.</p>".as_bytes());
        }
        else if status == HttpStatus::INTERNAL_SERVER_ERROR {
           stream.write(&"<p>Something went wrong. Contact gourdhelp@nd.edu for assistance.</p>".as_bytes());
        }
    
        stream.write(&"</body>".as_bytes());
    }
}

fn http_status_string(status: HttpStatus) -> String {
    match status {
        HttpStatus::OK => String::from("200 OK"),
        HttpStatus::BAD_REQUEST => String::from("400 Bad Request"),
        HttpStatus::NOT_FOUND => String::from("404 Not Found"),
        HttpStatus::INTERNAL_SERVER_ERROR => String::from("500 Internal Server Error"),    
    }
}
