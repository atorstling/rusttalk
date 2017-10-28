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
 move - tx måste flyttas till tråden
-->
