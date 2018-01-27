extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    Iron::new(router).http("localhost:8080").unwrap();

    fn handler(_req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World")))
    }
}
