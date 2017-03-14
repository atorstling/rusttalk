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
* Self-hosting compiler in 2010

# Goals

* Find Alternative to C++ (at Mozilla)
  * Memory safety
  * Thread safety
  * Compilation model
  * Module system, dependencies

# Why not Existing Languages?

* Complex GCs
* Different Paradigms
* Not enough static type checks
* Few with isolation, interference, concurrency guarantees

# Main Selling Points

* Memory safety without garbage collection (Novelish)
* Concurrency without data races (Novelish)
* Abstraction without overhead
* High-level language
* Low-level control - `unsafe`

# Language Properties

* System language
* Static, strong, inferred typing
* Compiled on LLVM
* Concurrent
* Imperative, Procedural
* Functional
  * First-class functions
  * Pattern matching
  * Ad-hoc polymorphism through Traits
  * Lambdas, Closures
  * Iterators (map, flatmap, filter etc)

# Language Properties 2

* Generics
* Macros
* FFI - C, C++, interop
* `Cargo` complete solution for 
  * setting up projects
  * building 
  * formatting
  * testing
  * sharing code

# But

* No Real Reflection
* No Green Threads or Tasks

# Compared to other Languages

# Speed

![](img/debian-fastest-time.svg "Fastest Time")

# CPU

![](img/debian-lowest-cpu.svg "Lowest CPU")

# Basics

# Primitives

signed integers: `i8`, `i16`, `i32`, `i64` and `isize` (pointer size)

unsigned integers: `u8`, `u16`, `u32`, `u64` and `usize` (pointer size)

floating point: `f32`, `f64`

char Unicode scalar values like `'a'`, `'å'` and `'∞'` (4 bytes each)

bool either `true` or `false`

and the unit type `()`, whose only value is also `()`

arrays like `[1, 2, 3]`

tuples like `(1, true)`

# Variable bindings

<script language="rust">
fn main() {
  let a: u32 = 4711;
  println!("1: {}",a);

  let (b, c) = (42, 1337i32);
  println!("2: {}-{}", b, c);
}
</script>

# Prevent Read of Uninitialized

<script language="rust">
fn main() {
  let c: u32;
  // c = 15; // FIXME
  println!("{}", c);
}
</script>

# Mutability 1

The *binding* is mutable or not

<script language="rust">
fn main() {
  let a: u32 = 4711;  //FIXME
  println!("{}",a);
  a = 4712;
  println!("{}",a);
}
</script>

# Ownership

* All data is either owned by some binding or `static`.
* There is only one owner of any given data.
* Ownership can be transfered - Move
* Sometimes data is copied instead -> Two pieces of data, two owners.
* When owner goes out of scope, data will be freed.

# Lifetimes, Automatic Destruction

<script language="rust">
fn main() {
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
}
</script>

# Copy

<script language="rust">
fn main() {
  let a = 4711;
  let b: u32 = a;
  println!("{}-{}", a, b);
}
</script>

# Move

* "Heavier" types
* Don't implement `Copy`

<script language="rust">
fn main() {
  let a = String::from("hej");
  let b: String = a;
  println!("{}", a); //FIXME
}
</script>

# References
* Views into data owned by someone else

## Types
* `&`
* `&mut`

## Rules
* A reference cannot outlive its referent

# Borrowing
Taking a reference "borrows" the data.
You can borrow for read or write. Still a borrow.

You can have EITHER

* one or more immutable references (&T) to a resource OR
* exactly one mutable reference (&mut T).

# Immutable References - Can Have Many

Can have many immutable references

-- Borrow immutably many times

<script language="rust">
fn main() {
  let a: u32 = 4711;
  let b: &u32 = &a;
  let c: &u32 = &a;
  println!("{:?}", (a,b,c));
}
</script>

# Immutable References - Mutate

You cannot mutate if borrowed. Even if only immutably borrowed

<script language="rust">
fn main() {
  let mut a: u32 = 4711;
  let b: &u32 = &a;
  a = 4712;
  println!("{:?}", (a,b));
}
</script>

# Mutable References 2

* Borrowing mutably "transfers access rights"

<script language="rust">
fn main() {
  let mut a: i64 = 4711;
  let b: &mut i64 = &mut a;
  let c = a; // FIXME
}
</script>

# Mutability 2

But you cannot "undo" immutability

<script language="rust">
fn main() {
    let a: i32 = 47;
    let b: &mut i32 = &a;
    *b = 48;
}
</script>


# `str` and references

String constants are of type `str`.

Part of the data segment of the executable.

Immutable. Have to refer to them by const reference

<script language="rust">
fn main() {
  let a: str = "hej"; //FIXME
  let b: &mut str = "på"; //FIXME
  let c: &str = "dig";
  println!("{}", c);
}
</script>

# `String`s

`String`s are mutable strings.

<script language="rust">
fn main() {
  let a: String = String::from("hej");
  let b: String = "på".to_string();
  let c: String = a + &b;
  println!("{}", c);  
}
</script>


# Function declarations

<script language="rust">
fn square(a: u32) -> u32 {
  a * a; // FIXME
}
fn main() {  
  println!("{}", square(10));
}
</script>

# Conditionals

<script language="rust">
fn main() {
  if 0 < 1 {
    println!("hej");
  }

  println!("{}", bigger(3, 2));

  let x = if 1 > 0 { 4 } else { 5 };
  println!("{}", x);  
}

fn bigger(a: u32, b: u32) -> String {
  if a > b {
    "yes".to_string()
  } else {
    "no".to_string()
  }
}
</script>

# Tuples

<script language="rust">
fn main() {
  let a = ("hej", "hå");
  println!("{}{}", a.0, a.1);

  fn test(a: u32) -> (u32, u32) {
    (a+1, a+2)
  }
  let b = test(4);
  println!("{:?}", b)
}
</script>

# Structs

<script language="rust">

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

# Left

Deconstruction
Everything is an expression
Desctructors
Safety
No gc
Borrow Checker
Type inference
Concurrency
Generics
Monomorphisation
Closures
Mut
Structs
Traits
Memory safety without garbage collection
Concurrency without data races
Abstraction without overhead
Tests parallel by default
Type aliases

# Phone Notes

No gc latencies
Destructors
Safety

No gc
Borrow checker
Type inference
Concurrency
Generics

monomorphisation ni


Mut
Structs
Traits

Memory safety without garbage collection
Concurrency without data races
Inga
This post begins exploring the third pillar:

Abstraction without overhead.

"If" is an expr


Mutable binding vs mutable reference

let mut x = 4;
let mut y = &mut x;
x: MutBind -----> 0x0bc32: 4
y: MutBind -----> mutBorrow ----> 0x0bc32: 4

First slide: http://venge.net/graydon/talks/intro-talk-2.pdf
