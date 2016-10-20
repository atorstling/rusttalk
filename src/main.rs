extern crate rustc_serialize;

use rustc_serialize::json;
//use std::thread;

fn fork<F1,R1,F2,R2>(f1: F1, f2: F2) -> (R1, R2) where F1: FnOnce() -> R1, F2: FnOnce() -> R2 {
  /*
  let t1 = thread::spawn(move || {
    f1();
  });
  let t2 = thread::spawn(move || {
    f2();
  });
  (t1.join().unwrap(), t2.join().unwrap());
  */
  (f1(), f2())
}

#[test]
fn test_what() {
  let (a, b) = fork(|| 1, || 2);
  assert!(a == 1);
  assert!(b == 2);
}

fn main() {
    println!("{:?}", json::encode(&42));
    println!("{:?}", json::encode(&vec!["to", "be", "or", "not", "to", "be"]));
    println!("{:?}", json::encode(&Some(true)));
}
