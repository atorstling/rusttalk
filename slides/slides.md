% In Rust we Trust
![rust](img/rust.svg)
- Ett säkert systemspråk utan skräpinsamling

# Rundfråga

<!-- Kolla hur många som har hållit på med C, C++ -->

# Innehåll

* Hej världen!
* Bakgrund & Mål
* Huvudconcept
* Tooling
* Frågor

# Hej världen!

<script language="rust">
fn main() {
    println!("Hej världen!");
}
</script>

# Hej nätet!

<script language="rust">
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
</script>

# Innehåll

* <strike>Hej världen!</strike>
* Bakgrund & Mål
* Huvudconcept
* Tooling
* Frågor

# I begynnelsen

![Graydon Hoare](img/graydon.jpg "Graydon Hoare")

* Startades av Graydon Hoare 2006. OCaml
* Mozilla 2009
* Själv-hysande kompilator 2010
* 1.0 maj 2015
* Nu 1.19 (Aug 2017)

# Mål

* Hitta ett alternative till C++ (Hos Mozilla)
  * Minnessäkerhet
  * Trådsäkerhet
  * Bättre kompilatorsystem
  * Modulsystem

# Unika säljfördelar

* Minnessäkerhet utan skräpinsamling (Nära unikt) <!-- Även ATS -->
* Trådsäkerhet&#42; genom statisk analys (Nära unikt) <!-- Concurrency without data races -->
* Kostanadsfria abstraktioner <!-- Also C++ -->
* Högnivåspråk <!-- High-level language -->
* Lågnivå-kontroll vid behov <!-- Low-level control - `unsafe` -->

# Användanden

* Servo. Parallell webbläsarrenderingsmotor av Mozilla
* Delar av Firefox <!-- Parallell, css styling engine -->
* Dropbox
* Npm
* Samsung (IoT)

# Språkegenskaper

* Systemspråk
* Statisk, stark, härledd typning
* Designat för samtidighet
* Multi-paradigm
* Funktionellt
  * Första klassens funktioner
  * Lambdor
  * Högre ordningens funktioner - map, flatmap, filter etc
  * Många saker är expressions
* OO-aktigt (metoder, Traits)
* Generics
* Makron
* FFI - C, C++

# Innehåll

* <strike>Hej världen!</strike>
* <strike>Bakgrund & Mål</strike>
* Huvudconcept
* Tooling
* Frågor

# Förebygger läsning av oinitaliserade variabler

<script language="rust">
fn main() {
  let c: u32;
  // c = 15; // FIXME
  println!("{}", c);
}
</script>

# Muterbarhet 1

<script language="rust">
fn main() {
  let a: u32 = 4711;  //FIXME
  println!("{}",a);
  a = 4712;
  println!("{}",a);
}
</script>

# Minnessäkerhet utan skräpinsamling.

Hur?

* Livstider
* Ägarskap
* Lån

# Ägarskap
* All data är antingen `statisk` eller ägs av en variabel.
* Vid varje givet tillfälle har datat endast en ägare
* Man kan överföra ägandeskap - move
* Datatyper kan implementera Copy -> två kopior, två ägare
* När data går ur scope frigörs minnet.

=> Automatisk deallokering utan GC!

# Livstider, Automatisk Destruering

<script language="rust">
fn main() {
  struct Test{};

  impl Drop for Test { 
    fn drop(&mut self) {
      println!("hejdå!");
    }
  }

  println!("före");
  {
    let _a = Test{};
  }
  println!("efter");
}
</script>

# Copy

<script language="rust">
fn main() {
  let a = 4711;
  let mut b: u32 = a;
  b+=1;
  println!("{}-{}", a, b);
}
</script>

# Move

* "Tyngre" typer
* Implementerar inte `Copy`
* Default för structs

<script language="rust">
fn main() {
  #[derive(Debug)]
  struct Test{};

  let a = Test{};
  let b = a;
  println!("{:?}", a); 
}
</script>

# Referenser & Lån
* En referens är en vy in i data som ägs av någon annan
* Implicerar ett lån

## Typer 
* `&mut` - Muterbar referens. Ger lån för skrivning.  Exklusivt
* `&` - Icke muterbar referens. Ger lån för läsning. Ej exklusivt

## Regler

