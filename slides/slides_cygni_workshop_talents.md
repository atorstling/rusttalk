% Rust - Trådning
![rust](img/rust.svg)

# Översikt

* Grundkoncept
* Läsning & livstider
* Skrivning
* Samtidig läsning & skrivning
* Send & sync
* Sammanfattning

# Grundkoncept

* Samma låneregler:
   * Flera samtidiga läsare (i olika trådar) XOR
   * En enda skrivare (i olika trådar)

```
CREATE
READ | READ | READ
WRITE
READ | READ
WRITE
```

<!--
  Varför funkar detta??
  Läsning Funkar för att läsning av effectively final variabel inte kräver sync
  Skrivning i annan tråd inget problem så länge ingen annan ser

  Kommer gå igenom detta gradvis
-->

# En tråd utan delad data

<script language="rust">
use std::thread::spawn;

fn main() {
    let thread = spawn(|| {
        println!("hej");
    });
    thread.join().unwrap();
}
</script>

<!-- Join returnerar Err om tråden panikade -->

# En tråd som ges data

<script language="rust">
use std::thread::spawn;

fn main() {
    let s = "hej".to_owned();
    let thread = spawn(move || {
        println!("{}", s);
    });
    thread.join().unwrap();
}
</script>

# En tråd som ger tillbaka data

<script language="rust">
use std::thread::spawn;

fn main() {
    let thread = spawn(|| {
        "hej".to_owned()
    });
    println!("{}", thread.join().unwrap());
}
</script>

<!--
  Trådar har returvärden
  Funkar också med Send!   
-->

<!-- 
   Move ger datat till barn-tråd 
   Varför går detta? Jo, för att String är Send
-->

# En tråd som lånar data för läsning

<script language="rust">
use std::thread::spawn;

fn main() {
    let s = "hej".to_owned();
    let thread = spawn(|| {
        println!("{}", s);
    });
    thread.join().unwrap();
}
</script>

<!-- Funkar för funktioner, men ej för trådar
    Vill låna för evigt. Tråden har ej slut.
-->

# thread::spawn och livstider

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, 
```

<!--
  'static implicerar statiskt livstid -> "samma livstid som programmet"
-->

# En scopad tråd som lånar data för läsning

<script language="rust">
extern crate crossbeam;

fn main() {
    let s = "hej".to_owned();
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          println!("{}", s);
        });
    });
}
</script>

<!-- 
  Funkar eftersom scope garanterar avslut 
   -> tidsbegränsat lån
-->

# Flera trådar som lånar för läsning

<script language="rust">
extern crate crossbeam;

fn main() {
    let s = "hej".to_owned();
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          println!("t1: {}", s);
        });
    });
    crossbeam::scope(|scope| {
        scope.spawn(|| { 
          println!("t2: {}", s);
        });
    });
}
</script>

<!--
 Ta bort mittendel - flera samtidiga läsningar
  Kör flera gånger - ordning odefinierad
-->

# En tråd som lånar för skrivning

<script language="rust">
extern crate crossbeam;

fn main() {
    let mut s: String = "hej".to_owned();
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          s.push_str(" på dig");
        });
    });
    println!("huvudtråd säger {}", s);
}
</script>

# Flera trådar som lånar för skrivning

<script language="rust">
extern crate crossbeam;

fn main() {
    let mut s: String = "hej".to_owned();
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          s.push_str(" på");
        });
    });
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          s.push_str(" dig");
        });
    });
    println!("huvudtråd säger {}", s);
}
</script>

<!--
  Editera bort mittendel - samtidiga skrivningar tillåts ej
-->

# Samtidig läsning och skrivning kräver synkronisering

* Mutex - En i taget
* RwLock - Flera samtidiga läsare XOR en samtidig skrivare
* Atomics - Högprestanda, tex eventually consistent multiple writers

<!-- 
 Mutex - I Java `synchronized`
 RwLock - Likt lånereglerna. I Java `ReadWriteLock`.
-->

# Synkronisering - Mutex

* En i taget

<script language="rust">
extern crate crossbeam;
use std::sync::Mutex;

fn main() {
    let s1 = "hej".to_owned();
    let m: Mutex<String>  = Mutex::new(s1);
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          m.lock().unwrap().push_str(" på");
        });
        scope.spawn(|| { 
          m.lock().unwrap().push_str(" dig");
        });
    });
    let s2 = m.into_inner().unwrap();
    println!("huvudtråd säger {}", s2);
}
</script>

<!--
  Förklara att vi ger s1 till mutexen och sedan plockar
  ut igen till s2

  ordning garanterad? kör några gånger
-->

# Synkronisering - RwLock

* En skrivare i taget, flera läsare

<script language="rust">
extern crate crossbeam;
use std::sync::RwLock;

fn main() {
    let s1 = "hej".to_owned();
    let l: RwLock<String>  = RwLock::new(s1);
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          l.write().unwrap().push_str(" på");
        });
        scope.spawn(|| { 
          println!("barntråd 2 ser {}", l.read().unwrap());
        });
        scope.spawn(|| { 
          println!("barntråd 3 ser {}", l.read().unwrap());
        });
    });
    let s2 = l.into_inner().unwrap();
    println!("huvudtråd säger {}", s2);
}
</script>

<!--
  ordning garanterad? Kör några gånger.
-->


# Hur funkar Mutex o dyl?

* Tillåter multipla skriv-lån - varför klagar inte kompilatorn?
* Utger sig för att vara immutable
* Interior mutability <- Thread safe??

<!--
  Utger sig för att vara immutable
     -> kompilator tillåter flera immutable - lån
     -> men i själva verket är det flera mutable-lån
     -> kallas interior mutability
  Vän av ordning undrar - hur funkar interior mutability med trådsäkerhet?
  Det gör det inte - måste spärra
-->

# Send och Sync 

* Extra marker-interface för typer:
   * Send - Ägarskap går att flytta till en annan tråd
   * Sync - Referenser går att flytta till en annan tråd
* Kan spärra eller låsa upp

<script language="rust">
pub unsafe trait Sync { }
pub unsafe trait Send { }
</script>

<!--
  Sync === Referens är Send
  Spärra - flagga som ej trådsäker - Rc
  Låsa upp - flagga att man löser trådningsproblem - mutex
-->

# Send och Sync i thread::spawn

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, 
```

