#![feature(fnbox)]
#![feature(closure_to_fn_coercion)]
extern crate iron;
extern crate router;
extern crate rustc_serialize;
use iron::prelude::*;
use iron::{status, Listening};
use router::Router;
use rustc_serialize::json;

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

fn server(port: &str) -> Listening {
    let mut router = Router::new();
    router.get("/yo/:phrase", get_yo, "get_yo");
    router.put("/yo",
               |_: &mut Request| Ok(Response::with((status::ImATeapot, "no"))),
               "put_yo");
    Iron::new(router).http(format!("localhost:{}", port)).unwrap()
}

fn main() {
    let _server = server("9999");
    println!("listening on port 9999");
    std::thread::park();
    panic!("spurious wakeup");
}

#[cfg(test)]
mod test {
    extern crate hyper;
    extern crate pcons;
    use self::pcons::{pcons, pconsl, pconsl2};
    use self::hyper::client::Client;
    use self::hyper::status::StatusCode;
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
        let mut res = client.get("http://127.0.0.1:9989/yo/mtg_bootstrap").send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut payload = String::new();
        res.read_to_string(&mut payload).unwrap();
        assert_eq!(payload, "{\"msg\":\"yo mtg_bootstrap!\"}");
    }

    #[test]
    fn http_put() {
        let _server = AutoServer::new("9994");
        let client = Client::new();
        let res = client.put("http://127.0.0.1:9994/yo").send().unwrap();
        assert_eq!(res.status, StatusCode::ImATeapot);
    }

    #[test]
    fn http_get_two() {
        let _server = AutoServer::new("9998");
        let client = Client::new();
        let (res1, res2) = pcons(|| client.get("http://127.0.0.1:9998/yo/bro").send().unwrap(),
                                 || client.get("http://127.0.0.1:9998/yo/sis").send().unwrap());
        assert_eq!(res1.status, hyper::Ok);
        assert_eq!(res2.status, hyper::Ok);
    }

    #[test]
    fn http_get_multiple() {
        let _server = AutoServer::new("9997");
        let client = Client::new();
        let (res1, (res2, res3)) =
            pcons(|| client.get("http://127.0.0.1:9997/yo/bigmama").send().unwrap(),
                  || {
                      pcons(|| client.get("http://127.0.0.1:9997/yo/wassa").send().unwrap(),
                            || client.get("http://127.0.0.1:9997/yo/lilboy").send().unwrap())
                  });
        assert_eq!(res1.status, hyper::Ok);
        assert_eq!(res2.status, hyper::Ok);
        assert_eq!(res3.status, hyper::Ok);
    }

    #[test]
    fn http_get_pconsl_single() {
        let _server = AutoServer::new("9996");
        let client = Client::new();
        let v1 = vec![|| client.get("http://127.0.0.1:9996/yo/myman").send().unwrap()];
        let resl = pconsl(v1);
        assert_eq!(resl.get(0).unwrap().status, hyper::Ok);
    }

    #[test]
    fn http_get_pconsl() {
        let _server = AutoServer::new("9995");
        let client = Client::new();
        let mut v1: Vec<Box<FnBox() -> hyper::client::Response + Send>> = Vec::new();
        v1.push(Box::new(|| client.get("http://127.0.0.1:9995/yo/lo").send().unwrap()));
        v1.push(Box::new(|| client.get("http://127.0.0.1:9995/yo/yo").send().unwrap()));
        let resl = pconsl2(v1);
        assert_eq!(resl.get(0).unwrap().status, hyper::Ok);
        assert_eq!(resl.get(1).unwrap().status, hyper::Ok);
    }
}
