% Rust - So Familiar yet so Different

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
extern crate iron;
extern crate router;
extern crate rustc_serialize;
use iron::prelude::*;
use iron::{status, Listening};
use router::Router;
use rustc_serialize::json;

fn server(port: &str) -> Listening {
    let mut router = Router::new();
    router.get("/yo/:phrase", get_yo, "get_yo");
    router.put("/yo",
               |_: &mut Request| Ok(Response::with((status::ImATeapot, "no"))),
               "put_yo");
    Iron::new(router).http(format!("localhost:{}", port)).unwrap()
}

#[derive(RustcEncodable)]
struct Answer {
    msg: String,
}

fn get_yo(req: &mut Request) -> IronResult<Response> {
    let phrase = req.extensions.get::<Router>().unwrap().find("phrase").unwrap();
    let ans = Answer { msg: format!("yo {}!", phrase).to_string() };
    let payload = json::encode(&ans).unwrap();
    Ok(Response::with((status::Ok, payload)))
}
    
fn main() {
    let _server = server("9999");
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
