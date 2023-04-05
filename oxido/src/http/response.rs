pub mod response {
    use std::{
        io::{ Write, Result as IoResult },
        fmt::{
            Display,
            Formatter,
            Result as FmtResult,
        },
    };
    use super::super::StatusCode;

    pub struct Response {
        status_code: StatusCode,
        body: Option<String>,
    }

    impl Response {
        pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
            Response { status_code, body }
        }

        // Option Two: Uses generic type definitions of any object that implement a "write trait"
        // This makes the code more generic and easier to implement tests;
        pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
            let body = match &self.body {
                Some(b) => b,
                None => "",
            };
    
            write!(
                stream, 
                "HTTP/1.1 {} {}\r\n\r\n{}", 
                self.status_code, 
                self.status_code.reason_phrase(),
                body,
            )
        }
        
    }

    impl Display for Response {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            let body = match &self.body {
                Some(b) => b,
                None => "",
            };

            write!(
                f, 
                "HTTP/1.1 {} {}\r\n\r\n{}", 
                self.status_code, 
                self.status_code.reason_phrase(),
                body,
            )
        }
    }
}

#[cfg(test)]
mod describe_response {
    use super::response::Response;
    use super::super::StatusCode;

    #[test]
    fn should_sent_response_without_body(){
        let mut out = Vec::new();

        Response::new(
            StatusCode::NotFound,
            None
        ).send(&mut out).unwrap();

        let string_output = String::from_utf8(out).unwrap();
        assert_eq!(string_output, "HTTP/1.1 404 Not Found\r\n\r\n");
    }

    #[test]
    fn should_sent_response_with_body(){
        let mut out = Vec::new();

        Response::new(
            StatusCode::Ok,
            Some("<h1>Test body</h1>".to_string())
        ).send(&mut out).unwrap();

        let string_output = String::from_utf8(out).unwrap();
        assert_eq!(string_output, "HTTP/1.1 200 Ok\r\n\r\n<h1>Test body</h1>");
    }
}