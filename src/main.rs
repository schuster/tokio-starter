extern crate futures;
extern crate tokio_core;
extern crate tokio_service;

use tokio_core::reactor::Core;
use tokio_service::Service;

struct SearchService {}

impl SearchService {
    fn new() -> SearchService {
        SearchService {}
    }
}

impl Service for SearchService {
    type Request = String;
    type Response = String;
    type Error = std::io::Error;
    // TODO: add some random timing into this future
    type Future = futures::Done<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // TODO: maybe return a Maybe String instead
        let req_slice: &str = &req;
        let raw_result =
            match req_slice {
                "a" => "x",
                "b" => "y",
                _ => "result not found",
            };
        futures::done(Ok(raw_result.to_string()))
    }
}

// New idea: build a fake version of Eriksen's psearch service: shows the main ideas


fn main() {
    // Core is the event loop. Its handle is a proxy for the service itself (so that we don't have
    // to worry about refs, I guess?)
    let mut core = Core::new().unwrap();

    let search = SearchService::new();
    let search_result = core.run(search.call("a".to_string())).unwrap();
    println!("Result is: {}", search_result);
}
