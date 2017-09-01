% In Rust we Trust
![rust](img/rust.svg)
- Ett säkert systemspråk utan skräpinsamling

# Rundfråga

<!-- Kolla hur många som har hållit på med C, C++ -->

# Innehåll

* Hej världen!
* Bakgrund & Mål
* Huvudconcept
* Avslut
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
            let route = req.extensions.get::<Router>().unwrap();
            let name = match route.find("name") {
               Some(name) => name,
               None => "mysterious stranger"
            };
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
* Avslut
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
* OO-aktigt (metoder, Traits)
* Generics
* Makron
* FFI - C, C++

# Unika säljfördelar

* Minnessäkerhet utan skräpinsamling (Nära unikt) <!-- Även ATS -->
* Trådsäkerhet&#42; genom statisk analys (Nära unikt) <!-- Concurrency without data races -->
* Zero-cost abstractions <!-- Also C++ -->
* Högnivåspråk med lågnivå-kontroll vid behov <!-- Low-level control - `unsafe` -->

# Att återta minne & resurser

* Att bara allokera skapar massa garbage


## Lösning
  * Flesta säkra språk - GC 
  * C - disciplin
  * C++ - deterministisk destruering + disciplin
  * Rust - deterministisk destruering + statisk analys

## Deterministisk destruering 

* Tightare
* Inte bara minne


# Minnessäkerhet

## Enkeltrådat
* Åtkomstfel 
   * Buffer overflow/overread 
   * Use after free
* Oinitialiserade variabler
   * Null pointer access 
   * Wild pointers
* Minnesläckor
   * Double free
   * Invalid free (free invalid address)
   * Mismatched free (free med fel allokator)

## Trådat 
* Osynkroniserad läsning/skrivning - klurig!

# Skapa minnessäkerhet

## Klassisk - GC++
* ta över ansvar för all allokering
* låt saker läcka
* gör runtime garbage analys
* runtime-checkar för bounds 

## Nackdelar
* runtime overhead - latency
* tillåter inte andra allokeringar (flash, minne i gpu, andra algos)
* löser inte freeing av andra resurser (filer, db connections, etc)

## Problemformulering

* Kan vi lösa problemen vid kompilering?
  - få ett säkert språk utan GC?
  - kanske även lösa trådad access?

# Approachen

## Sjukt grinig kompilator
![Stop](img/Stop.jpg)

# Innehåll

* <strike>Hej världen!</strike>
* <strike>Bakgrund & Mål</strike>
* Huvudconcept
* Avslut
* Frågor

# Förebygger läsning av oinitaliserade variabler

<script language="rust">
fn main() {
  let c: u32;
  // c = 15; // FIXME
  println!("{}", c);
}
</script>

# Immutable om inte annat är sagt

<script language="rust">
fn main() {
  let a = 4711;  //FIXME
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

* För vissa typer kopieras data automatiskt

<script language="rust">
fn main() {
  let a = 4711;
  let mut b = a;
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

# Referenser och livstid

* Måste leva kortare än datat den pekar på

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

# Trådsäkerhet

# Trådsäkerhet - skrivning i annan tråd

* Livstider är viktiga här

<script language="rust">
extern crate crossbeam;

fn main() {
  let mut a = String::from("hej");
  std::thread::spawn(|| { a += "a"; });
  //crossbeam::scope(|scope| { 
  //  scope.spawn(|| {a += "a"; });
  //});
  println!("{}", a);
}
</script>

# Trådsäkerhet - ej trådsäkra typer 

* Send - kan skickas till andra trådar
  - under vanliga ägarskapsregler
* Icke send endast för typer med "gömd muterbarhet"

<!-- Rc upprätthåller "flera läsare samtidigt" trots att den har en intern muterbar räknare.
     Om man muterar från flera trådar så behöver man locking eller memory barriers, 
     så därför är den explicit inte sync.
 -->

<script language="rust">
fn main() {
  let rc = std::rc::Rc::new(String::from("hej"));
  let t = std::thread::spawn(move || { println!("{}", rc); });
  t.join().unwrap();
}
</script>

# Trådsäkerhet - multi-skrivning

* Utlåning spelar roll i trådning också

<script language="rust">
extern crate crossbeam;

fn main() {
  let mut a = String::from("hej");
  crossbeam::scope(|scope| { 
    scope.spawn(|| {a += "a"; });
    scope.spawn(|| {a += "a"; });
  });
  println!("{}", a);
}
</script>

# Trådsäkerhet - multi-skrivning löst

* Implementerar Sync - referenser kan delas

<script language="rust">
extern crate crossbeam;

fn main() {
  let a = std::sync::Mutex::new(String::from("hej"));
  crossbeam::scope(|scope| { 
    scope.spawn(|| { *a.lock().unwrap() += "a"; });
    scope.spawn(|| { *a.lock().unwrap() += "a"; });
  });
  println!("{}", a.lock().unwrap());
}
</script>

# Innehåll

* <strike>Hej världen!</strike>
* <strike>Bakgrund & Mål</strike>
* <strike>Huvudconcept</strike>
* Avslut
* Frågor

# Avslut

# Skippat

## Tooling
* Cargo
  - "allt" - bygg, dependencies, formatting, testning, publicering

## Språkfeatures
* "Streams"
* Algebraiska datatyper
* Generics
* Typalias
* Bra enkla datatyper
* If & Loopar är expressions
* Tuples
* Första klassens funktioner
* Lambdor
* Högre ordningens funktioner - map, flatmap, filter etc
* Många saker är expressions



# Sammanfattning
* Systemspråk
* Säkrare än många andra språk
* Högnivå
* Hög prestanda
* Bra interop - minne, resurser, FFI
* Kompilerar -> fungerar?

## Usecase
* Firefox CSS

## Runtkring
* Superb tooling
* https://www.rustup.rs

# Frågor