<!-- 
 Kräver att datat är send
-->

# Send och Sync för Standardtyper

* Enkla typer är Send och Sync
* "Ärvs" -> POJOS är Send och Sync
* Ej för vissa "smarta" eller externa typer som
  * Rc, Cell, MutexGuard, Pekare.

<!-- 
 Stora majoriteten av typer är både sync och send.
 Är opt-out för vissa typer.
 Websockets är inte sync tex. Bundna till server-tråd
 Vad är en smart typ? Exempel
-->


# Send & Sync - exempel

<script language="rust">
#![feature(optin_builtin_traits)]
extern crate nix;
extern crate crossbeam;

#[derive(Debug)]
struct TidHolder { tid : nix::unistd::Pid }
impl TidHolder {
    fn new() -> TidHolder {
          TidHolder { tid: nix::unistd::gettid() }
    }
}
// Pid would be different in other thread
impl !Send for TidHolder {}

fn main() {
  let ph = TidHolder::new();
  crossbeam::scope(|scope| {
    scope.spawn(move || {
      println!("{:?}", ph);
    });
  });
}
</script>

<!--
  Explain - type which holds pid. Unique for thread
  Remove move -> runs
  Replace !Send with !Sync -> does not run
-->

# Riktigt exempel - Message Passing
<script language="rust">
use std::thread::spawn;
use std::sync::mpsc::{Sender, Receiver, channel};

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let _thread = spawn(|| {
        tx.send(", World!".to_owned()).unwrap();
    });
    println!("Hello {}", rx.recv().unwrap());
}
</script>

<!-- 
 Visa felmeddelande och peka på att den inte implementerar Sync
 Men den implementerar send, så kan ge bort
   -> Kan ge till annan tråd, men inte dela mellan trådar
 Sätt tillbaka move
-->




# Sammanfattning

* Samma regler som för vanlig utlåning
* Vanliga trådar har oändlig livstid -> lämnar aldrig tillbaka lån
* Crossbeam::scoped ger tidsbegränsade trådar
* Bryterna man mot lånereglerna, tex flera parallella skrivare så
   * Måste man synka med Mutex el dyl
* En del typer är av naturen ej trådsäkra
  * Styrs med Sync & Send

# Utelämnat

* Rc, Arc

# Frågor

# Extra Slide om att implementera Sync  

<script language="rust">
#![feature(optin_builtin_traits)]
struct OSynkTyp {}
impl !Sync for OSynkTyp {}

#[derive(Debug)]
struct SynkTyp {
	osynk: *mut u32
}
unsafe impl Sync for SynkTyp{}

fn main() {
    let _ = OSynkTyp{};
    let mut nummer = 10u32;
    let s = SynkTyp{ osynk: &mut nummer };
		println!("pekare: {:?}", s);
}
</script>

# Extra Slide om hur Mutex funkar

<script language="rust">
extern crate crossbeam;
use std::sync::{Mutex, MutexGuard};
use std::ops::DerefMut;

fn main() {
    let m = std::sync::Mutex::new(0u32);
    crossbeam::scope(|scope| {
      scope.spawn(|| {
       let m_ref_1: & Mutex<u32> = &m;
       let mut g: MutexGuard<u32> = m_ref_1.lock().unwrap();
       let d: &mut u32 = g.deref_mut();
       *d+=1;
      });
      scope.spawn(|| {
       let m_ref_2: & Mutex<u32> = &m;
       let mut g: MutexGuard<u32> = m_ref_2.lock().unwrap();
       let d: &mut u32 = g.deref_mut();
       *d+=1;
      });
    });
    println!("final result:{}", m.into_inner().unwrap());
}
</script>

# Extra slide - example code

```rust
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
```

# Extra slide - hello world

```rust
fn main() {
  println!("Hej, världen!");
}
```



