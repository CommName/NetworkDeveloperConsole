use std::str::FromStr;
use serde::Deserialize;
use serde::de::{value, IntoDeserializer};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize ="UPPERCASE"))]
pub enum Method{
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect
}

#[derive(Debug, Clone)]
pub struct RequestLine {
    method: Method,
    uri: String,
}

impl FromStr for Method {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	Self::deserialize(s.to_uppercase().into_deserializer())
    }
}

pub struct RequestLineParser<'a> {
    bytes: &'a [u8]
}

impl<'a> RequestLineParser<'a> {

    pub fn new(bytes: &'a[u8]) -> Self {
	Self {
	    bytes: bytes
	}
    }

    pub fn parse(self) -> Result<RequestLine, String> {

	// Method
	let index = self.bytes.iter().position(|x| *x == (' ' as u8)).ok_or("Error deserializing, Request line missing Method".to_string())?;
	let s =std::str::from_utf8(&self.bytes[..index]).map_err(|_| "Not a string".to_string())?;
	let method = Method::from_str(s).map_err(|_| "Error couldn't deserialize Method".to_string())?;

	// URI
	let bytes = &self.bytes[index+1..];
	let index = bytes.iter().position(|x| *x == (' ' as u8)).ok_or("Error deserializing, Request line mising URI")?;
	let uri = std::str::from_utf8(&bytes[..index]).unwrap().to_owned();

	// let bytes = bytes[index..];

	Ok(RequestLine {
	    method,
	    uri
	})
    }
}