* En referens får aldrig leva längre än datat den refererar till

Man kan ANTINGEN ha

* En eller flera lån för läsning ELLER
* Precis ett lån för skrivning

<!-- Datat som en läslån pekar på förändras inte under
lånets varaktighet. -->

# Många lån för läsning

<script language="rust">
fn main() {
  let a = 4711;
  let b = &a;
  let c = &a;
  println!("{:?}", (a,b,c));
}
</script>

# Lån för skrivning

* Muterbara referenser endast till muterbar data
* Muterbara referenser överför skrivrätt

<!-- 1: a should be mutable -->
<!-- 2: end scope before writing to a-->
<script language="rust">
fn main() {
    let a = 47; 
    {
      let b = &mut a;
      *b += 1; 
      a += 1;
    }
    println!("{}", a);
}
</script>

# Låns livstid kollas

<script language="rust">
fn main() {
  let b : &u32;
  {
    let a = 47; 
    b = &a;
  }
  println!("{}", b);  
}
</script>

# Summering minnessäkerhet utan GC

Livstider och ägarskap garanterar säkra referenser
och förhindrar ofrivillig muterbarhet 

# Trådsäkerhet


# If Statement is Expression

<script language="rust">
fn main() {
  let x = if 1 > 0 { 4 } else { 5 };
  println!("{}", x);
}
</script>

# Loops

<script language="rust">
fn main() {
  let a = vec![1, 2, 3, 4];
  for i in a {
    if i % 2 == 0 {
      println!("{}", i);
    }
  }
}
</script>

# Functions

<script language="rust">
fn square(a: u32) -> u32 {
  a * a; // FIXME
}
fn main() {
  println!("{}", square(10));
}
</script>

# Functions Early Return

<script language="rust">
fn is_neg(a: i32) -> String {
  if a < 0 {
    return String::from("yes");
  }
  String::from("no")
}
fn main() {
  println!("{}", is_neg(0));
}
</script>

# Structs

<script language="rust">
#[derive(Debug)]
struct A {
  x: String,
  y: f64
}

fn main() {
  let a = A{ x: "hej".to_string(), y: 7.0 };
  println!("{:?}", a);
}
</script>

# Member Functions

<script language="rust">

struct Person { age: u32 }

impl Person {
  fn print(&self) {
    println!("A person aged {}", self.age);
  }
}
fn main() {
  Person { age: 14 }.print();
}
</script>


# Traits

* Interfaces, typeclasses
* Monomorphization

<script language="rust">
struct Person { age: u32 }

trait Printable {
  fn print(&self);
}

impl Printable for Person {
  fn print(&self) {
    println!("A person aged {}", self.age);
  }
}

impl Printable for u32 {
  fn print(&self) {
    println!("An int with value {}", self);
  }
}

fn main() {
  Person { age: 14 }.print();
  13.print();
}
</script>

# Closures

<script language="rust">
fn main() {
  let mut x = String::from("hej");
  {
    let mut append = | s: &str | { x.push_str(s); };
    append("san");
  }
  println!("{}", x);
}
</script>

# Generic Function Arguments

<script language="rust">
fn print<T: std::fmt::Display>(a: T, b: T) {
  println!("{}-{}", a, b);
}
fn print2<T>(a: T, b: T)
  where T: std::fmt::Display + std::fmt::Debug
{
  println!("{}-{:?}", a, b);
}

fn main() {
  print("hej", 32); //FIXME
  print2("hej", "hej");
}
</script>


# First-class Functions and Closures

<script language="rust">
fn plus_one(a: u32) -> u32 {
  a + 1
}

fn do_twice(f: fn(u32) -> u32, i: u32) -> u32 {
  f(f(i))
}

fn do_twice_2<T: Fn(u32) -> u32>(f: T, i: u32) -> u32 {
  f(f(i))
}


fn main() {
  let _ref: fn(u32) -> u32 = plus_one;
  println!("{}", do_twice(plus_one, 1));
  println!("{}", do_twice_2(plus_one, 1));
  println!("{}", do_twice(| i | { i * 2 }, 4)); // FIXME
}
</script>

# Tuples

<script language="rust">
fn main() {
  let a : (&str, u32) = ("hej", 43);
  println!("{}{}", a.0, a.1);
}
</script>


