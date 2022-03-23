use std::collections::HashMap;
use std::sync::RwLock;
use std::net::IpAddr;
use crate::http::httprequest::HttpRequest;
use crate::http::httpresponse::HttpResponse;

pub struct HttpStore {
    http_pairs: RwLock<HashMap<IpAddr, HttpPair>> 
}

impl HttpStore {

    pub fn new() -> Self {
        let http_pairs = RwLock::new(HashMap::new());
        Self {
            http_pairs
        }
    }
}

pub struct HttpPair {
    request: Option<HttpRequest>,
    response: Option<HttpResponse>,
}

impl HttpPair {

    pub fn new() -> Self {

	Self {
	    request: None,
	    response: None,
	}
    }

    pub fn update_request(self, request: HttpRequest) {
	    //self.request.replace(request);
    }

    pub fn update_response(self, response: HttpResponse) {
	    //self.response.replace(response);
    }

}
