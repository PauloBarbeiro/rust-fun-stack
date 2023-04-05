use std::{
    net::TcpListener, 
    io::{ Read }, 
    convert::TryFrom,
};

use crate::http::{ 
    Request, 
    Response, 
    StatusCode, 
    ParseError,
};

// Custom Trait
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        print!("Failed to parse the request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}
    
impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    
    pub fn run(self, mut handler: impl Handler) {
        print!("Listening on {}\n", self.addr);

        // Unwrap will terminate the app if the result is an error
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // Creates an array of zeros, with length 1024. It MUST assure all memory is initialized in the buffer.
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Request received: {}", String::from_utf8_lossy(&buffer));
                            
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                print!("Failed to set response: {}", e);
                            }
                        }
                        Err(e) => print!("Fail to read stream: {}", e)
                    }

                }
                Err(e) => {
                    print!("Connection failed: {}", e);
                    continue
                }
            }
        }
    }
}
