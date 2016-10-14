extern crate rustc_serialize;
extern crate core;

use rustc_serialize::json;

#[test]
fn test_some() {

}

fn main() {
    println!("{:?}", json::encode(&42));
    println!("{:?}", json::encode(&vec!["to", "be", "or", "not", "to", "be"]));
    println!("{:?}", json::encode(&Some(true)));
}
