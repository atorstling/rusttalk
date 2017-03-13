% Rust

![alt text](img/rust.svg)
Alexander Torstling

# Poll

# Hello, World!

<script language="rust">
fn main() {
    println!("Hello, World!");
}
</script>

# Hello, Web!

<script language="rust">
extern crate hyper;
extern crate hyper_router;
extern crate rustc_serialize;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper_router::{Route, RouterBuilder};
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Answer {
    msg: String,
}

fn get_yo(_: Request, res: Response) {
    let ans = Answer { msg: "mtg_bootstrap!".to_string() };
    let payload = json::encode(&ans).unwrap();
    res.send(payload.as_bytes()).unwrap();
}

fn put_yo(_: Request, mut res: Response) {
    *res.status_mut() = StatusCode::ImATeapot;
    res.send(b"no").unwrap();
}

struct AutoServer {
    listening: hyper::server::Listening,
}

impl AutoServer {
    pub fn new(port: &str) -> AutoServer {
        let router = RouterBuilder::new()
            .add(Route::get("/yo").using(get_yo))
            .add(Route::put("/yo").using(put_yo))
            .build();
        let root_handler = move |req: Request, mut res: Response|
          match router.find_handler(&req) {
            Ok(handler) => handler(req, res),
            Err(sc) => *res.status_mut() = sc,
          };
        let addr = format!("127.0.0.1:{}", port);
        let server = Server::http(addr).unwrap().handle(root_handler).unwrap();
        AutoServer { listening: server }
    }
}

impl Drop for AutoServer {
    fn drop(&mut self) {
        self.listening.close().unwrap();
    }
}

fn main() {
    let _server = AutoServer::new("9999");
    println!("listening on port 9999");
    std::thread::park();
    panic!("spurious wakeup");
}
</script>

# Background

# Early Stages

![Graydon Hoare](img/graydon.jpg "Graydon Hoare")

Graydon Hoare

# Yo


<script language="rust">
//This can be executed on Playpen
fn main() {
    println!("Hello, world!");
}
</script>

```rust
//This is display only code
fn main() {
    println!("Hello, World!");
}
```
