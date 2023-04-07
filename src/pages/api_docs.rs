use std::str::{FromStr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use httpdate::HttpDate;
use tk_http::Status;
use tk_http::server::{Codec, Error, Encoder, RecvMode};
use futures::{Async};
use futures::future::{ok};

use pages::{ResponseFuture, Response};
use service::{Request};

pub struct CustomStatus {
    _status: Status,
    _prefix: Arc<PathBuf>,
}

const PAGE: &str = "";

impl<S: 'static> Codec<S> for CustomStatus {
    type ResponseFuture = ResponseFuture<S>;

    fn recv_mode(&mut self) -> RecvMode {
        RecvMode::buffered_upfront(0)
    }
    fn data_received(&mut self, data: &[u8], end: bool)
        -> Result<Async<usize>, Error>
    { 
        assert!(end);
        assert!(data.len() == 0);
        Ok(Async::Ready(0))
    }
    fn start_response(&mut self, mut e: Encoder<S>) -> ResponseFuture<S> {
        e.status(Status::PermanentRedirect);
        e.add_length(0 as u64).unwrap();

        e.format_header("Date", HttpDate::from(SystemTime::now())).unwrap();
        e.add_header("Location", "https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/src/templates/openapi.yaml&poo=1&nocors").unwrap();
        e.add_header("Server",
            concat!("httpbin-rs/", env!("CARGO_PKG_VERSION"))
        ).unwrap();

        if e.done_headers().unwrap() {
            e.write_body(PAGE.as_bytes());
        }
        Box::new(ok(e.done()))
    }
}

pub fn serve<S: 'static>(req: Request) -> Response<S> {
    let parsed = req.suffix().to_str()
        .and_then(|s| u16::from_str(s).ok())
        .and_then(|x| Status::from(x));
   let status = match parsed {
        Some(status) => status,
        None => Status::BadRequest,
    };
    Box::new(CustomStatus { _status: status, _prefix: req.prefix().clone() })
}
