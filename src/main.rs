extern crate crossbeam;
extern crate hyper;
use hyper::server::{Server, Request, Response};
use hyper::client::Client;

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

//fn pconsl<F, R>(fs: &[F]) -> Vec<R>
//  where F: FnOnce() -> R,
//{
  //if let Some((head, tail)) = fs.split_first() {
 //   let res = pcons(
 //    || head(),
 //    || pconsl(tail)
 ////////////////   );
//    let mut arr: Vec<R> = Vec::new();
//    return arr;
//  } else {
 //   panic!("empty list");
 // }
//}

// fn plist<F, R>(fs: &[&F]) -> &

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

fn pconsl<F: ?Sized,R>(fs: Vec<Box<F>>) -> Vec<R> 
  where F: FnOnce() -> R 
{
  let res: Vec<R> = Vec::new();
  res
}

#[test]
fn pcons_list() {
    let mut arr: Vec<Box<FnOnce() -> String>> = Vec::new();
    arr.push(Box::new(|| String::from("a")));
    arr.push(Box::new(|| String::from("b")));
    let res = pconsl(arr);
    assert_eq!(res.get(0).unwrap(),&"hej");
}

struct TestServer {
  listening: hyper::server::Listening
}

impl TestServer {
  pub fn new() -> TestServer {
    TestServer {
      listening: Server::http("127.0.0.1:9999").unwrap().handle( |_: Request, _: Response| {}).unwrap() }
  }
}

impl Drop for TestServer {
  fn drop(&mut self) {
    self.listening.close().unwrap();
  }
}

#[test]
fn http_get() {
    let _server = TestServer::new();
    let client = Client::new();
    let res = client.get("http://127.0.0.1:9999").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
}

/*
#[test]
fn http_get_multiple_get() {
    let _server = TestServer::new();
    let client = Client::new();
    let (res1, (res2, res3)) = pcons(
      || client.get("http://127.0.0.1:9999").send().unwrap(),
      pcons(
      || client.get("http://127.0.0.1:9999").send().unwrap(),
      || client.get("http://127.0.0.1:9999").send().unwrap()
      )
    );
    assert_eq!(res1.status, hyper::Ok);
    assert_eq!(res2.status, hyper::Ok);
    assert_eq!(res3.status, hyper::Ok);
}
*/

fn main() {
  TestServer::new();
}
