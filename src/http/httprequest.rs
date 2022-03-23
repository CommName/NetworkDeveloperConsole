use std::collections::HashMap;
use super::request_line::Method;
use super::HttpVersion;

pub struct HttpRequest {
    pub request_line: HttpRequestLine,
    pub http_version: String,
    pub method: Method,
    pub header: HashMap<String, String>,
    pub body: Vec<u8>
}


pub struct HttpRequestLine {
    pub method: Method,
    pub request_uri: String,
    pub http_version: HttpVersion
}

pub struct HttpRequestParser {

}

impl HttpRequestParser {

    pub fn new() -> Self {
	    Self {
	    }
    }
}

impl HttpRequest {

    pub fn from_bytes(byres: &[u8]) {

    }

}
