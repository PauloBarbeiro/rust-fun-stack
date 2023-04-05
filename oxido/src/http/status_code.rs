pub mod status_code {
    use std::fmt::{
        Display,
        Formatter,
        Result as FmtResult
    };

    /// StatusCode:
    /// Defines the behavior and functionality needed for the use of http status code
    /// in the HTTP server.
    #[derive(Copy, Clone, Debug)]
    pub enum StatusCode {
        Ok = 200,
        BadRequest = 400,
        NotFound = 404,
    }

    impl StatusCode {
        pub fn reason_phrase(&self) -> &str {
            match self {
                Self::Ok => "Ok",
                Self::BadRequest => "Bad Request",
                Self::NotFound => "Not Found",
            }
        }
    }

    impl Display for StatusCode {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            write!(f, "{}", *self as u16)
        }
    }
}

#[cfg(test)]
mod describe_reason_phrases {
    use super::status_code::StatusCode;

    #[test]
    fn should_return_ok() {
        assert_eq!(StatusCode::Ok.reason_phrase(), "Ok");
    }

    #[test]
    fn should_return_bad_request() {
        let code = StatusCode::BadRequest;
        assert_eq!(code.reason_phrase(), "Bad Request");
    }

    #[test]
    fn should_return_not_found() {
        let code = StatusCode::NotFound;
        assert_eq!(code.reason_phrase(), "Not Found");
    }
}


#[cfg(test)]
mod describe_status_code_display {
    use super::status_code::StatusCode;

    #[test]
    fn should_render_200() {
        assert_eq!(StatusCode::Ok.to_string(), "200");
    }

    #[test]
    fn should_render_400() {
        assert_eq!(StatusCode::BadRequest.to_string(), "400");
    }

    #[test]
    fn should_render_404() {
        assert_eq!(StatusCode::NotFound.to_string(), "404");
    }
}