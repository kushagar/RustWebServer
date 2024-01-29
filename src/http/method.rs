
use std::str::FromStr;
#[derive(Debug)]
pub enum Method{
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    }
impl FromStr for Method{
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s{
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "HEAD" => Ok(Self::HEAD),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _=> Err(MethodError),
        }
    }
}

pub struct  MethodError;