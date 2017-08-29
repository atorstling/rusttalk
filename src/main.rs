#![feature(fnbox)]
extern crate iron;
extern crate router;
extern crate rustc_serialize;
use iron::prelude::*;
use iron::{status, Listening};
use router::Router;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Answer {
    msg: String,
}

fn server(port: &str) -> Listening {
    let mut router = Router::new();
    router.get(
        "/hello/:name",
        |req: &mut Request| {
            let name = req.extensions.get::<Router>().unwrap().find("name").unwrap();
            let ans = Answer {
                msg: format!("hello {}!", name).to_string(),
            };
            let payload = json::encode(&ans).unwrap();
            Ok(Response::with((status::Ok, payload)))
        },
        "hello_world",
    );
    Iron::new(router)
        .http(format!("localhost:{}", port))
        .unwrap()
}

fn main() {
    let _server = server("9999");
    println!("listening on port 9999");
    std::thread::park();
    panic!("spurious wakeup");
}

#[cfg(test)]
mod test;
