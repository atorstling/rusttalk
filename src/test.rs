extern crate hyper;
extern crate pcons;
use self::pcons::{pcons, pconsl, pconsl2};
use self::hyper::client::Client;
use iron::Listening;
use std::io::Read;
use std::boxed::FnBox;

struct AutoServer {
    listening: Listening,
}

impl AutoServer {
    pub fn new(port: &str) -> AutoServer {
        let server = super::server(port);
        AutoServer { listening: server }
    }
}

impl Drop for AutoServer {
    fn drop(&mut self) {
        // Workaround for https://github.com/hyperium/hyper/issues/338
        self.listening.close().unwrap();
    }
}

#[test]
fn http_get() {
    let _server = AutoServer::new("9989");
    let client = Client::new();
    let mut res = client
        .get("http://127.0.0.1:9989/hello/mtg_bootstrap")
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut payload = String::new();
    res.read_to_string(&mut payload).unwrap();
    assert_eq!(payload, "{\"msg\":\"hello mtg_bootstrap!\"}");
}

#[test]
fn http_get_two() {
    let _server = AutoServer::new("9998");
    let client = Client::new();
    let (res1, res2) = pcons(
        || client.get("http://127.0.0.1:9998/hello/bro").send().unwrap(),
        || client.get("http://127.0.0.1:9998/hello/sis").send().unwrap(),
    );
    assert_eq!(res1.status, hyper::Ok);
    assert_eq!(res2.status, hyper::Ok);
}

#[test]
fn http_get_multiple() {
    let _server = AutoServer::new("9997");
    let client = Client::new();
    let (res1, (res2, res3)) = pcons(
        || {
            client
                .get("http://127.0.0.1:9997/hello/bigmama")
                .send()
                .unwrap()
        },
        || {
            pcons(
                || client.get("http://127.0.0.1:9997/hello/wassa").send().unwrap(),
                || {
                    client
                        .get("http://127.0.0.1:9997/hello/lilboy")
                        .send()
                        .unwrap()
                },
            )
        },
    );
    assert_eq!(res1.status, hyper::Ok);
    assert_eq!(res2.status, hyper::Ok);
    assert_eq!(res3.status, hyper::Ok);
}

#[test]
fn http_get_pconsl_single() {
    let _server = AutoServer::new("9996");
    let client = Client::new();
    let v1 = vec![
        || client.get("http://127.0.0.1:9996/hello/myman").send().unwrap(),
    ];
    let resl = pconsl(v1);
    assert_eq!(resl.get(0).unwrap().status, hyper::Ok);
}

#[test]
fn http_get_pconsl() {
    let _server = AutoServer::new("9995");
    let client = Client::new();
    let mut v1: Vec<Box<FnBox() -> hyper::client::Response + Send>> = Vec::new();
    v1.push(Box::new(
        || client.get("http://127.0.0.1:9995/hello/lo").send().unwrap(),
    ));
    v1.push(Box::new(
        || client.get("http://127.0.0.1:9995/hello/yo").send().unwrap(),
    ));
    let resl = pconsl2(v1);
    assert_eq!(resl.get(0).unwrap().status, hyper::Ok);
    assert_eq!(resl.get(1).unwrap().status, hyper::Ok);
}