# Back to Lifetimes and Ownership

# Lifetimes and Functions - Calling

* Move

<script language="rust">
fn gimme(_: String) {
}
fn main() {
  let a = String::from("a");
  {
    gimme(a); //FIXME
  }
  println!("{}", a);
}
</script>


# Lifetimes and Functions - Ref

* Borrow for the lifetime of call

<script language="rust">
fn gimme(_: &String) {
}
fn main() {
  let mut a = String::from("a");
  gimme(&a);
  gimme(&mut a);
  println!("{}", a);
}
</script>

# Lifetimes and Functions - Return

Functions must

* Declare how the lifetime of the return value
  relates to the lifetime of the in-parameters

<script language="rust">
fn unit(a: &str) -> &str {
  a
}
fn main() {
  let a = unit("hej");
  println!("{}", a);
}
</script>

# Lifetimes and Functions - Return

Functions must

* Declare how the lifetime of the return value
  relates to the lifetime of the in-parameters

<script language="rust">
fn unit<'a>(a: &'a str) -> &'a str {
  a
}
fn main() {
  let a = unit("hej");
  println!("{}", a);
}
</script>

# Lifetimes and Functions - Non-Automatic

<script language="rust">
struct Person { name: String }
fn get_first_name(p1: & Person, _: &Person) -> &String { //FIXME
  &p1.name
}
fn main() {
  let p1 = Person { name: "Arne".to_string() };
  let p2 = Person { name: "Ragnhild".to_string() };
  let name = get_first_name(&p1, &p2);
  println!("{}", name);
}
</script>

# Concurrency

# Threading - Basics

<script language="rust">
use std::thread;

fn main() {
  let ta = thread::spawn(|| { println!("in a"); "package from a" });
  println!("in main");
  println!("{}", ta.join().unwrap());
}
</script>

# Threading - Sharing Values

* Ref-Value
* Send

<script language="rust">
use std::thread;

fn main() {
  let i = 3;
  let ta = thread::spawn(|| { println!("a{}", i) }); // FIXME
  ta.join().unwrap();
}
</script>

# Threading - Mutable Values

* Scoping Problem - Crossbeam

<script language="rust">
use std::thread;

fn main() {
  let mut i = 3;
  let ta = thread::spawn(|| { i += 1 });
  println!("i: {}", i);
  //CANT-FIX
}
</script>


# Threading - Mutable Values

<script language="rust">
extern crate crossbeam;

fn main() {
  let mut i = 3;
  crossbeam::scope(|scope| {
    scope.spawn(|| { i += 1; });
    // ADD ONE
  });
  println!("i:{}", i);
}
</script>

# Threading - Mutex

<script language="rust">
extern crate crossbeam;
use std::sync::Mutex;

#[test]
fn mutate_in_threads() {
  let m = Mutex::new(3);
  crossbeam::scope(|scope| {
    scope.spawn(|| {
      let mut lock = m.lock().unwrap();
      *lock += 1;
    });
    scope.spawn(|| {
      let mut lock = m.lock().unwrap();
      *lock += 1;
    });
  });
  assert_eq!(*m.lock().unwrap(), 5);
}
</script>

# Threading - Conclusions

## Thread safety guaranteed by the compiler

## There is more
* Atomics
* Reference Counted
* Channels

# Build

## Demo

# More

# Arrays

<script language="rust">
fn main() {
  let xs: [i32; 5] = [0, 1, 2, 3, 4];
  let ys: [i32; 5] = [77; 5];
  println!("{:?}{:?}", xs, ys);
}
</script>

# Tuple Structs
<script language="rust">
#[test]
fn test() {
  #[derive(Debug)]
  struct Color(u32, u32, u32);
  let a = Color(1,2,3);
  assert_eq!(format!("{:?}", a), "Color(1, 2, 3)");
}
</script>

# Enums

Algebraic. A.k.a Case class, Data

<script language="rust">
#[derive(Debug)]
enum Animal {
  Horse { tail_length_mm: u32 },
  Moose(i32,u64),
  Duck { quackiness_dba: f64, diving: bool },
  Snake
}
fn main() {
  let (a, b) = (Animal::Snake,
                Animal::Duck { quackiness_dba : 7.0, diving : false });
  let c: Animal = Animal::Moose(46,46);
  let d = Animal::Horse { tail_length_mm: 16 };
  println!("{:?}", (a, b, c, d));
}
</script>

