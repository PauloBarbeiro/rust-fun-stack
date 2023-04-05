use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{ Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

use super::method::method::MethodError;
use super::{ Request, Method };
use super::query_string::query_string::QueryString;

pub mod request {
    use crate::http::method;
    use super::QueryString;
    
    #[derive(Debug)]
    pub struct Request<'buffer> {
        pub path: &'buffer str,
        pub query: Option<QueryString<'buffer>>,
        pub method: method::method::Method
    }

    impl<'buffer> Request<'buffer> {
        pub fn path(&self) -> &str {
            &self.path
        }
        
        pub fn method(&self) -> &method::method::Method {
            &self.method
        }

        pub fn query(&self) -> Option<&QueryString> {
            self.query.as_ref()
        }
    }
}

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    fn try_from(buf: &'buffer [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        
        let mut query_string = None;

        // Option 3: uses Rust optimization ofr the patterns above, less code and no extra boilerplate;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
print!("{:?}", query_string);
        Ok(Self {
            path,
            query: query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod describe_request {
    use super::request::Request;
    use super::super::method::method::Method;
    use super::super::query_string::query_string::QueryString;
/*
GET / HTTP/1.1
GET /test HTTP/1.1
GET /test?a=1&b=2&c&d=&e===&d=7&d=abc HTTP/1.1

GET /style.css HTTP/1.1
*/
    #[test]
    fn should_build_a_request_to_root(){
        let buffer = b"GET / HTTP/1.1\r";
        let request = Request::try_from(&buffer[..]).unwrap();

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path(), "/");
        assert_eq!(request.query, None);
    }

    #[test]
    fn should_build_a_request_to_special_path(){
        let buffer = b"GET /test HTTP/1.1\r";
        let request = Request::try_from(&buffer[..]).unwrap();

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path(), "/test");
        assert_eq!(request.query, None);
    }

    #[test]
    fn should_build_a_request_with_query_string(){
        let buffer = b"GET /test?a=1&b=2&c&d=&e===&d=7&d=abc HTTP/1.1\r";
        let request = Request::try_from(&buffer[..]).unwrap();

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path(), "/test");
        // @todo: how to assert the query_string
        // assert_eq!(request.query.expect("a"), "1");
    }
}

#[cfg(test)]
mod describe_parse_error {

}