% Rust - Trådning
![rust](img/rust.svg)

# Grundkoncept

* Samma regler som för borrows:
   * Flera samtidiga läsare (i olika trådar) eller
   * En enda skrivare (i olika trådar)
* Extra marker-interface för typer där 
   * Ägarskap går att flytta till en annan tråd - Send
   * Det går att referera till från en annan tråd - Sync

<!--
  Läsning Funkar för att läsning av effectively final variabel inte kräver sync
  Skrivning i annan tråd inget problem så länge ingen annan ser

  Metoder som skickar data till andra trådar kräver Send, men 
   vad kompilatorn beträffar är det vanligt Trait. 

  Kommer gå igenom detta gradvis
-->

# En Tråd

<script language="rust">
fn main() {
    let thread = std::thread::spawn(|| {
        println!("Hello, world!");
    });
    thread.join().unwrap();
}
</script>

<!-- Join returnerar Err om tråden panikade -->

# Trådar har returvärden

<script language="rust">
fn main() {
    let thread = std::thread::spawn(|| {
      "Hej, förälder!"
    });
    println!("{}", thread.join().unwrap());
}
</script>

<!--
   Funkar för att datat är Send - går att 
   flytta ut ur tråden
-->

# Message Passing

<script language="rust">
use std::sync::mpsc::{Sender, Receiver, channel};

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let _thread = std::thread::spawn(move || {
        tx.send(", World!".to_owned()).unwrap();
    });
    println!("Hello {}", rx.recv().unwrap());
}
</script>

<!-- 
 Inget delat state, ingen synkronisation
-->

# Message Passing Tagning 2

<script language="rust">
use std::sync::mpsc::{Sender, Receiver, channel};

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let _thread = std::thread::spawn(|| {
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

# Send och Sync i thread::spawn

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, 
```

* En typ är Send om den går att ge till en annan tråd
* En typ är Sync om referenser går att ge till en annan tråd

<!-- 
 Sync betyder är typens referens är Send
-->

# Send och Sync för Standardtyper

* Primitiva typer är Send och Sync
* "Ärvs" -> Nästan alla typer är Send och Sync

<!-- 
 Typer som enbart innehåller typer som är sync och send
 blir själva sync och send

 Är marker-interface. 
   Skrivit en wrapper-typ som gör saker trådsäkra? Implementera Sync.
   Har du en typ 
-->

# Sync och Send Exempel

<script language="rust">
extern crate crossbeam;

fn main() {
    let s: String = "hej".to_owned();
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          println!("barntråd säger {}", s);
        });
    });
    println!("huvudtråd säger {}", s);
}
</script>

<!-- 
  Closure är by ref per default
  läsning -> läsref
  eftersom det gick så är den sync
  Gör closuren move och visa vad som händer
  - värdet flyttades in 
  eftersom det gick så är den send

  men flyttar aldrig ut igen, så sista println funkar ej
-->

# Sync funkar även med Mut

<script language="rust">
extern crate crossbeam;

fn main() {
    let mut s: String = "hej".to_owned();
    crossbeam::scope(|scope| { 
        scope.spawn(|| { 
          println!("barntråd säger {}", s);
          s.push_str(" på dig");
        });
    });
    println!("huvudtråd säger {}", s);
}
</script>


<!-- 
  Men varför använde jag crossbeam? 
-->

# Sync med långlivad tråd

<script language="rust">
use std::thread::spawn;

fn main() {
    let s: String = "hej".to_owned();
    spawn(|| { 
        println!("barntråd säger {}", s);
    });
    println!("huvudtråd säger {}", s);
}
</script>

<!--
 Det här är fortfarande vanliga ägarskapsregler.
 Lån får inte vara längre än variabeln själv.
 Men trådar har oändlig livslängd per default.
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

# Mutera i flera trådar

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
  Ta bort mittdelen så det blir en scope
  -> två mutable borrows -> fel.
  SAMMA BORROW-REGLER SOM VANLIGT
  Hur fixa? Förslag?
-->

# Samtidig Användning Kräver Synkronisering

* En i taget - Mutex
* Flera läsare i taget, en skrivare i taget - RwLock
* Högprestanda, tex eventually consistent multiple writers - Atomics

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

# Atomics

* Som Java

# Hur funkar Mutex o dyl?

* Tillåter multipla skriv-lån - varför klagar inte kompilatorn?
* Utger sig för att vara immutable
* Interior mutability

# Sammanfattning

* Samma regler som för vanlig utlåning + Sync & Send
* Nästan alla typer är Sync & Send
  * -> Vanliga utlåningscenarion funkar med trådar
* MEN andra scenarion, tex flera parallela skrivare så
   * Måste man synka med mutex el dyl
* Vanliga trådar har oändlig livstid -> lämnar aldrig tillbaka lån
* Crossbeam::scoped garanterade att terminera

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