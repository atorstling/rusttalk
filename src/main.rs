#![feature(closure_to_fn_coercion)]
extern crate crossbeam;
extern crate hyper;
use hyper::server::{Server, Request, Response};
use hyper::client::Client;

// Crossbeam::scope requires Send:
// fn spawn<F, T>(&self, f: F) -> ScopedJoinHandle<T> 
// where F: FnOnce() -> T + Send + 'a, T: Send + 'a
// Send means Sync + Copy
fn pcons<F1, R1, F2, R2>(f1: F1, f2: F2) -> (R1, R2)
    where F1: FnOnce() -> R1 + Send,
          F2: FnOnce() -> R2 + Send,
          R1: Send,
          R2: Send 
{
    crossbeam::scope(|scope| {
        (scope.spawn(f1)
            .join(),
         scope.spawn(f2).join())
    })
}


#[test]
fn pcons_returns_correct_values() {
    let (a, b) = pcons(|| 1, || 2);
    assert_eq!(a,1);
    assert_eq!(b,2);
}

#[test]
fn pcons_can_be_chained() {
    let (a, (b, c)) = pcons(|| 1, || pcons(|| 2, || 3));
    assert_eq!(a,1);
    assert_eq!(b,2);
    assert_eq!(c,3);
}

// If we use &[&F], we can pass the closure, but
// cannot call it since the size is unknown, see
// http://stackoverflow.com/questions/30411594/moving-a-boxed-function

// &[Box<F>] is also unusable for the same reason, Box<FnOnce> is
// ususable.

// We cannot use &[F], since F is unsized
// note: slice and array elements must have `Sized` type

//
// where F: FnBox() -> R 

// Goal: have pconsl use pcons
/*
fn pconsl<R>(fs: &[&Fn() -> R]) -> Vec<R> 
{
  if let Some((head, _)) = fs.split_first() {
    let head_res: R = head();
    
    let mut res: Vec<R> = Vec::new();
    res.push(head_res);
    res
  } else {
    panic!("empty list");
  }
}
*/
fn pconsl<F, R>(mut fs: Vec<F>) -> Vec<R>
  where F: FnOnce() -> R + Send + Sync,
        R: Send ,
{
    if fs.len() == 0 {
        let vec: Vec<R> = vec![];
        return vec;
    }
    if fs.len() == 1 {
        return vec![fs.remove(0)()];
    }
  let tail: Vec<F> = fs.split_off(1);
  let head: F = fs.remove(0);
    let mut res: (R, Vec<R>) = pcons(
     || head(),
     || pconsl(tail)
    );
    let mut arr: Vec<R> = Vec::new();
    arr.push(res.0);
    arr.append(&mut res.1);
    arr
}

// fn plist<F, R>(fs: &[&F]) -> &
#[test]
fn pconsl_works() {
    let a = || 1;
    let b = || 2;
    let arr: Vec<fn() -> u32> = vec![a, b];
    //arr.push(&a);
    //arr.push(&b);
    let res = pconsl(arr);
    assert_eq!(res.get(0).unwrap(), &1);
    //assert_eq!(res.get(0).unwrap(),&String::from("a"));
}

struct TestServer {
  listening: hyper::server::Listening
}

impl TestServer {
  pub fn new(port: &str) -> TestServer {
    TestServer {
      listening: Server::http(["127.0.0.1:", port].join("")).unwrap().handle( |_: Request, _: Response| {}).unwrap() }
  }
}

impl Drop for TestServer {
  fn drop(&mut self) {
    self.listening.close().unwrap();
  }
}

#[test]
fn http_get() {
    let _server = TestServer::new("9999");
    let client = Client::new();
    let res = client.get("http://127.0.0.1:9999").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
}

#[test]
fn http_get_two() {
    let _server = TestServer::new("9998");
    let client = Client::new();
    let (res1, res2) = pcons(|| client.get("http://127.0.0.1:9998").send().unwrap(),
        || client.get("http://127.0.0.1:9998").send().unwrap());
    assert_eq!(res1.status, hyper::Ok);
    assert_eq!(res2.status, hyper::Ok);
}

#[test]
fn http_get_multiple_get() {
    let _server = TestServer::new("9997");
    let client = Client::new();
    let (res1, (res2, res3)) = pcons(
      || client.get("http://127.0.0.1:9997").send().unwrap(),
      || pcons(
        || client.get("http://127.0.0.1:9997").send().unwrap(),
        || client.get("http://127.0.0.1:9997").send().unwrap()
      )
    );
    assert_eq!(res1.status, hyper::Ok);
    assert_eq!(res2.status, hyper::Ok);
    assert_eq!(res3.status, hyper::Ok);
}

fn main() {
  TestServer::new("9999");
}
