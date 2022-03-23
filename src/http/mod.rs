pub mod request_line;
pub mod httprequest;
pub mod httpresponse;
use httparse::{Request, Response, Status, Error};

pub enum Http {
    HttpRequest,
    HttpResponse,
}


pub struct HttpVersion {
    pub major: u8,
    pub minor: u8
}

pub fn parse_bytes(bytes: &[u8]) -> Result<Status<Http>, Error> {
    
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    // TODO if error parse Response
    let result = req.parse(bytes)?;

    match result {
        Status::Complete(req) => {
            // TODO parse request
            Ok(Status::Complete(Http::HttpRequest))
        },
        Status::Partial => {
            Ok(Status::Partial)
        }
    }
}


trait State<T> {
    fn next_step(previouse_state_result: T, bytes: [u8]);
}