# Match

<script language="rust">
fn main() {
  let x = 5;
  match x {
    1 | 2 => println!("small"),
    _ => println!("big")
  }
}
</script>

# Match Destructuring

<script language="rust">
enum Thing {
  Shoesize(u32),
  Coord { x: u32, y: u32 }
}

fn main() {
  let x: Thing = Thing::Coord { x: 13, y:47 };
  let _y = Thing::Shoesize(14);
  match x {
    Thing::Shoesize(s) => println!("shoesize {}", s),
    Thing::Coord { x, y } => println!("[{}, {}]", x, y)
  }
}
</script>

# Iterators

<script language="rust">
fn main() {
  let i = (1..10)
  .filter(|i| { i % 2 == 0 })
  .fold(0, | acc, i | { acc + i });
  println!("{}", i);
}
</script>

# Destructors

<script language="rust">

struct A {}

impl Drop for A {
  fn drop(&mut self) {
    println!("I am the weakest link, goodbye!");
  }
}

fn main() {
  {
    let _a = A{};
  }
  println!("Carry on!");
}
</script>

# Generic Structs

<script language="rust">
struct Wrapped<T> {
  pub value: T
}

fn main() {
  let a = Wrapped{ value: "hej".to_string() };
  let mut b = Wrapped{ value: 7 };
  b = a; //FIXME
}
</script>

# Type Aliases

<script language="rust">
type Alex = String;
fn main() {
  let _a: Alex = String::from("Alex");
}
</script>

# Thank You

<img src="img/stackoverflow.png" alt="stackoverflow" style="width: 400px;"/>
![](img/most_loved.png "Most Loved")

# Left

x Deconstruction

x Everything is an expression

x Desctructors

x Safety

x No gc

x Borrow Checker

x Type inference

x Concurrency

x Generics

x Monomorphisation

x Closures

x Mut

x Structs

x Traits

x Memory safety without garbage collection

x Concurrency without data races

x Abstraction without overhead

Tests parallel by default

- Type aliases

# Phone Notes

x No gc latencies

x Destructors

x Safety

x No gc

x Borrow checker

x Type inference

x Concurrency

x Generics

x monomorphisation ni

x Mut

x Structs

x Traits

x Memory safety without garbage collection
Concurrency without data races

x Inga

x This post begins exploring the third pillar:

x Abstraction without overhead.

x "If" is an expr


x Mutable binding vs mutable reference

x let mut x = 4;

x let mut y = &mut x;

x x: MutBind -----> 0x0bc32: 4

x y: MutBind -----> mutBorrow ----> 0x0bc32: 4

First slide: http://venge.net/graydon/talks/intro-talk-2.pdf

# CPU

![](img/debian-lowest-cpu.svg "Lowest CPU")

# Säkerhetsaspekter

* Inga godtyckliga pekare
* Inga nullpekare
* Oföränderlig om inget annat är sagt (immutable by default)
* I princip inget globalt state
* Bounds checking, eller eleminiering därav
* Delat minne måste vara trådsäkert


# Hastighet

![](img/debian-fastest-time.svg "Fastest Time")

# Lån förhindrar mutering

Gäller även lån för läsning

<script language="rust">
fn main() {
  let mut a = 4711;
  let b = &a;
  a = 4712; 
  println!("{:?}", (a,b));
}
</script>

# Lån för skrivning

* Lån för skrivning överför "åtkomsträtten" 

<script language="rust">
fn main() {
  let mut a = 4711;
  let _b = &mut a;
  let _c = a;
}
</script>

# References - Summary

* You can share variables through references
* Writing to a variable behaves like read/write locks
  * Write is exclusive
  * Read is concurrent

# Important Data Types and Constructs

# `str`

String constants are of type `str`.

`static` - part of the data segment of the executable.

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

`String`s are mutable.

<script language="rust">
fn main() {
  let mut a: String = String::from("hej");
  a.push_str(" på dig");
  println!("{}", a);
}
</script>

# If Statement

<script language="rust">
fn main() {
  if 0 < 1 {
    println!("hej");
  }
}
</script>

