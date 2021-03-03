use std::str::FromStr;

#[derive(Debug)]
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => {
                return Ok(Self::GET);
            }
            "DELETE" => {
                return Ok(Self::DELETE);
            }
            "POST" => {
                return Ok(Self::POST);
            }
            "PUT" => {
                return Ok(Self::PUT);
            }
            "HEAD" => {
                return Ok(Self::HEAD);
            }
            "CONNECT" => {
                return Ok(Self::CONNECT);
            }
            "OPTIONS" => {
                return Ok(Self::OPTIONS);
            }
            "TRACE" => {
                return Ok(Self::TRACE);
            }
            "PATCH" => {
                return Ok(Self::PATCH);
            }
            _ => {
                return Err(MethodError);
            }
        }
    }
}

pub struct MethodError;