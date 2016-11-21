extern crate futures;
extern crate tokio_core;
extern crate tokio_service;

use tokio_core::reactor::Core;
use tokio_service::Service;

struct SearchService {
    searcher_a: SegmentSearchService,
    searcher_b: SegmentSearchService,
    searcher_c: SegmentSearchService
}

impl SearchService {
    fn new() -> SearchService {
        SearchService {
            // TODO: initialize each with a different dictionary
            searcher_a: SegmentSearchService::new(),
            searcher_b: SegmentSearchService::new(),
            searcher_c: SegmentSearchService::new(),
        }
    }
}

// TODO: change this to return a *set* of results
impl Service for SearchService {
    type Request = String;
    type Response = Vec<String>;
    type Error = std::io::Error;
    type Future = futures::Collect<

            Vec<futures::Done<String, Self::Error>>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // TODO: maybe return a Maybe String instead
        futures::collect(
            vec![self.searcher_a.call(req.clone()),
                 self.searcher_b.call(req.clone()),
                 self.searcher_c.call(req)])
    }
}

struct SegmentSearchService {
}

impl SegmentSearchService {
    fn new() -> SegmentSearchService {
        SegmentSearchService {}
    }
}

impl Service for SegmentSearchService {
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

struct RewriteRequest {
    user: String,
    query: String
}

struct RewriteService {}

impl RewriteService {
    fn new() -> RewriteService {
        RewriteService {}
    }
}

impl Service for RewriteService {
    type Request = RewriteRequest;
    type Response = String;
    type Error = std::io::Error;
    type Future = futures::Done<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let new_query = if req.user.as_str() == "root" { "*".to_string() } else { req.query };
        futures::done(Ok(new_query))
    }
}

fn main() {
    // Core is the event loop. Its handle is a proxy for the service itself (so that we don't have
    // to worry about refs, I guess?)
    let mut core = Core::new().unwrap();

    let search = SearchService::new();
    let search_result = core.run(search.call("a".to_string())).unwrap();
    // println!("Result is: {}", search_result);
}
