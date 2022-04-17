use std::str::FromStr;


pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(verb: &str) -> Result<Self, Self::Err> {
        match verb {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(String::from("error")),
        }
    }
}

pub struct MethodError {

}
