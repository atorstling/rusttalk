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

#[test]
fn http_get() {
    let mut server =
        Server::http("127.0.0.1:9999").unwrap().handle(|_: Request, _: Response| {}).unwrap();
    let client = Client::new();
    let res = client.get("http://127.0.0.1:9999").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    server.close().unwrap();
}
