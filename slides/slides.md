% Rust - So Familiar yet so Different

![alt text](img/rust.svg)

Alexander Torstling

# Poll

# Disclaimer

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

* Started by Graydon Hoare 2006. OCaml
* Mozilla 2009
* Compiler Bootstrapped 2010

# Goals

* Find Alternative to C++ (at Mozilla?)
  * Memory safety
  * Thread safety
  * Compilation model
  * Module system, dependencies

# Why not Existing Languages?

* Complex GCs
* Different Paradigms
* Not enough static type checks
* Few with isolation, interference, concurrencey guarantee

# Compared to other Languages

# Speed

![](img/debian-fastest-time.svg "Fastest Time")

# CPU

![](img/debian-lowest-cpu.svg "Lowest CPU")

# Basics

# Primitives

signed integers: i8, i16, i32, i64 and isize (pointer size)

unsigned integers: u8, u16, u32, u64 and usize (pointer size)

floating point: f32, f64

char Unicode scalar values like 'a', 'å' and '∞' (4 bytes each)

bool either true or false

and the unit type (), whose only value is also ()

arrays like [1, 2, 3]

slices like &[char]

tuples like (1, true)

string constants of type str like "hej"

functions (first class): fn main(){}

# Variable bindings

<script language="rust">
fn main() {
  let a: u32 = 4711;  
  println!("1: {}",a);

  let (b, c) = (42, 1337i32);
  println!("2: {}-{}", b, c);

  let c: u32;
  // c = 15;
  println!("3: {}", c);
}
</script>

# Function declarations

<script language="rust">
fn square(a: u32) -> u32 {
  a * a
}
fn main() {  
  println!("{}", square(10));
}
</script>

# Tuples

<script language="rust">
fn main() {
  let a = ("hej", "hå")
  println!("{}{}", a.0, )
}
</script>

# Enums

<script language="rust">
enum Animal {
  Horse(tail_length_mm: u32),
  Duck(quackiness_dba: f64, diving: bool)

}
</script>

# Main Features

# Memory Safety

<script language="rust">
fn main() {
  let mut a: u32 = 1;
  let b: &mut u32 = &mut a;
  println!("{}", a);
}
</script>
