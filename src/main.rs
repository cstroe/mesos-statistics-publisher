// https://hyper.rs/guides/server/hello-world/

extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;
extern crate simple_logger;

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use log::Level;

struct HelloWorld;

const PHRASE: &'static str = "Hello, World!";

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();

    info!("Starting server at http://localhost:3000");
    server.run().unwrap();
}