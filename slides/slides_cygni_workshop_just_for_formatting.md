% Rust - Trådning
![rust](img/rust.svg)

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

# Trådar har returnvärden

<script language="rust">
fn main() {
    let thread = std::thread::spawn(|| {
      "Hej, förälder!"
    });
    println!("{}", thread.join().unwrap());
}
</script>

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

# Send och Sync

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, 
```

* En typ är Send om den går att ge till en annan tråd
* En typ är Sync om referenser går att ge till en annan tråd

# Send och Sync för Standardtyper

* Primitiva typer är Send och Sync
* "Ärvs" eller smittar av sig
* Är opt out:

<script language="rust">
#![feature(optin_builtin_traits)]
struct MinTyp {}
impl !Sync for MinTyp {}
fn main() {
    let _ = MinTyp{};
}
</script>

<!-- 
 Typer som enbart innehåller typer som är sync och send
 blir själva sync och send
-->

# Sync Exempel

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
  Gör closuren move och visa vad som händer
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

# Thread::spawn signatur

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

* Trådning har samma borrow-regler som vanligt

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

* Kommer det övning på, inget speciellt

# Sammanfattning

* Vanliga trådar har oändlig livstid -> lämnar aldrig tillbaka lån
* Crossbeam::scoped garanterade att terminera
* För att dela data mellan trådar finns Sync & Send
   * De flesta typer implementerar båda
* När man delar Sync data gäller vanliga borrow-regler
   * Ett skrivlån eller
   * Flera läslån
* Men om man vill tillåta andra scenarion, tex flera skrivare så
   * Måste man synka med mutex el dyl

# Frågor
