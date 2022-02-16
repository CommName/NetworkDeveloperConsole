pub mod request_line;


pub enum Htpp {
    HttpRequest,
    HttpResponse,
}


pub struct HttpVersion {
    pub major: u8,
    pub minor: u8
}

pub trait Http {
    fn get_header();
    fn get_body();
}

pub fn from_raw(bytes: &[u8]) {
    let parser = request_line::RequestLineParser::new(bytes);

    let method = parser.parse();

    match method {
	Ok(request) => {
	    println!("{:?}", request);
	},
	Err(e) => {
	    
	}
    }
}


trait State<T> {
    fn next_step(previouse_state_result: T, bytes: [u8]);
}
