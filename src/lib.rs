extern crate time;
extern crate futures;
extern crate tokio_core;
extern crate tk_bufstream;
extern crate minihttp;
extern crate serde_json;
extern crate httparse;

mod service;
mod pages;
mod response;

pub use service::HttpBin;
