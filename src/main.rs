#![feature(fnbox)]
#![feature(closure_to_fn_coercion)]
extern crate hyper;
extern crate hyper_router;
extern crate rustc_serialize;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper_router::{Route, RouterBuilder};
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Answer {
    msg: String,
}

fn get_yo(_: Request, res: Response) {
    let ans = Answer { msg: "mtg_bootstrap!".to_string() };
    let payload = json::encode(&ans).unwrap();
    res.send(payload.as_bytes()).unwrap();
}

fn put_yo(_: Request, mut res: Response) {
    *res.status_mut() = StatusCode::ImATeapot;
    res.send(b"no").unwrap();
}

struct AutoServer {
    listening: hyper::server::Listening,
}

impl AutoServer {
    pub fn new(port: &str) -> AutoServer {
        let router = RouterBuilder::new()
            .add(Route::get("/yo").using(get_yo))
            .add(Route::put("/yo").using(put_yo))
            .build();
        let root_handler = move |req: Request, mut res: Response| match router.find_handler(&req) {
            Ok(handler) => handler(req, res),
            Err(sc) => *res.status_mut() = sc,
        };
        let addr = format!("127.0.0.1:{}", port);
        let server = Server::http(addr).unwrap().handle(root_handler).unwrap();
        AutoServer { listening: server }
    }
}

impl Drop for AutoServer {
    fn drop(&mut self) {
        self.listening.close().unwrap();
    }
}

fn main() {
    let _server = AutoServer::new("9999");
    println!("listening on port 9999");
    std::thread::park();
    panic!("spurious wakeup");
}

#[cfg(test)]
mod test {
    extern crate hyper;
    extern crate pcons;
    use self::pcons::{pcons, pconsl, pconsl2};
    use super::AutoServer;
    use hyper::client::Client;
    use hyper::status::StatusCode;
    use std::io::Read;
    use std::boxed::FnBox;

    #[test]
    fn http_get() {
        let _server = AutoServer::new("9999");
        let client = Client::new();
        let mut res = client.get("http://127.0.0.1:9999/yo").send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut payload = String::new();
        res.read_to_string(&mut payload).unwrap();
        assert_eq!(payload, "{\"msg\":\"mtg_bootstrap!\"}");
    }

    #[test]
    fn http_put() {
        let _server = AutoServer::new("9994");
        let client = Client::new();
        let res = client.put("http://127.0.0.1:9999/yo").send().unwrap();
        assert_eq!(res.status, StatusCode::ImATeapot);
    }

    #[test]
    fn http_get_two() {
        let _server = AutoServer::new("9998");
        let client = Client::new();
        let (res1, res2) = pcons(|| client.get("http://127.0.0.1:9998/yo").send().unwrap(),
                                 || client.get("http://127.0.0.1:9998/yo").send().unwrap());
        assert_eq!(res1.status, hyper::Ok);
        assert_eq!(res2.status, hyper::Ok);
    }

    #[test]
    fn http_get_multiple() {
        let _server = AutoServer::new("9997");
        let client = Client::new();
        let (res1, (res2, res3)) = pcons(|| client.get("http://127.0.0.1:9997/yo").send().unwrap(),
                                         || {
            pcons(|| client.get("http://127.0.0.1:9997/yo").send().unwrap(),
                  || client.get("http://127.0.0.1:9997/yo").send().unwrap())
        });
        assert_eq!(res1.status, hyper::Ok);
        assert_eq!(res2.status, hyper::Ok);
        assert_eq!(res3.status, hyper::Ok);
    }

    #[test]
    fn http_get_pconsl_single() {
        let _server = AutoServer::new("9996");
        let client = Client::new();
        let v1 = vec![|| client.get("http://127.0.0.1:9996/yo").send().unwrap()];
        let resl = pconsl(v1);
        assert_eq!(resl.get(0).unwrap().status, hyper::Ok);
    }

    #[test]
    fn http_get_pconsl() {
        let _server = AutoServer::new("9995");
        let client = Client::new();
        let mut v1: Vec<Box<FnBox() -> hyper::client::Response + Send>> = Vec::new();
        v1.push(Box::new(|| client.get("http://127.0.0.1:9995/yo").send().unwrap()));
        v1.push(Box::new(|| client.get("http://127.0.0.1:9995/yo").send().unwrap()));
        let resl = pconsl2(v1);
        assert_eq!(resl.get(0).unwrap().status, hyper::Ok);
        assert_eq!(resl.get(1).unwrap().status, hyper::Ok);
    }
}
